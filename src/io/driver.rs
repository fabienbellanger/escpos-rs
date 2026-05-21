//! Drivers used to send data to the printer (Network or USB)

use crate::errors::{PrinterError, Result};
#[cfg(feature = "hidapi")]
use hidapi::{HidApi, HidDevice};
#[cfg(feature = "native_usb")]
use nusb::{MaybeFuture, transfer::EndpointType};
#[cfg(feature = "usb")]
use rusb::{Context, DeviceHandle, Direction, TransferType, UsbContext, UsbOption};
#[cfg(feature = "serial_port")]
use serialport::SerialPort;
use std::sync::{Arc, Mutex};
#[cfg(all(feature = "usbprint", target_os = "windows"))]
use std::{ffi::OsString, mem, os::windows::ffi::OsStringExt, ptr};
use std::{
    fs::File,
    fs::OpenOptions,
    io::{self, Read, Write},
    net::{IpAddr, SocketAddr, TcpStream},
    path::Path,
    time::Duration,
};

/// Default timeout in seconds for read/write operations
const DEFAULT_TIMEOUT_SECONDS: u64 = 5;

/// Printer driver trait
///
/// A custom driver can be implemented by implementing this trait.
pub trait Driver {
    /// Driver name
    fn name(&self) -> String;

    /// Write data
    fn write(&self, data: &[u8]) -> Result<()>;

    /// Read data
    fn read(&self, buf: &mut [u8]) -> Result<usize>;

    /// Flush data
    fn flush(&self) -> Result<()>;
}

// ================ Console driver ================

/// Console driver for debug
#[derive(Default, Clone)]
pub struct ConsoleDriver {
    show_output: bool,
}

impl ConsoleDriver {
    /// Open the Console driver
    ///
    /// # Example
    ///
    /// ```rust
    /// use escpos::printer::Printer;
    /// use escpos::utils::*;
    /// use escpos::driver::*;
    ///
    /// let driver = ConsoleDriver::open(true);
    /// let mut printer = Printer::new(driver, Protocol::default(), None);
    /// ```
    pub fn open(show_output: bool) -> Self {
        Self { show_output }
    }
}

impl Driver for ConsoleDriver {
    fn name(&self) -> String {
        "console".to_owned()
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        if self.show_output {
            io::stdout().write_all(data)?
        }
        Ok(())
    }

    fn read(&self, _buf: &mut [u8]) -> Result<usize> {
        Ok(0)
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }
}

// ================ Network driver ================

/// Driver for network printer
#[derive(Clone)]
pub struct NetworkDriver {
    host: String,
    port: u16,
    stream: Arc<Mutex<TcpStream>>,
    timeout: Duration,
}

impl NetworkDriver {
    /// Open the network driver
    ///
    /// # Example
    ///
    /// ```no_run
    /// use escpos::printer::Printer;
    /// use escpos::utils::*;
    /// use escpos::driver::*;
    /// use std::time::Duration;
    ///
    /// let driver = NetworkDriver::open("192.168.1.248", 9100, Some(Duration::from_secs(1))).unwrap();
    /// let mut printer = Printer::new(driver, Protocol::default(), None);
    /// ```
    pub fn open(host: &str, port: u16, timeout: Option<Duration>) -> Result<Self> {
        let stream = match timeout {
            Some(timeout) => {
                let addr = SocketAddr::new(
                    host.parse::<IpAddr>().map_err(|e| PrinterError::Io(e.to_string()))?,
                    port,
                );
                TcpStream::connect_timeout(&addr, timeout)?
            }
            None => TcpStream::connect((host, port))?,
        };
        let timeout = timeout.unwrap_or(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS));

        Ok(Self {
            host: host.to_string(),
            port,
            stream: Arc::new(Mutex::new(stream)),
            timeout,
        })
    }
}

impl Driver for NetworkDriver {
    fn name(&self) -> String {
        format!("network ({}:{})", self.host, self.port)
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        let mut stream = self.stream.lock()?;
        stream.set_write_timeout(Some(self.timeout))?;

        Ok(stream.write_all(data)?)
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        let mut stream = self.stream.lock()?;
        stream.set_read_timeout(Some(self.timeout))?;

        Ok(stream.read(buf)?)
    }

    fn flush(&self) -> Result<()> {
        Ok(self.stream.lock()?.flush()?)
    }
}

// ================ File driver ================

/// Driver for USB printer using file
#[derive(Clone)]
pub struct FileDriver {
    path: String,
    file: Arc<Mutex<File>>,
}

impl FileDriver {
    /// Open the file driver
    ///
    /// # Example
    ///
    /// ```no_run
    /// use escpos::printer::Printer;
    /// use escpos::utils::*;
    /// use escpos::driver::*;
    /// use std::path::Path;
    ///
    /// let path = Path::new("./foo/bar.txt");
    /// let driver = FileDriver::open(&path).unwrap();
    /// let mut printer = Printer::new(driver, Protocol::default(), None);
    /// ```
    pub fn open(path: &Path) -> Result<Self> {
        let file = File::options().read(true).append(true).open(path)?;
        Ok(Self {
            path: path.to_string_lossy().to_string(),
            file: Arc::new(Mutex::new(file)),
        })
    }

    /// Open the file driver using custom options
    ///
    /// # Example
    ///
    /// ```no_run
    /// use escpos::printer::Printer;
    /// use escpos::utils::*;
    /// use escpos::driver::*;
    /// use std::fs::OpenOptions;
    /// use std::path::Path;
    ///
    /// let path = Path::new("./foo/bar.txt");
    /// let mut options = OpenOptions::new();
    /// options.write(true).create(true).truncate(true);
    /// let driver = FileDriver::open_with_options(&path, &options).unwrap();
    /// let mut printer = Printer::new(driver, Protocol::default(), None);
    /// ```
    pub fn open_with_options(path: &Path, options: &OpenOptions) -> Result<Self> {
        let file = options.open(path)?;
        Ok(Self {
            path: path.to_string_lossy().to_string(),
            file: Arc::new(Mutex::new(file)),
        })
    }
}

impl Driver for FileDriver {
    fn name(&self) -> String {
        format!("file ({})", self.path)
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        self.file.lock()?.write_all(data)?;
        Ok(())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        Ok(self.file.lock()?.read(buf)?)
    }

    fn flush(&self) -> Result<()> {
        Ok(self.file.lock()?.flush()?)
    }
}

// ================ USB drivers ================

/// Driver for USB printer
#[cfg(feature = "usb")]
#[derive(Clone)]
pub struct UsbDriver {
    vendor_id: u16,
    product_id: u16,
    output_endpoint: u8,
    input_endpoint: u8,
    device: Arc<Mutex<DeviceHandle<Context>>>,
    timeout: Duration,
}

#[cfg(feature = "usb")]
impl UsbDriver {
    /// Open a new USB connection
    ///
    /// # Example
    ///
    /// ```no_run
    /// use escpos::printer::Printer;
    /// use escpos::utils::*;
    /// use escpos::driver::*;
    ///
    /// let driver = UsbDriver::open(0x0525, 0xa700, None, None).unwrap();
    /// let mut printer = Printer::new(driver, Protocol::default(), None);
    /// ```
    pub fn open(
        vendor_id: u16,
        product_id: u16,
        timeout: Option<Duration>,
        options: Option<&[UsbOption]>,
    ) -> Result<Self> {
        let context = if let Some(options) = options {
            Context::with_options(options).map_err(|e| PrinterError::Io(e.to_string()))?
        } else {
            Context::new().map_err(|e| PrinterError::Io(e.to_string()))?
        };
        let devices = context.devices().map_err(|e| PrinterError::Io(e.to_string()))?;

        for device in devices.iter() {
            let device_descriptor = device
                .device_descriptor()
                .map_err(|e| PrinterError::Io(e.to_string()))?;

            if device_descriptor.vendor_id() == vendor_id && device_descriptor.product_id() == product_id {
                let config_descriptor = device
                    .active_config_descriptor()
                    .map_err(|e| PrinterError::Io(e.to_string()))?;

                let (output_endpoint, input_endpoint, interface_number) = config_descriptor
                    .interfaces()
                    .flat_map(|interface| interface.descriptors())
                    .flat_map(|descriptor| {
                        let interface_number = descriptor.interface_number();

                        // Find input and output endpoints
                        let mut input_endpoint = None;
                        let mut output_endpoint = None;
                        for endpoint in descriptor.endpoint_descriptors() {
                            if endpoint.transfer_type() == TransferType::Bulk && endpoint.direction() == Direction::In {
                                input_endpoint = Some(endpoint.address());
                            } else if endpoint.transfer_type() == TransferType::Bulk
                                && endpoint.direction() == Direction::Out
                            {
                                output_endpoint = Some(endpoint.address());
                            }
                        }

                        match (output_endpoint, input_endpoint) {
                            (Some(output_endpoint), Some(input_endpoint)) => {
                                Some((output_endpoint, input_endpoint, interface_number))
                            }
                            _ => None,
                        }
                    })
                    .next()
                    .ok_or_else(|| {
                        PrinterError::Io("no suitable endpoints or interface number found for USB device".to_string())
                    })?;

                return match device.open() {
                    Ok(device_handle) => {
                        #[cfg(not(target_os = "windows"))]
                        match device_handle.kernel_driver_active(interface_number) {
                            Ok(active) => {
                                if active && let Err(e) = device_handle.detach_kernel_driver(interface_number) {
                                    return Err(PrinterError::Io(e.to_string()));
                                }
                            }
                            Err(e) => return Err(PrinterError::Io(e.to_string())),
                        }

                        // Claims device interface
                        device_handle
                            .claim_interface(interface_number)
                            .map_err(|e| PrinterError::Io(e.to_string()))?;

                        Ok(Self {
                            vendor_id,
                            product_id,
                            output_endpoint,
                            input_endpoint,
                            device: Arc::new(Mutex::new(device_handle)),
                            timeout: timeout.unwrap_or(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS)),
                        })
                    }
                    Err(e) => Err(PrinterError::Io(e.to_string())),
                };
            }
        }

        Err(PrinterError::Io("USB device not found".to_string()))
    }
}

#[cfg(feature = "usb")]
impl Driver for UsbDriver {
    fn name(&self) -> String {
        format!(
            "USB (VID: {}, PID: {}, output endpoint: {}, input endpoint: {})",
            self.vendor_id, self.product_id, self.output_endpoint, self.input_endpoint
        )
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        self.device
            .lock()?
            .write_bulk(self.output_endpoint, data, self.timeout)
            .map_err(|e| PrinterError::Io(e.to_string()))?;
        Ok(())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        self.device
            .lock()?
            .read_bulk(self.input_endpoint, buf, self.timeout)
            .map_err(|e| PrinterError::Io(e.to_string()))
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }
}

/// Driver for USB printer
#[cfg(feature = "native_usb")]
#[derive(Clone)]
pub struct NativeUsbDriver {
    vendor_id: u16,
    product_id: u16,
    output_endpoint: u8,
    input_endpoint: u8,
    device: Arc<Mutex<nusb::Interface>>,
}

#[cfg(feature = "native_usb")]
impl NativeUsbDriver {
    /// Open a new USB connection
    ///
    /// # Example
    ///
    /// ```no_run
    /// use escpos::printer::Printer;
    /// use escpos::utils::*;
    /// use escpos::driver::*;
    ///
    /// let driver = NativeUsbDriver::open(0x0525, 0xa700).unwrap();
    /// let mut printer = Printer::new(driver, Protocol::default(), None);
    /// ```
    pub fn open(vendor_id: u16, product_id: u16) -> Result<Self> {
        let device_info = nusb::list_devices()
            .wait()
            .map_err(|e| PrinterError::Io(e.to_string()))?
            .find(|dev| dev.vendor_id() == vendor_id && dev.product_id() == product_id)
            .ok_or(PrinterError::Io("USB device not found".to_string()))?;
        let device = device_info.open().wait().map_err(|e| PrinterError::Io(e.to_string()))?;

        // Get endpoints
        let configuration = device
            .active_configuration()
            .map_err(|e| PrinterError::Io(e.to_string()))?;

        let (output_endpoint, input_endpoint) = match configuration.interface_alt_settings().next() {
            Some(settings) => {
                let endpoints = settings.endpoints();
                let (mut output, mut input) = (None, None);

                for endpoint in endpoints {
                    if endpoint.transfer_type() == nusb::transfer::Bulk::TYPE
                        && endpoint.direction() == nusb::transfer::Direction::Out
                    {
                        output = Some(endpoint.address())
                    } else if endpoint.transfer_type() == nusb::transfer::Bulk::TYPE
                        && endpoint.direction() == nusb::transfer::Direction::In
                    {
                        input = Some(endpoint.address())
                    }
                }

                match (output, input) {
                    (Some(output), Some(input)) => Some((output, input)),
                    _ => None,
                }
            }
            None => None,
        }
        .ok_or(PrinterError::Io(
            "no suitable input or output endpoints found for USB device".to_string(),
        ))?;

        // Get interface number
        let interface_number = device_info
            .interfaces()
            .map(|interface| interface.interface_number())
            .next()
            .ok_or_else(|| PrinterError::Io("no suitable interface number found for USB device".to_string()))?;

        let interface = device
            .detach_and_claim_interface(interface_number)
            .wait()
            .map_err(|e| PrinterError::Io(e.to_string()))?;

        Ok(Self {
            vendor_id,
            product_id,
            output_endpoint,
            input_endpoint,
            device: Arc::new(Mutex::new(interface)),
        })
    }
}

#[cfg(feature = "native_usb")]
impl Driver for NativeUsbDriver {
    fn name(&self) -> String {
        format!(
            "USB (VID: {}, PID: {}, output endpoint: {}, input endpoint: {})",
            self.vendor_id, self.product_id, self.output_endpoint, self.input_endpoint
        )
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        let endpoint = self
            .device
            .lock()?
            .endpoint::<nusb::transfer::Bulk, nusb::transfer::Out>(self.output_endpoint)
            .map_err(|e| PrinterError::Io(e.to_string()))?;

        let max_size = endpoint.max_packet_size();

        let mut writer = endpoint
            .writer(max_size)
            .with_write_timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS));

        writer.write_all(data).map_err(|e| PrinterError::Io(e.to_string()))?;
        writer.flush().map_err(|e| PrinterError::Io(e.to_string()))?;
        Ok(())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        let endpoint = self
            .device
            .lock()?
            .endpoint::<nusb::transfer::Bulk, nusb::transfer::In>(self.input_endpoint)
            .map_err(|e| PrinterError::Io(e.to_string()))?;

        let max_size = endpoint.max_packet_size();

        let mut reader = endpoint
            .reader(max_size)
            .with_read_timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS));

        let mut pkt_reader = reader.until_short_packet();
        let size = pkt_reader
            .read_to_end(&mut buf.to_vec())
            .map_err(|e| PrinterError::Io(e.to_string()))?;
        pkt_reader.consume_end().map_err(|e| PrinterError::Io(e.to_string()))?;

        Ok(size)
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }
}

// ================ HidApi driver ================

/// Driver for USB printer
#[cfg(feature = "hidapi")]
#[derive(Clone)]
pub struct HidApiDriver {
    vendor_id: u16,
    product_id: u16,
    device: Arc<Mutex<HidDevice>>,
}

#[cfg(feature = "hidapi")]
impl HidApiDriver {
    /// Open a new USB connection
    ///
    /// # Example
    ///
    /// ```no_run
    /// use escpos::printer::Printer;
    /// use escpos::utils::*;
    /// use escpos::driver::*;
    ///
    /// let driver = HidApiDriver::open(0x0525, 0xa700).unwrap();
    /// let mut printer = Printer::new(driver, Protocol::default(), None);
    /// ```
    pub fn open(vendor_id: u16, product_id: u16) -> Result<Self> {
        let api = HidApi::new().map_err(|e| PrinterError::Io(e.to_string()))?;
        let device = api
            .open(vendor_id, product_id)
            .map_err(|e| PrinterError::Io(e.to_string()))?;

        Ok(Self {
            vendor_id,
            product_id,
            device: Arc::new(Mutex::new(device)),
        })
    }
}

#[cfg(feature = "hidapi")]
impl Driver for HidApiDriver {
    fn name(&self) -> String {
        format!("HidApi (VID: {}, PID: {})", self.vendor_id, self.product_id)
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        self.device
            .lock()?
            .write(data)
            .map_err(|e| PrinterError::Io(e.to_string()))?;
        Ok(())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        self.device
            .lock()?
            .read_timeout(buf, i32::try_from(DEFAULT_TIMEOUT_SECONDS * 1_000)?)
            .map_err(|e| PrinterError::Io(e.to_string()))
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }
}

// ================ Serial port driver ================

/// Driver for Serial printer
#[cfg(feature = "serial_port")]
#[derive(Clone)]
pub struct SerialPortDriver {
    path: String,
    port: Arc<Mutex<Box<dyn SerialPort>>>,
}

#[cfg(feature = "serial_port")]
impl SerialPortDriver {
    /// Open a new Serial port connection
    ///
    /// # Example
    ///
    /// ```no_run
    /// use escpos::printer::Printer;
    /// use escpos::utils::*;
    /// use escpos::driver::*;
    /// use std::time::Duration;
    ///
    /// let driver = SerialPortDriver::open("/dev/ttyUSB0", 115_200, Some(Duration::from_secs(5))).unwrap();
    /// let mut printer = Printer::new(driver, Protocol::default(), None);
    /// ```
    pub fn open(path: &str, baud_rate: u32, timeout: Option<Duration>) -> Result<Self> {
        let mut port = serialport::new(path, baud_rate);
        if let Some(timeout) = timeout {
            port = port.timeout(timeout);
        }
        let port = port.open().map_err(|e| PrinterError::Io(e.to_string()))?;

        Ok(Self {
            path: path.to_string(),
            port: Arc::new(Mutex::new(port)),
        })
    }
}

#[cfg(feature = "serial_port")]
impl Driver for SerialPortDriver {
    fn name(&self) -> String {
        format!("Serial port ({})", self.path)
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        self.port.lock()?.write_all(data)?;

        Ok(())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        let mut port = self.port.lock()?;
        port.set_timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS))
            .map_err(|e| PrinterError::Io(e.to_string()))?;
        Ok(port.read(buf)?)
    }

    fn flush(&self) -> Result<()> {
        Ok(self.port.lock()?.flush()?)
    }
}

// ================ Windows USB print driver ================

/// Information about a USB printer discovered via the Windows `usbprint.sys` driver
#[cfg(all(feature = "usbprint", target_os = "windows"))]
#[derive(Debug, Clone)]
pub struct WindowsUsbPrintInfo {
    /// Device interface path (can be passed to `WindowsUsbPrintDriver::open`)
    pub device_path: String,
    /// USB Vendor ID parsed from the device path (when available)
    pub vendor_id: Option<u16>,
    /// USB Product ID parsed from the device path (when available)
    pub product_id: Option<u16>,
}

/// Driver for a POS printer connected via the Windows `usbprint.sys` kernel driver.
///
/// This driver talks to the printer through the Windows Win32 API (`CreateFile` / `ReadFile` /
/// `WriteFile`) by opening the device interface exposed by the standard `usbprint.sys` class
/// driver. It is the recommended way to drive a USB ESC/POS printer on Windows without having
/// to replace the driver with WinUSB/libusb (e.g. via Zadig).
///
/// **Status:** ESC/POS real-time status (`DLE EOT`) responses often do not appear as data from
/// [`ReadFile`](https://learn.microsoft.com/windows/win32/api/fileapi/nf-fileapi-readfile) on
/// this stack (you may see success with zero bytes read). For a status DWORD from the class
/// driver, use [`WindowsUsbPrintDriver::lpt_status`].
///
/// The device interface class GUID used is `GUID_DEVINTERFACE_USBPRINT`
/// (`{28D78FAD-5A12-11D1-AE5B-0000F803A8C2}`).
#[cfg(all(feature = "usbprint", target_os = "windows"))]
#[derive(Clone)]
pub struct WindowsUsbPrintDriver {
    device_path: String,
    handle: Arc<Mutex<WinHandle>>,
}

#[cfg(all(feature = "usbprint", target_os = "windows"))]
struct WinHandle(windows_sys::Win32::Foundation::HANDLE);

// `HANDLE` is a raw pointer but Windows kernel handles are safe to move between threads and are
// synchronized internally for the file I/O operations we perform.
#[cfg(all(feature = "usbprint", target_os = "windows"))]
unsafe impl Send for WinHandle {}
#[cfg(all(feature = "usbprint", target_os = "windows"))]
unsafe impl Sync for WinHandle {}

#[cfg(all(feature = "usbprint", target_os = "windows"))]
impl Drop for WinHandle {
    fn drop(&mut self) {
        use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
        unsafe {
            if self.0 as isize != 0 && self.0 as isize != INVALID_HANDLE_VALUE as isize {
                CloseHandle(self.0);
            }
        }
    }
}

#[cfg(all(feature = "usbprint", target_os = "windows"))]
impl WindowsUsbPrintDriver {
    // {28D78FAD-5A12-11D1-AE5B-0000F803A8C2}
    const GUID_DEVINTERFACE_USBPRINT: windows_sys::core::GUID = windows_sys::core::GUID {
        data1: 0x28d7_8fad,
        data2: 0x5a12,
        data3: 0x11d1,
        data4: [0xae, 0x5b, 0x00, 0x00, 0xf8, 0x03, 0xa8, 0xc2],
    };

    /// Enumerate all USB printers currently exposed by the Windows `usbprint.sys` driver.
    ///
    /// Returns a list of [`WindowsUsbPrintInfo`] that can be used to open a specific printer by
    /// passing its `device_path` to [`WindowsUsbPrintDriver::open`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use escpos::driver::WindowsUsbPrintDriver;
    ///
    /// for info in WindowsUsbPrintDriver::list().unwrap() {
    ///     println!("{} (VID={:?}, PID={:?})", info.device_path, info.vendor_id, info.product_id);
    /// }
    /// ```
    pub fn list() -> Result<Vec<WindowsUsbPrintInfo>> {
        use windows_sys::Win32::Devices::DeviceAndDriverInstallation::{
            DIGCF_DEVICEINTERFACE, DIGCF_PRESENT, SP_DEVICE_INTERFACE_DATA, SetupDiDestroyDeviceInfoList,
            SetupDiEnumDeviceInterfaces, SetupDiGetClassDevsW, SetupDiGetDeviceInterfaceDetailW,
        };
        use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;

        unsafe {
            let hdev = SetupDiGetClassDevsW(
                &Self::GUID_DEVINTERFACE_USBPRINT,
                ptr::null(),
                ptr::null_mut(),
                DIGCF_PRESENT | DIGCF_DEVICEINTERFACE,
            );
            if hdev as isize == 0 || hdev as isize == INVALID_HANDLE_VALUE as isize {
                return Err(PrinterError::Io("SetupDiGetClassDevsW failed".to_string()));
            }

            let mut results = Vec::new();
            let mut index: u32 = 0;
            loop {
                let mut iface_data: SP_DEVICE_INTERFACE_DATA = mem::zeroed();
                iface_data.cbSize = mem::size_of::<SP_DEVICE_INTERFACE_DATA>() as u32;

                let ok = SetupDiEnumDeviceInterfaces(
                    hdev,
                    ptr::null_mut(),
                    &Self::GUID_DEVINTERFACE_USBPRINT,
                    index,
                    &mut iface_data,
                );
                if ok == 0 {
                    break;
                }
                index += 1;

                // First call to get required buffer size.
                let mut required_size: u32 = 0;
                SetupDiGetDeviceInterfaceDetailW(
                    hdev,
                    &iface_data,
                    ptr::null_mut(),
                    0,
                    &mut required_size,
                    ptr::null_mut(),
                );
                if required_size == 0 {
                    continue;
                }

                // Allocate a raw buffer for the detail structure + variable length path.
                let mut buffer: Vec<u8> = vec![0; required_size as usize];
                // The detail struct's cbSize is the size of the fixed header (DWORD + first WCHAR),
                // not the total buffer size. On 64-bit Windows this is 8 bytes due to alignment;
                // on 32-bit Windows it is 6. Using size_of of the repr-C struct matches both.
                let header_size = {
                    #[repr(C)]
                    struct DetailHeader {
                        cb_size: u32,
                        _device_path: [u16; 1],
                    }
                    mem::size_of::<DetailHeader>() as u32
                };
                // Write cbSize at the start of the buffer.
                *(buffer.as_mut_ptr() as *mut u32) = header_size;

                let ok = SetupDiGetDeviceInterfaceDetailW(
                    hdev,
                    &iface_data,
                    buffer.as_mut_ptr() as *mut _,
                    required_size,
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
                if ok == 0 {
                    continue;
                }

                // DevicePath starts after the cbSize field at offset 4.
                let path_ptr = buffer.as_ptr().add(4) as *const u16;
                let mut len = 0usize;
                while *path_ptr.add(len) != 0 {
                    len += 1;
                }
                let slice = std::slice::from_raw_parts(path_ptr, len);
                let device_path = OsString::from_wide(slice).to_string_lossy().into_owned();

                let (vendor_id, product_id) = parse_vid_pid(&device_path);
                results.push(WindowsUsbPrintInfo {
                    device_path,
                    vendor_id,
                    product_id,
                });
            }

            SetupDiDestroyDeviceInfoList(hdev);
            Ok(results)
        }
    }

    /// Open a Windows USB printer using its full device interface path as returned by
    /// [`WindowsUsbPrintDriver::list`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use escpos::printer::Printer;
    /// use escpos::utils::*;
    /// use escpos::driver::*;
    ///
    /// let devices = WindowsUsbPrintDriver::list().unwrap();
    /// let driver = WindowsUsbPrintDriver::open(&devices[0].device_path).unwrap();
    /// let mut printer = Printer::new(driver, Protocol::default(), None);
    /// ```
    pub fn open(device_path: &str) -> Result<Self> {
        use windows_sys::Win32::Foundation::{GENERIC_READ, GENERIC_WRITE, INVALID_HANDLE_VALUE};
        use windows_sys::Win32::Storage::FileSystem::{
            CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
        };

        let wide: Vec<u16> = device_path.encode_utf16().chain(std::iter::once(0)).collect();

        unsafe {
            let handle = CreateFileW(
                wide.as_ptr(),
                (GENERIC_READ | GENERIC_WRITE) as u32,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                ptr::null(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                ptr::null_mut(),
            );
            if handle as isize == 0 || handle as isize == INVALID_HANDLE_VALUE as isize {
                let err = io::Error::last_os_error();
                return Err(PrinterError::Io(format!("CreateFileW failed for {device_path}: {err}")));
            }

            Ok(Self {
                device_path: device_path.to_string(),
                handle: Arc::new(Mutex::new(WinHandle(handle))),
            })
        }
    }

    /// Open the first USB printer exposed by `usbprint.sys` whose device path matches the given
    /// USB vendor and product IDs.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use escpos::printer::Printer;
    /// use escpos::utils::*;
    /// use escpos::driver::*;
    ///
    /// let driver = WindowsUsbPrintDriver::open_by_vid_pid(0x0525, 0xa700).unwrap();
    /// let mut printer = Printer::new(driver, Protocol::default(), None);
    /// ```
    pub fn open_by_vid_pid(vendor_id: u16, product_id: u16) -> Result<Self> {
        let devices = Self::list()?;
        let info = devices
            .into_iter()
            .find(|d| d.vendor_id == Some(vendor_id) && d.product_id == Some(product_id))
            .ok_or_else(|| {
                PrinterError::Io(format!(
                    "No USB print device found with VID=0x{vendor_id:04x}, PID=0x{product_id:04x}"
                ))
            })?;
        Self::open(&info.device_path)
    }

    /// Parallel-port-style status value from `usbprint.sys` via `IOCTL_USBPRINT_GET_LPT_STATUS`.
    ///
    /// This is the usual way to read a hardware status word on Windows when [`Driver::read`] does
    /// not return ESC/POS `DLE EOT` reply bytes. Interpretation is IEEE-1284-related; see your
    /// printer documentation for bit meanings.
    pub fn lpt_status(&self) -> Result<u32> {
        use windows_sys::Win32::System::IO::DeviceIoControl;

        // usbprint.h: CTL_CODE(FILE_DEVICE_UNKNOWN, 12, METHOD_BUFFERED, FILE_ANY_ACCESS)
        const FILE_DEVICE_UNKNOWN: u32 = 0x22;
        const METHOD_BUFFERED: u32 = 0;
        const FILE_ANY_ACCESS: u32 = 0;
        const IOCTL_USBPRINT_GET_LPT_STATUS: u32 =
            (FILE_DEVICE_UNKNOWN << 16) | (FILE_ANY_ACCESS << 14) | (12 << 2) | METHOD_BUFFERED;

        let guard = self.handle.lock()?;
        let mut status: u32 = 0;
        let mut returned: u32 = 0;
        let ok = unsafe {
            DeviceIoControl(
                guard.0,
                IOCTL_USBPRINT_GET_LPT_STATUS,
                ptr::null(),
                0,
                &mut status as *mut u32 as *mut _,
                mem::size_of::<u32>() as u32,
                &mut returned,
                ptr::null_mut(),
            )
        };
        if ok == 0 {
            return Err(PrinterError::Io(format!(
                "IOCTL_USBPRINT_GET_LPT_STATUS failed: {}",
                io::Error::last_os_error()
            )));
        }
        Ok(status)
    }
}

#[cfg(all(feature = "usbprint", target_os = "windows"))]
impl Driver for WindowsUsbPrintDriver {
    fn name(&self) -> String {
        format!("Windows USB print ({})", self.device_path)
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        use windows_sys::Win32::Storage::FileSystem::WriteFile;

        let guard = self.handle.lock()?;
        let mut remaining = data;
        while !remaining.is_empty() {
            let mut written: u32 = 0;
            let ok = unsafe {
                WriteFile(
                    guard.0,
                    remaining.as_ptr(),
                    remaining.len() as u32,
                    &mut written,
                    ptr::null_mut(),
                )
            };
            if ok == 0 {
                return Err(PrinterError::Io(format!(
                    "WriteFile failed: {}",
                    io::Error::last_os_error()
                )));
            }
            if written == 0 {
                return Err(PrinterError::Io("WriteFile wrote 0 bytes".to_string()));
            }
            remaining = &remaining[written as usize..];
        }
        Ok(())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        use windows_sys::Win32::Storage::FileSystem::ReadFile;

        let guard = self.handle.lock()?;
        let mut read: u32 = 0;
        let ok = unsafe { ReadFile(guard.0, buf.as_mut_ptr(), buf.len() as u32, &mut read, ptr::null_mut()) };
        if ok == 0 {
            return Err(PrinterError::Io(format!(
                "ReadFile failed: {}",
                io::Error::last_os_error()
            )));
        }
        Ok(read as usize)
    }

    fn flush(&self) -> Result<()> {
        use windows_sys::Win32::Foundation::{ERROR_INVALID_FUNCTION, ERROR_NOT_SUPPORTED};
        use windows_sys::Win32::Storage::FileSystem::FlushFileBuffers;

        // `usbprint.sys` does not implement `IRP_MJ_FLUSH_BUFFERS`, so `FlushFileBuffers` returns
        // ERROR_INVALID_FUNCTION (1) / ERROR_NOT_SUPPORTED (50). Since `WriteFile` on a USB bulk
        // pipe has already handed the data to the stack, treat these as success (no-op flush).
        let guard = self.handle.lock()?;
        let ok = unsafe { FlushFileBuffers(guard.0) };
        if ok == 0 {
            let err = io::Error::last_os_error();
            let code = err.raw_os_error().unwrap_or(0) as u32;
            if code == ERROR_INVALID_FUNCTION || code == ERROR_NOT_SUPPORTED {
                return Ok(());
            }
            return Err(PrinterError::Io(format!("FlushFileBuffers failed: {err}")));
        }
        Ok(())
    }
}

/// Parse `VID_XXXX` and `PID_XXXX` (case insensitive) from a Windows device interface path.
#[cfg(all(feature = "usbprint", target_os = "windows"))]
fn parse_vid_pid(path: &str) -> (Option<u16>, Option<u16>) {
    fn parse_hex_after(haystack: &str, needle: &str) -> Option<u16> {
        let lower = haystack.to_ascii_lowercase();
        let idx = lower.find(needle)?;
        let start = idx + needle.len();
        let hex: String = haystack[start..]
            .chars()
            .take_while(|c| c.is_ascii_hexdigit())
            .take(4)
            .collect();
        if hex.is_empty() {
            None
        } else {
            u16::from_str_radix(&hex, 16).ok()
        }
    }

    (parse_hex_after(path, "vid_"), parse_hex_after(path, "pid_"))
}

//! Drivers used to send data to the printer (Network or USB)

#[cfg(any(feature = "usb", feature = "native_usb", feature = "hidapi", feature = "serial_port"))]
use crate::errors::PrinterError;
use crate::errors::Result;
#[cfg(feature = "native_usb")]
use futures_lite::future::block_on;
#[cfg(feature = "hidapi")]
use hidapi::{HidApi, HidDevice};
#[cfg(feature = "native_usb")]
use nusb::transfer::RequestBuffer;
#[cfg(feature = "usb")]
use rusb::{Context, DeviceHandle, Direction, TransferType, UsbContext};
#[cfg(feature = "serial_port")]
use serialport::SerialPort;
use std::{
    cell::RefCell,
    fs::File,
    io::{self, Read, Write},
    net::{IpAddr, SocketAddr, TcpStream},
    path::Path,
    rc::Rc,
    time::Duration,
};

/// Default timeout in seconds for read/write operations
const DEFAULT_TIMEOUT_SECONDS: u64 = 5;

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
    stream: Rc<RefCell<TcpStream>>,
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
            stream: Rc::new(RefCell::new(stream)),
            timeout,
        })
    }
}

impl Driver for NetworkDriver {
    fn name(&self) -> String {
        format!("network ({}:{})", self.host, self.port)
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        let mut stream = self.stream.try_borrow_mut()?;
        stream.set_write_timeout(Some(self.timeout))?;

        Ok(stream.write_all(data)?)
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        let mut stream = self.stream.try_borrow_mut()?;
        stream.set_read_timeout(Some(self.timeout))?;

        Ok(stream.read(buf)?)
    }

    fn flush(&self) -> Result<()> {
        Ok(self.stream.try_borrow_mut()?.flush()?)
    }
}

// ================ File driver ================

/// Driver for USB printer using file
#[derive(Clone)]
pub struct FileDriver {
    path: String,
    file: Rc<RefCell<File>>,
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
            file: Rc::new(RefCell::new(file)),
        })
    }
}

impl Driver for FileDriver {
    fn name(&self) -> String {
        format!("file ({})", self.path)
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        self.file.try_borrow_mut()?.write_all(data)?;
        Ok(())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        Ok(self.file.try_borrow_mut()?.read(buf)?)
    }

    fn flush(&self) -> Result<()> {
        Ok(self.file.try_borrow_mut()?.flush()?)
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
    device: Rc<RefCell<DeviceHandle<Context>>>,
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
    /// let driver = UsbDriver::open(0x0525, 0xa700, None).unwrap();
    /// let mut printer = Printer::new(driver, Protocol::default(), None);
    /// ```
    pub fn open(vendor_id: u16, product_id: u16, timeout: Option<Duration>) -> Result<Self> {
        let context = Context::new().map_err(|e| PrinterError::Io(e.to_string()))?;
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
                    Ok(mut device_handle) => {
                        #[cfg(not(target_os = "windows"))]
                        match device_handle.kernel_driver_active(interface_number) {
                            Ok(active) => {
                                if active {
                                    if let Err(e) = device_handle.detach_kernel_driver(interface_number) {
                                        return Err(PrinterError::Io(e.to_string()));
                                    }
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
                            device: Rc::new(RefCell::new(device_handle)),
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
            .try_borrow_mut()?
            .write_bulk(self.output_endpoint, data, self.timeout)
            .map_err(|e| PrinterError::Io(e.to_string()))?;
        Ok(())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        self.device
            .try_borrow_mut()?
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
    device: Rc<RefCell<nusb::Interface>>,
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
            .map_err(|e| PrinterError::Io(e.to_string()))?
            .find(|dev| dev.vendor_id() == vendor_id && dev.product_id() == product_id)
            .ok_or(PrinterError::Io("USB device not found".to_string()))?;
        let device = device_info.open().map_err(|e| PrinterError::Io(e.to_string()))?;

        // Get endpoints
        let configuration = device
            .active_configuration()
            .map_err(|e| PrinterError::Io(e.to_string()))?;

        let (output_endpoint, input_endpoint) = match configuration.interface_alt_settings().next() {
            Some(settings) => {
                let endpoints = settings.endpoints();
                let (mut output, mut input) = (None, None);

                for endpoint in endpoints {
                    if endpoint.transfer_type() == nusb::transfer::EndpointType::Bulk
                        && endpoint.direction() == nusb::transfer::Direction::Out
                    {
                        output = Some(endpoint.address())
                    } else if endpoint.transfer_type() == nusb::transfer::EndpointType::Bulk
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

        #[cfg(not(target_os = "windows"))]
        let interface = device
            .detach_and_claim_interface(interface_number)
            .map_err(|e| PrinterError::Io(e.to_string()))?;
        #[cfg(target_os = "windows")]
        let interface = device
            .claim_interface(interface_number)
            .map_err(|e| PrinterError::Io(e.to_string()))?;

        Ok(Self {
            vendor_id,
            product_id,
            output_endpoint,
            input_endpoint,
            device: Rc::new(RefCell::new(interface)),
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
        block_on(
            self.device
                .try_borrow_mut()?
                .bulk_out(self.output_endpoint, data.to_vec()),
        )
        .into_result()
        .map_err(|e| PrinterError::Io(e.to_string()))?;
        Ok(())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        // Seems to read responses one by one
        let mut size = 0;
        for b in buf.iter_mut() {
            let result = block_on(
                self.device
                    .try_borrow_mut()?
                    .bulk_in(self.input_endpoint, RequestBuffer::new(1)),
            )
            .into_result()
            .map_err(|e| PrinterError::Io(e.to_string()))?;

            if !result.is_empty() {
                *b = result[0];
                size += 1;
            }
        }

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
    device: Rc<RefCell<HidDevice>>,
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
            device: Rc::new(RefCell::new(device)),
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
            .try_borrow_mut()?
            .write(data)
            .map_err(|e| PrinterError::Io(e.to_string()))?;
        Ok(())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        self.device
            .try_borrow_mut()?
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
    port: Rc<RefCell<Box<dyn SerialPort>>>,
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
            port: Rc::new(RefCell::new(port)),
        })
    }
}

#[cfg(feature = "serial_port")]
impl Driver for SerialPortDriver {
    fn name(&self) -> String {
        format!("Serial port ({})", self.path)
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        self.port.try_borrow_mut()?.write_all(data)?;

        Ok(())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        let mut port = self.port.try_borrow_mut()?;
        port.set_timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS))
            .map_err(|e| PrinterError::Io(e.to_string()))?;
        Ok(port.read(buf)?)
    }

    fn flush(&self) -> Result<()> {
        Ok(self.port.try_borrow_mut()?.flush()?)
    }
}

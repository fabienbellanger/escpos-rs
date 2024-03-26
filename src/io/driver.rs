//! Drivers used to send data to the printer (Network or USB)

#[cfg(any(feature = "usb", feature = "hidapi", feature = "serial_port"))]
use crate::errors::PrinterError;
use crate::errors::Result;
#[cfg(feature = "hidapi")]
use hidapi::{HidApi, HidDevice};
#[cfg(feature = "usb")]
use rusb::{Context, DeviceHandle, Direction, TransferType, UsbContext};
#[cfg(feature = "serial_port")]
use serialport::SerialPort;
use std::time::Duration;
use std::{
    cell::RefCell,
    fs::File,
    io::{self, Write},
    net::TcpStream,
    path::Path,
};
use std::{io::Read, rc::Rc};

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
}

impl NetworkDriver {
    /// Open the network driver
    pub fn open(host: &str, port: u16) -> Result<Self> {
        Ok(Self {
            host: host.to_string(),
            port,
            stream: Rc::new(RefCell::new(TcpStream::connect((host, port))?)),
        })
    }
}

impl Driver for NetworkDriver {
    fn name(&self) -> String {
        format!("network ({}:{})", self.host, self.port)
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        self.stream.try_borrow_mut()?.write_all(data)?;
        Ok(())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        let mut stream = self.stream.try_borrow_mut()?;
        stream.set_read_timeout(Some(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS)))?;

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

// ================ USB driver ================

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
    pub fn open(vendor_id: u16, product_id: u16, timeout: Option<Duration>) -> Result<Self> {
        let context = Context::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let devices = context.devices().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

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
                    .ok_or_else(|| PrinterError::Io("no suitable endpoints found for USB device".to_string()))?;

                return match device.open() {
                    Ok(mut device_handle) => {
                        #[cfg(not(target_os = "windows"))]
                        match device_handle.kernel_driver_active(0) {
                            Ok(active) => {
                                if active {
                                    if let Err(e) = device_handle.detach_kernel_driver(0) {
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
                    Err(_) => Err(PrinterError::Io("USB device busy".to_string())),
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
            "USB (VID: {}, PID: {}, output endpoint: {})",
            self.vendor_id, self.product_id, self.output_endpoint
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
        Ok(self
            .device
            .try_borrow_mut()?
            .read_bulk(self.input_endpoint, buf, self.timeout)
            .map_err(|e| PrinterError::Io(e.to_string()))?)
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
        Ok(self
            .device
            .try_borrow_mut()?
            .read_timeout(buf, (DEFAULT_TIMEOUT_SECONDS * 1_000) as i32) // TODO: Do better!
            .map_err(|e| PrinterError::Io(e.to_string()))?)
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

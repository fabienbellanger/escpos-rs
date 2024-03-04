//! Drivers used to send data to the printer (Network or USB)

use crate::errors::{PrinterError, Result};
use hidapi::{HidApi, HidDevice};
use serialport::SerialPort;
use std::rc::Rc;
use std::time::Duration;
use std::{
    cell::RefCell,
    fs::File,
    io::{self, Write},
    net::TcpStream,
    path::Path,
};

pub trait Driver {
    /// Driver name
    fn name(&self) -> String;

    /// Write data
    fn write(&self, data: &[u8]) -> Result<()>;

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
    device: Rc<RefCell<HidDevice>>,
}

#[cfg(feature = "usb")]
impl UsbDriver {
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

#[cfg(feature = "usb")]
impl Driver for UsbDriver {
    fn name(&self) -> String {
        format!("USB (VID: {}, PID: {})", self.vendor_id, self.product_id)
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        self.device
            .try_borrow_mut()?
            .write(data)
            .map_err(|e| PrinterError::Io(e.to_string()))?;
        Ok(())
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

    fn flush(&self) -> Result<()> {
        Ok(self.port.try_borrow_mut()?.flush()?)
    }
}

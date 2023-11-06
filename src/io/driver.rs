//! Drivers used to send data to the printer (Network or USB)

use crate::errors::Result;
use std::rc::Rc;
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

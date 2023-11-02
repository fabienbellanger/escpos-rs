//! Drivers

use crate::errors::Result;
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
pub struct ConsoleDriver {}

impl ConsoleDriver {
    pub fn open() -> Self {
        Self {}
    }
}

impl Driver for ConsoleDriver {
    fn name(&self) -> String {
        "console".to_owned()
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        Ok(io::stdout().write_all(data)?)
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }
}

// ================ Network driver ================

/// Driver for network printer
pub struct NetworkDriver {
    host: String,
    port: u16,
    stream: RefCell<TcpStream>,
}

impl NetworkDriver {
    pub fn open(host: &str, port: u16) -> Result<Self> {
        Ok(Self {
            host: host.to_string(),
            port,
            stream: RefCell::new(TcpStream::connect((host, port))?),
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

/// Driver for USB printer
pub struct FileDriver {
    path: String,
    file: RefCell<File>,
}

impl FileDriver {
    pub fn open(path: &Path) -> Result<Self> {
        let file = File::options().read(true).append(true).open(path)?;
        Ok(Self {
            path: path.to_string_lossy().to_string(),
            file: RefCell::new(file),
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

//! Drivers

use crate::errors::Result;
use std::io::{self, Write};

pub trait Driver {
    /// Driver name
    fn name(&self) -> &str;

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
    fn name(&self) -> &str {
        "console"
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        io::stdout().write_all(data).map_err(Into::into)
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }
}

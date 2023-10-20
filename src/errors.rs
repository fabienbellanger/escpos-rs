//! Custom errors

use std::{borrow::Cow, fmt, io};

/// Custom Result
pub type Result<T> = std::result::Result<T, PrinterError>;

/// Printer error
#[derive(Debug)]
pub enum PrinterError {
    Io(String),
    Input(String),
    Network(String),
}

impl fmt::Display for PrinterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PrinterError::Io(ref err) => write!(f, "IO error: {err}"),
            PrinterError::Network(ref err) => write!(f, "Network error: {err}"),
            PrinterError::Input(ref err) => write!(f, "Input error: {err}"),
        }
    }
}

impl From<io::Error> for PrinterError {
    fn from(err: std::io::Error) -> PrinterError {
        PrinterError::Io(err.to_string())
    }
}

impl From<Cow<'_, str>> for PrinterError {
    fn from(value: Cow<'_, str>) -> Self {
        PrinterError::Io(value.into_owned())
    }
}

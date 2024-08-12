//! Custom error

#[cfg(feature = "graphics")]
use image::ImageError;
#[cfg(feature = "ui")]
use std::string::FromUtf8Error;
use std::{borrow::Cow, cell::BorrowMutError, fmt, io, num::TryFromIntError};

/// Custom Result for `PrinterError`
pub type Result<T> = std::result::Result<T, PrinterError>;

/// Printer error
#[derive(Debug)]
pub enum PrinterError {
    Io(String),
    Input(String),
    InvalidResponse(String),
}

impl std::error::Error for PrinterError {}

impl fmt::Display for PrinterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PrinterError::Io(ref err) => write!(f, "IO error: {err}"),
            PrinterError::Input(ref err) => write!(f, "Input error: {err}"),
            PrinterError::InvalidResponse(ref err) => write!(f, "Invalid response: {err}"),
        }
    }
}

impl From<io::Error> for PrinterError {
    fn from(err: io::Error) -> PrinterError {
        PrinterError::Io(err.to_string())
    }
}

impl From<Cow<'_, str>> for PrinterError {
    fn from(value: Cow<'_, str>) -> Self {
        PrinterError::Io(value.into_owned())
    }
}

impl From<BorrowMutError> for PrinterError {
    fn from(err: BorrowMutError) -> Self {
        PrinterError::Io(err.to_string())
    }
}

impl From<TryFromIntError> for PrinterError {
    fn from(err: TryFromIntError) -> Self {
        PrinterError::Io(err.to_string())
    }
}

#[cfg(feature = "graphics")]
impl From<ImageError> for PrinterError {
    fn from(err: ImageError) -> Self {
        PrinterError::Io(err.to_string())
    }
}

#[cfg(feature = "ui")]
impl From<FromUtf8Error> for PrinterError {
    fn from(err: FromUtf8Error) -> Self {
        PrinterError::Io(err.to_string())
    }
}

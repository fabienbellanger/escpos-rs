#![allow(dead_code)] // TODO: Remove!

//! DataMatrix

use crate::errors::{PrinterError, Result};
use std::fmt;

/// DataMatrix type
#[derive(Debug, Clone, Copy)]
pub enum DataMatrixType {
    Square(u8),
    Rectangle(u8, u8),
}

impl Default for DataMatrixType {
    fn default() -> Self {
        Self::Square(0)
    }
}

impl fmt::Display for DataMatrixType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataMatrixType::Square(d) => write!(f, "DataMatrix Square ({d}, {d})"),
            DataMatrixType::Rectangle(d1, d2) => write!(f, "DataMatrix Rectangle ({d1}, {d2})"),
        }
    }
}

impl TryFrom<DataMatrixType> for (u8, u8, u8) {
    type Error = PrinterError;

    fn try_from(value: DataMatrixType) -> core::result::Result<Self, Self::Error> {
        match value {
            DataMatrixType::Square(d) => {
                let possible_values = [
                    0, 10, 12, 14, 16, 18, 20, 22, 24, 26, 32, 36, 40, 44, 48, 52, 64, 72, 80, 88, 96, 104, 120, 132,
                    144,
                ];
                if !possible_values.contains(&d) {
                    return Err(PrinterError::Input(format!(
                        "invalid DataMatrix number of rows and columns: {d}"
                    )));
                }
                Ok((0, d, d))
            }
            DataMatrixType::Rectangle(d1, d2) => {
                let possible_values = [
                    (8, 0),
                    (8, 18),
                    (8, 32),
                    (12, 0),
                    (12, 26),
                    (12, 36),
                    (16, 0),
                    (16, 36),
                    (16, 48),
                ];
                if !possible_values.contains(&(d1, d2)) {
                    return Err(PrinterError::Input(format!(
                        "invalid DataMatrix number of rows and columns: ({d1}, {d2})"
                    )));
                }
                Ok((1, d1, d2))
            }
        }
    }
}

/// DataMatrix option
#[derive(Debug)]
pub struct DataMatrixOption {
    pub(crate) code_type: DataMatrixType,
    pub(crate) size: u8,
}

impl Default for DataMatrixOption {
    fn default() -> Self {
        Self {
            code_type: DataMatrixType::default(),
            size: 3,
        }
    }
}

impl DataMatrixOption {
    /// Create a new `DataMatrixOption`
    pub fn new(code_type: DataMatrixType, size: u8) -> Result<Self> {
        if !(2..=16).contains(&size) {
            return Err(PrinterError::Input(format!("DataMatrix size must in 2 - 16: {size}")));
        }

        Ok(Self { code_type, size })
    }
}

/// DataMatrix
#[derive(Debug)]
pub struct DataMatrix {
    pub data: String,
    pub option: DataMatrixOption,
}

impl DataMatrix {
    /// Create a new `DataMatrix`
    pub fn new(data: &str, option: DataMatrixOption) -> Self {
        Self {
            data: data.to_string(),
            option,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::Result;

    #[test]
    fn test_tyr_from_data_matrix_type_square() {
        let t: Result<(u8, u8, u8)> = DataMatrixType::Square(0).try_into();
        assert!(t.is_ok());

        let t: Result<(u8, u8, u8)> = DataMatrixType::Square(2).try_into();
        assert!(t.is_err());
    }

    #[test]
    fn test_tyr_from_data_matrix_type_rectangle() {
        let t: Result<(u8, u8, u8)> = DataMatrixType::Rectangle(8, 0).try_into();
        assert!(t.is_ok());

        let t: Result<(u8, u8, u8)> = DataMatrixType::Rectangle(0, 0).try_into();
        assert!(t.is_err());

        let t: Result<(u8, u8, u8)> = DataMatrixType::Rectangle(7, 8).try_into();
        assert!(t.is_err());
    }

    #[test]
    fn test_tyr_from_data_matrix_option() {
        assert!(DataMatrixOption::new(DataMatrixType::default(), 3).is_ok());
        assert!(DataMatrixOption::new(DataMatrixType::default(), 1).is_err());
        assert!(DataMatrixOption::new(DataMatrixType::default(), 17).is_err());
    }
}

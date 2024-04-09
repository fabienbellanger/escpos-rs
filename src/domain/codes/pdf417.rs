//! PDF417

use crate::errors::{PrinterError, Result};
use std::fmt;

/// PDF417 correction level
#[derive(Debug, Clone, Copy)]
pub enum Pdf417CorrectionLevel {
    Level0,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    Ratio(u8),
}

impl Default for Pdf417CorrectionLevel {
    fn default() -> Self {
        Self::Ratio(1)
    }
}

impl fmt::Display for Pdf417CorrectionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pdf417CorrectionLevel::Level0 => write!(f, "PDF417 correction level set by level 0"),
            Pdf417CorrectionLevel::Level1 => write!(f, "PDF417 correction level set by level 1"),
            Pdf417CorrectionLevel::Level2 => write!(f, "PDF417 correction level set by level 2"),
            Pdf417CorrectionLevel::Level3 => write!(f, "PDF417 correction level set by level 3"),
            Pdf417CorrectionLevel::Level4 => write!(f, "PDF417 correction level set by level 4"),
            Pdf417CorrectionLevel::Level5 => write!(f, "PDF417 correction level set by level 5"),
            Pdf417CorrectionLevel::Level6 => write!(f, "PDF417 correction level set by level 6"),
            Pdf417CorrectionLevel::Level7 => write!(f, "PDF417 correction level set by level 7"),
            Pdf417CorrectionLevel::Level8 => write!(f, "PDF417 correction level set by level 8"),
            Pdf417CorrectionLevel::Ratio(value) => write!(f, "PDF417 correction level set by ratio {value}"),
        }
    }
}

impl TryFrom<Pdf417CorrectionLevel> for (u8, u8) {
    type Error = PrinterError;
    fn try_from(value: Pdf417CorrectionLevel) -> core::result::Result<Self, Self::Error> {
        match value {
            Pdf417CorrectionLevel::Level0 => Ok((48, 48)),
            Pdf417CorrectionLevel::Level1 => Ok((48, 49)),
            Pdf417CorrectionLevel::Level2 => Ok((48, 50)),
            Pdf417CorrectionLevel::Level3 => Ok((48, 51)),
            Pdf417CorrectionLevel::Level4 => Ok((48, 52)),
            Pdf417CorrectionLevel::Level5 => Ok((48, 53)),
            Pdf417CorrectionLevel::Level6 => Ok((48, 54)),
            Pdf417CorrectionLevel::Level7 => Ok((48, 55)),
            Pdf417CorrectionLevel::Level8 => Ok((48, 56)),
            Pdf417CorrectionLevel::Ratio(value) if (1..=40).contains(&value) => Ok((49, value)),
            Pdf417CorrectionLevel::Ratio(value) => Err(PrinterError::Input(format!(
                "invalid PDF417 correction level: Ratio({value})"
            ))),
        }
    }
}

/// PDF417 type
#[derive(Debug, Default, Clone, Copy)]
pub enum Pdf417Type {
    #[default]
    Standard,
    Truncated,
}

impl fmt::Display for Pdf417Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pdf417Type::Standard => write!(f, "PDF417 standard"),
            Pdf417Type::Truncated => write!(f, "PDF417 truncated"),
        }
    }
}

impl From<Pdf417Type> for u8 {
    fn from(value: Pdf417Type) -> Self {
        match value {
            Pdf417Type::Standard => 0,
            Pdf417Type::Truncated => 1,
        }
    }
}

/// PDF417 option
// TODO: Make all Option type (barcode, qrcode, GS1, etc.) pub(crate) instead of pub?
#[derive(Debug, Default)]
pub struct Pdf417Option {
    columns: u8,    // Default: 0
    rows: u8,       // Default: 0
    width: u8,      // Default: ?
    row_height: u8, // Default: ?
    code_type: Pdf417Type,
    correction_level: Pdf417CorrectionLevel,
}

impl Pdf417Option {
    /// Create a new `Pdf417Option`
    pub fn new(
        columns: u8,
        rows: u8,
        width: u8,
        row_height: u8,
        code_type: Pdf417Type,
        correction_level: Pdf417CorrectionLevel,
    ) -> Result<Self> {
        if !(0..=30).contains(&columns) {
            return Err(PrinterError::Input(format!(
                "number of PDF417 columns is not valid(0-30): {columns}"
            )));
        }

        if rows != 0 && !(3..=90).contains(&rows) {
            return Err(PrinterError::Input(format!(
                "number of PDF417 rows is not valid(0, 3-90): {rows}"
            )));
        }

        Ok(Self {
            columns,
            rows,
            width,
            row_height,
            code_type,
            correction_level,
        })
    }

    /// Get number of columns
    pub fn columns(&self) -> u8 {
        self.columns
    }

    /// Get number of rows
    pub fn rows(&self) -> u8 {
        self.rows
    }

    /// Get width
    pub fn width(&self) -> u8 {
        self.width
    }

    /// Get row height
    pub fn row_height(&self) -> u8 {
        self.row_height
    }

    /// Get code type
    pub fn code_type(&self) -> Pdf417Type {
        self.code_type
    }

    /// Get correction level
    pub fn correction_level(&self) -> Pdf417CorrectionLevel {
        self.correction_level
    }
}

/// PDF417
#[derive(Debug)]
pub struct Pdf417 {
    pub data: String,
    pub option: Pdf417Option,
}

impl Pdf417 {
    /// Create a new `Pdf417`
    pub fn new(data: &str, option: Pdf417Option) -> Self {
        Self {
            data: data.to_string(),
            option,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pfd417_option_new() {
        assert!(Pdf417Option::new(31, 0, 8, 8, Pdf417Type::Standard, Pdf417CorrectionLevel::Level0).is_err());
        assert!(Pdf417Option::new(0, 2, 8, 8, Pdf417Type::Standard, Pdf417CorrectionLevel::Level0).is_err());
        assert!(Pdf417Option::new(0, 100, 8, 8, Pdf417Type::Standard, Pdf417CorrectionLevel::Level0).is_err());
        assert!(Pdf417Option::new(0, 0, 8, 8, Pdf417Type::Standard, Pdf417CorrectionLevel::Level0).is_ok());
    }
}

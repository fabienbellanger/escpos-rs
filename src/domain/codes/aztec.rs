//! Aztec code

use crate::errors::{PrinterError, Result};
use std::fmt;

/// Aztec code mode
#[derive(Debug, Clone, Copy)]
pub enum AztecMode {
    FullRange(u8),
    Compact(u8),
}

impl Default for AztecMode {
    fn default() -> Self {
        Self::FullRange(0)
    }
}

impl fmt::Display for AztecMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AztecMode::FullRange(n) => write!(f, "Aztec code Full-Range ({n})"),
            AztecMode::Compact(n) => write!(f, "Aztec code Compact ({n})"),
        }
    }
}

impl TryFrom<AztecMode> for (u8, u8) {
    type Error = PrinterError;

    fn try_from(value: AztecMode) -> core::result::Result<Self, Self::Error> {
        match value {
            AztecMode::FullRange(n) => {
                if n != 0 && !(4..=32).contains(&n) {
                    return Err(PrinterError::Input(format!(
                        "invalid Aztec code Full-Range number of data layers (0, 4-32): {n}"
                    )));
                }
                Ok((0, n))
            }
            AztecMode::Compact(n) => {
                if !(0..=4).contains(&n) {
                    return Err(PrinterError::Input(format!(
                        "invalid Aztec code Compact number of data layers (0, 1-4): {n}"
                    )));
                }
                Ok((1, n))
            }
        }
    }
}

/// Aztec code option
#[derive(Debug)]
pub struct AztecOption {
    mode: AztecMode,
    size: u8,
    correction_level: u8,
}

impl Default for AztecOption {
    fn default() -> Self {
        Self {
            mode: AztecMode::default(),
            size: 3,
            correction_level: 23,
        }
    }
}

impl AztecOption {
    /// Create a new `AztecOption`
    pub fn new(mode: AztecMode, size: u8, correction_level: u8) -> Result<Self> {
        if !(2..=16).contains(&size) {
            return Err(PrinterError::Input(format!("invalid Aztec size (2-16): {size}")));
        }

        if !(5..=95).contains(&correction_level) {
            return Err(PrinterError::Input(format!(
                "invalid Aztec error correction level (5-95): {size}"
            )));
        }

        Ok(Self {
            mode,
            size,
            correction_level,
        })
    }

    /// Get mode
    pub fn mode(&self) -> AztecMode {
        self.mode
    }

    /// Get size
    pub fn size(&self) -> u8 {
        self.size
    }

    /// Get error correction level
    pub fn correction_level(&self) -> u8 {
        self.correction_level
    }
}

/// Aztec code
#[derive(Debug)]
pub struct Aztec {
    pub data: String,
    pub option: AztecOption,
}

impl Aztec {
    /// Create a new `Aztec`
    pub fn new(data: &str, option: AztecOption) -> Self {
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
    fn test_try_from_aztec_mode() {
        // Full-Range
        let t: Result<(u8, u8)> = AztecMode::FullRange(0).try_into();
        assert!(t.is_ok());
        let t: Result<(u8, u8)> = AztecMode::FullRange(4).try_into();
        assert!(t.is_ok());
        let t: Result<(u8, u8)> = AztecMode::FullRange(32).try_into();
        assert!(t.is_ok());
        let t: Result<(u8, u8)> = AztecMode::FullRange(2).try_into();
        assert!(t.is_err());
        let t: Result<(u8, u8)> = AztecMode::FullRange(33).try_into();
        assert!(t.is_err());

        // Compact
        let t: Result<(u8, u8)> = AztecMode::Compact(0).try_into();
        assert!(t.is_ok());
        let t: Result<(u8, u8)> = AztecMode::Compact(4).try_into();
        assert!(t.is_ok());
        let t: Result<(u8, u8)> = AztecMode::Compact(5).try_into();
        assert!(t.is_err());
    }

    #[test]
    fn test_aztec_option_new() {
        assert!(AztecOption::new(AztecMode::default(), 3, 23).is_ok());
        assert!(AztecOption::new(AztecMode::default(), 1, 15).is_err());
        assert!(AztecOption::new(AztecMode::default(), 17, 15).is_err());
        assert!(AztecOption::new(AztecMode::default(), 3, 4).is_err());
        assert!(AztecOption::new(AztecMode::default(), 3, 96).is_err());
    }
}

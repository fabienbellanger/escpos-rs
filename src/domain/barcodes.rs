//! Barcodes

#![cfg(feature = "barcode")]

use crate::errors::Result;
use std::fmt;

const CODE39_VALID_CHARS: [char; 44] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '$', '%', '*', '+', '-', '.', '/', 'A', 'B', 'C', 'D', 'E', 'F',
    'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', ' ',
];
const CODABAR_VALID_CHARS: [char; 24] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'a', 'b', 'c', 'd', '$', '+', '-', '.', '/',
    ':',
];

/// Barcode system (function A used)
#[derive(Debug, Clone, Copy)]
pub enum BarcodeSystem {
    UPCA = 0,
    UPCE = 1,
    EAN13 = 2,
    EAN8 = 3,
    CODE39 = 4,
    ITF = 5,
    CODABAR = 6,
}

impl fmt::Display for BarcodeSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BarcodeSystem::UPCA => write!(f, "UPC-A"),
            BarcodeSystem::UPCE => write!(f, "UPC-E"),
            BarcodeSystem::EAN8 => write!(f, "EAN8"),
            BarcodeSystem::EAN13 => write!(f, "EAN13"),
            BarcodeSystem::CODE39 => write!(f, "CODE39"),
            BarcodeSystem::ITF => write!(f, "ITF"),
            BarcodeSystem::CODABAR => write!(f, "CODABAR"),
        }
    }
}

/// Barcodes function A
#[derive(Debug)]
pub enum BarcodeFont {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
}

impl fmt::Display for BarcodeFont {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BarcodeFont::A => write!(f, "Barcode Font A"),
            BarcodeFont::B => write!(f, "Barcode Font B"),
            BarcodeFont::C => write!(f, "Barcode Font C"),
            BarcodeFont::D => write!(f, "Barcode Font D"),
            BarcodeFont::E => write!(f, "Barcode Font E"),
        }
    }
}

/// Barcodes function A
#[derive(Debug)]
pub enum BarcodePosition {
    None = 0,
    Above = 1,
    Below = 2,
    Both = 3,
}

impl fmt::Display for BarcodePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BarcodePosition::None => write!(f, "Barcode HRI characters None"),
            BarcodePosition::Above => write!(f, "Barcode HRI characters Above"),
            BarcodePosition::Below => write!(f, "Barcode HRI characters Below"),
            BarcodePosition::Both => write!(f, "Barcode HRI characters Both above and below"),
        }
    }
}

#[derive(Debug)]
pub enum BarcodeWidth {
    XS = 1,
    S = 2,
    M = 3,
    L = 4,
    XL = 5,
}

#[derive(Debug)]
pub struct BarcodeOption {
    pub width: BarcodeWidth,
    pub height: u8,
    pub font: BarcodeFont,
    pub position: BarcodePosition,
}

impl Default for BarcodeOption {
    fn default() -> Self {
        Self {
            width: BarcodeWidth::M,
            height: 100,
            font: BarcodeFont::A,
            position: BarcodePosition::Below,
        }
    }
}

#[derive(Debug)]
pub struct Barcode {
    pub system: BarcodeSystem,
    pub data: String,
    pub option: BarcodeOption,
}

impl Barcode {
    /// Create a new `Barcode`
    // TODO: Add test
    pub fn new(system: BarcodeSystem, data: &str, option: Option<BarcodeOption>) -> Result<Self> {
        Self::validate(system, data)?;

        let option = if let Some(option) = option {
            option
        } else {
            BarcodeOption::default()
        };

        Ok(Self {
            system,
            data: data.to_string(),
            option,
        })
    }

    /// Validate data
    fn validate(system: BarcodeSystem, data: &str) -> Result<()> {
        let data_len = data.len();
        let is_data_all_digits = data.chars().all(|c| c.is_ascii_digit());

        match system {
            BarcodeSystem::UPCA => {
                if is_data_all_digits && [11, 12].contains(&data_len) {
                    Ok(())
                } else {
                    Err(crate::errors::PrinterError::Input(format!(
                        "invalid UPC-A data: {data}"
                    )))
                }
            }
            BarcodeSystem::UPCE => {
                if is_data_all_digits && [6, 7, 8, 11, 12].contains(&data_len) && data_len == 6
                    || data.chars().nth(0) == Some('0')
                {
                    Ok(())
                } else {
                    Err(crate::errors::PrinterError::Input(format!(
                        "invalid UPC-E data: {data}"
                    )))
                }
            }
            BarcodeSystem::EAN8 => {
                if is_data_all_digits && [7, 8].contains(&data_len) {
                    Ok(())
                } else {
                    Err(crate::errors::PrinterError::Input(format!("invalid EAN8 data: {data}")))
                }
            }
            BarcodeSystem::EAN13 => {
                if is_data_all_digits && [12, 13].contains(&data_len) {
                    Ok(())
                } else {
                    Err(crate::errors::PrinterError::Input(format!(
                        "invalid EAN13 data: {data}"
                    )))
                }
            }
            BarcodeSystem::ITF => {
                if data_len >= 2 && is_data_all_digits {
                    Ok(())
                } else {
                    Err(crate::errors::PrinterError::Input(format!("invalid ITF data: {data}")))
                }
            }
            BarcodeSystem::CODE39 => {
                if data_len >= 1 && data.chars().all(|c| CODE39_VALID_CHARS.contains(&c)) {
                    Ok(())
                } else {
                    Err(crate::errors::PrinterError::Input(format!(
                        "invalid CODE39 data: {data}"
                    )))
                }
            }
            BarcodeSystem::CODABAR => {
                // (However, d1 = 65 – 68, dk = 65 – 68, d1 = 97 – 100, dk = 97 – 100)
                if data_len >= 2 && data.chars().all(|c| CODABAR_VALID_CHARS.contains(&c)) {
                    Ok(())
                } else {
                    Err(crate::errors::PrinterError::Input(format!(
                        "invalid CODABAR data: {data}"
                    )))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barcode_validate_upca() {
        assert!(Barcode::validate(BarcodeSystem::UPCA, "12587965874").is_ok());
        assert!(Barcode::validate(BarcodeSystem::UPCA, "125879658746").is_ok());

        assert!(Barcode::validate(BarcodeSystem::UPCA, "1258796587").is_err());
        assert!(Barcode::validate(BarcodeSystem::UPCA, "1258796587000").is_err());
        assert!(Barcode::validate(BarcodeSystem::UPCA, "1d8796587000").is_err());
    }

    #[test]
    fn test_barcode_validate_upce() {
        assert!(Barcode::validate(BarcodeSystem::UPCE, "02587965874").is_ok());
        assert!(Barcode::validate(BarcodeSystem::UPCE, "025879658746").is_ok());
        assert!(Barcode::validate(BarcodeSystem::UPCE, "02980547").is_ok());
        assert!(Barcode::validate(BarcodeSystem::UPCE, "985487").is_ok());
        assert!(Barcode::validate(BarcodeSystem::UPCE, "085487").is_ok());

        assert!(Barcode::validate(BarcodeSystem::UPCE, "1f2-58").is_err());
        assert!(Barcode::validate(BarcodeSystem::UPCE, "9805874").is_err());
        assert!(Barcode::validate(BarcodeSystem::UPCE, "92587965874").is_err());
        assert!(Barcode::validate(BarcodeSystem::UPCE, "925879658746").is_err());
        assert!(Barcode::validate(BarcodeSystem::UPCE, "92980547").is_err());
    }

    #[test]
    fn test_barcode_validate_ean8() {
        assert!(Barcode::validate(BarcodeSystem::EAN8, "0325874").is_ok());
        assert!(Barcode::validate(BarcodeSystem::EAN8, "98574587").is_ok());

        assert!(Barcode::validate(BarcodeSystem::EAN8, "5g47u29").is_err());
        assert!(Barcode::validate(BarcodeSystem::EAN8, "980587407").is_err());
    }

    #[test]
    fn test_barcode_validate_ean13() {
        assert!(Barcode::validate(BarcodeSystem::EAN13, "012403258746").is_ok());
        assert!(Barcode::validate(BarcodeSystem::EAN13, "0124032587468").is_ok());

        assert!(Barcode::validate(BarcodeSystem::EAN13, "01240325874").is_err());
        assert!(Barcode::validate(BarcodeSystem::EAN13, "98058740701009").is_err());
        assert!(Barcode::validate(BarcodeSystem::EAN13, "9805874070s09").is_err());
    }

    #[test]
    fn test_barcode_validate_itf() {
        assert!(Barcode::validate(BarcodeSystem::ITF, "01").is_ok());
        assert!(Barcode::validate(BarcodeSystem::ITF, "0124032587468").is_ok());

        assert!(Barcode::validate(BarcodeSystem::ITF, "").is_err());
        assert!(Barcode::validate(BarcodeSystem::ITF, "3").is_err());
        assert!(Barcode::validate(BarcodeSystem::ITF, "   ").is_err());
        assert!(Barcode::validate(BarcodeSystem::ITF, "  3 ").is_err());
        assert!(Barcode::validate(BarcodeSystem::ITF, "9805f8740701009").is_err());
        assert!(Barcode::validate(BarcodeSystem::ITF, "98f874d0d70s09").is_err());
    }

    #[test]
    fn test_barcode_validate_code39() {
        assert!(Barcode::validate(BarcodeSystem::CODE39, "3").is_ok());
        assert!(Barcode::validate(BarcodeSystem::CODE39, "01").is_ok());
        assert!(Barcode::validate(BarcodeSystem::CODE39, "   ").is_ok());
        assert!(Barcode::validate(BarcodeSystem::CODE39, "  3 ").is_ok());
        assert!(Barcode::validate(BarcodeSystem::CODE39, "0ADGH J347%F*L-M.Q/C").is_ok());

        assert!(Barcode::validate(BarcodeSystem::CODE39, "").is_err());
        assert!(Barcode::validate(BarcodeSystem::CODE39, "9805f8740701009").is_err());
        assert!(Barcode::validate(BarcodeSystem::CODE39, "98f874d0d70s09").is_err());
    }

    #[test]
    fn test_barcode_validate_codabar() {
        assert!(Barcode::validate(BarcodeSystem::CODABAR, "01").is_ok());
        assert!(Barcode::validate(BarcodeSystem::CODABAR, "4Adc/D.8/$0").is_ok());

        assert!(Barcode::validate(BarcodeSystem::CODABAR, "").is_err());
        assert!(Barcode::validate(BarcodeSystem::CODABAR, "3").is_err());
        assert!(Barcode::validate(BarcodeSystem::CODABAR, "   ").is_err());
        assert!(Barcode::validate(BarcodeSystem::CODABAR, "  3 ").is_err());
        assert!(Barcode::validate(BarcodeSystem::CODABAR, "9805f8740701009").is_err());
        assert!(Barcode::validate(BarcodeSystem::CODABAR, "98f874d0d70s09").is_err());
    }
}

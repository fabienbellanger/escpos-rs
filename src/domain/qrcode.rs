//! QR Code

#![cfg(feature = "qrcode")]

use crate::errors::{PrinterError, Result};
use std::fmt;

const QRCODE_MAX_DATA_SIZE: usize = 7089;

/// QR Code model
#[derive(Debug, Clone, Copy)]
pub enum QRCodeModel {
    Model1,
    Model2,
    Micro,
}

impl From<QRCodeModel> for u8 {
    fn from(value: QRCodeModel) -> Self {
        match value {
            QRCodeModel::Model1 => 49,
            QRCodeModel::Model2 => 50,
            QRCodeModel::Micro => 51,
        }
    }
}

impl fmt::Display for QRCodeModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QRCodeModel::Model1 => write!(f, "Model 1"),
            QRCodeModel::Model2 => write!(f, "Model 2"),
            QRCodeModel::Micro => write!(f, "Model Micro QR Code"),
        }
    }
}

/// QR Code error correction level
#[derive(Debug, Clone, Copy)]
pub enum QRCodeCorrectionLevel {
    L,
    M,
    Q,
    H,
}

impl From<QRCodeCorrectionLevel> for u8 {
    fn from(value: QRCodeCorrectionLevel) -> Self {
        match value {
            QRCodeCorrectionLevel::L => 48,
            QRCodeCorrectionLevel::M => 49,
            QRCodeCorrectionLevel::Q => 50,
            QRCodeCorrectionLevel::H => 51,
        }
    }
}

impl fmt::Display for QRCodeCorrectionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QRCodeCorrectionLevel::L => write!(f, "Error correction level L"),
            QRCodeCorrectionLevel::M => write!(f, "Error correction level M"),
            QRCodeCorrectionLevel::Q => write!(f, "Error correction level Q"),
            QRCodeCorrectionLevel::H => write!(f, "Error correction level H"),
        }
    }
}

/// QR code option
#[derive(Debug)]
pub struct QRCodeOption {
    pub model: QRCodeModel,
    pub size: u8,
    pub correction_level: QRCodeCorrectionLevel,
}

impl Default for QRCodeOption {
    fn default() -> Self {
        Self {
            model: QRCodeModel::Model1,
            size: 4,
            correction_level: QRCodeCorrectionLevel::H,
        }
    }
}

impl QRCodeOption {
    /// Create a new `QRCodeOption`
    pub fn new(model: QRCodeModel, size: u8, correction_level: QRCodeCorrectionLevel) -> Self {
        Self {
            model,
            size,
            correction_level,
        }
    }
}

/// QR code
#[derive(Debug)]
pub struct QRCode {
    pub data: String,
    pub option: QRCodeOption,
}

impl QRCode {
    /// Create a new `QRCode`
    pub fn new(data: &str, option: Option<QRCodeOption>) -> Result<Self> {
        Self::check_data(data)?;

        let option = if let Some(option) = option {
            option
        } else {
            QRCodeOption::default()
        };

        Ok(Self {
            data: data.to_string(),
            option,
        })
    }

    /// Check data
    fn check_data(data: &str) -> Result<()> {
        let bytes = data.as_bytes();
        let data_len = bytes.len();
        if data_len > QRCODE_MAX_DATA_SIZE {
            return Err(crate::errors::PrinterError::Input(format!(
                "QR code data is too long ({data_len}), its length should be smaller than 7090"
            )));
        }
        Ok(())
    }

    /// Get size (pL, pH) information
    pub fn get_size_values(data: &str) -> Result<(u8, u8)> {
        Self::check_data(data)?;

        let bytes = data.as_bytes();
        let data_len = bytes.len() + 3;
        let ph = data_len / 256;
        let pl = data_len
            .checked_add_signed(-256 * isize::try_from(ph)?)
            .ok_or(PrinterError::Input("QR code invalid data".to_owned()))?;

        Ok((u8::try_from(pl)?, u8::try_from(ph)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qrcode_new() {
        let data = "azerty123456789QTG,{";
        assert!(QRCode::new(data, None).is_ok());
        assert!(QRCode::new(
            data,
            Some(QRCodeOption::new(QRCodeModel::Model1, 4, QRCodeCorrectionLevel::L))
        )
        .is_ok());
    }

    #[test]
    fn test_qrcode_check_data() {
        let data = "azerty123456789QTG,{";
        assert!(QRCode::check_data(data).is_ok());

        let data = "azerty123456789QTG,{".repeat(400);
        assert!(QRCode::check_data(&data).is_err());
    }

    #[test]
    fn test_qrcode_get_size_values() {
        let data = "azerty123456789QTG,{".repeat(400);
        assert!(QRCode::get_size_values(&data).is_err());

        let data = "azerty123456789QTG,{";
        assert_eq!(QRCode::get_size_values(data).unwrap(), (23, 0));

        let data = "azerty123456789QTG,{".repeat(200);
        assert_eq!(QRCode::get_size_values(&data).unwrap(), (163, 15));

        let data = "";
        assert_eq!(QRCode::get_size_values(data).unwrap(), (3, 0));
    }
}

//! QR Code

#![cfg(feature = "codes_2d")]

use crate::errors::Result;
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
    model: QRCodeModel,
    size: u8,
    correction_level: QRCodeCorrectionLevel,
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

    /// Get model
    pub fn model(&self) -> QRCodeModel {
        self.model
    }

    /// Get size
    pub fn size(&self) -> u8 {
        self.size
    }

    /// Get error correction level
    pub fn correction_level(&self) -> QRCodeCorrectionLevel {
        self.correction_level
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
}

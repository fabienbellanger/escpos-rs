//! 2D GS1 DataBar

use crate::errors::{PrinterError, Result};
use std::fmt;

const EXPANDED_STACKED_VALID_CHARS: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', ' ', '!', '"', '%', '$', '\'', '(', ')', '*',
    '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '_', '{',
];

#[derive(Debug, Default, Clone, Copy)]
pub enum GS1DataBar2DType {
    #[default]
    Stacked,
    StackedOmnidirectional,
    ExpandedStacked,
}

impl fmt::Display for GS1DataBar2DType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GS1DataBar2DType::Stacked => write!(f, "GS1 DataBar Stacked"),
            GS1DataBar2DType::StackedOmnidirectional => write!(f, "GS1 DataBar Stacked Omnidirectional"),
            GS1DataBar2DType::ExpandedStacked => write!(f, "GS1 DataBar Expanded Stacked"),
        }
    }
}

impl From<GS1DataBar2DType> for u8 {
    fn from(value: GS1DataBar2DType) -> Self {
        match value {
            GS1DataBar2DType::Stacked => 72,
            GS1DataBar2DType::StackedOmnidirectional => 73,
            GS1DataBar2DType::ExpandedStacked => 76,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum GS1DataBar2DWidth {
    S,
    #[default]
    M,
    L,
}

impl fmt::Display for GS1DataBar2DWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GS1DataBar2DWidth::S => write!(f, "GS1 DataBar S width"),
            GS1DataBar2DWidth::M => write!(f, "GS1 DataBar M width"),
            GS1DataBar2DWidth::L => write!(f, "GS1 DataBar L width"),
        }
    }
}

impl From<GS1DataBar2DWidth> for u8 {
    fn from(value: GS1DataBar2DWidth) -> Self {
        match value {
            GS1DataBar2DWidth::S => 2,
            GS1DataBar2DWidth::M => 1,
            GS1DataBar2DWidth::L => 4,
        }
    }
}

/// GS1 DataBar option
#[derive(Debug, Clone, Default)]
pub struct GS1DataBar2DOption {
    width: GS1DataBar2DWidth,
    code_type: GS1DataBar2DType,
}

impl GS1DataBar2DOption {
    /// Create a new `GS1DataBar2DOption`
    pub fn new(width: GS1DataBar2DWidth, code_type: GS1DataBar2DType) -> Self {
        Self { width, code_type }
    }

    /// Get width
    pub fn width(&self) -> GS1DataBar2DWidth {
        self.width
    }

    /// Get code type
    pub fn code_type(&self) -> GS1DataBar2DType {
        self.code_type
    }
}

/// 2D GS1 DataBar
#[derive(Debug)]
pub struct GS1DataBar2D {
    pub data: String,
    pub option: GS1DataBar2DOption,
}

impl GS1DataBar2D {
    /// Create a new `GS1DataBar2D`
    pub fn new(data: &str, option: GS1DataBar2DOption) -> Result<Self> {
        Self::check_data(data, &option.code_type)?;

        Ok(Self {
            data: data.to_string(),
            option,
        })
    }

    /// Check data
    fn check_data(data: &str, code_type: &GS1DataBar2DType) -> Result<()> {
        let data_len = data.len();
        let is_data_all_digits = data.chars().all(|c| c.is_ascii_digit());

        match code_type {
            GS1DataBar2DType::Stacked | GS1DataBar2DType::StackedOmnidirectional => {
                if is_data_all_digits && data_len == 13 {
                    Ok(())
                } else {
                    Err(PrinterError::Input(format!(
                        "invalid GS1 DataBar Stacked Omnidirectional data: {data}"
                    )))
                }
            }
            GS1DataBar2DType::ExpandedStacked => {
                if (0..256).contains(&data_len) && data.chars().all(|c| EXPANDED_STACKED_VALID_CHARS.contains(&c)) {
                    Ok(())
                } else {
                    Err(PrinterError::Input(format!(
                        "invalid GS1 DataBar Stacked Omnidirectional data: {data}"
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
    fn test_gs1_databar_stacked_check_data() {
        assert!(GS1DataBar2D::check_data("1234560987654", &GS1DataBar2DType::Stacked).is_ok());
        assert!(GS1DataBar2D::check_data("123456098765", &GS1DataBar2DType::Stacked).is_err());
        assert!(GS1DataBar2D::check_data("123456098765d", &GS1DataBar2DType::Stacked).is_err());
        assert!(GS1DataBar2D::check_data("azs,rfT;YTfGq", &GS1DataBar2DType::Stacked).is_err());

        assert!(GS1DataBar2D::check_data("1234560987654", &GS1DataBar2DType::StackedOmnidirectional).is_ok());
        assert!(GS1DataBar2D::check_data("123456098765", &GS1DataBar2DType::StackedOmnidirectional).is_err());
        assert!(GS1DataBar2D::check_data("123456098765d", &GS1DataBar2DType::StackedOmnidirectional).is_err());
        assert!(GS1DataBar2D::check_data("azs,rfT;YTfGq", &GS1DataBar2DType::StackedOmnidirectional).is_err());
    }

    #[test]
    fn test_gs1_databar_expanded_check_data() {
        assert!(GS1DataBar2D::check_data("1234560987654", &GS1DataBar2DType::ExpandedStacked).is_ok());
        assert!(GS1DataBar2D::check_data("123456098765AC!,", &GS1DataBar2DType::ExpandedStacked).is_ok());
        assert!(GS1DataBar2D::check_data("123456098765d", &GS1DataBar2DType::ExpandedStacked).is_err());
        assert!(GS1DataBar2D::check_data("", &GS1DataBar2DType::ExpandedStacked).is_ok());
    }
}

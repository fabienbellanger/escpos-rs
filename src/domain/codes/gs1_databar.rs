//! GS1 DataBar

use crate::errors::{PrinterError, Result};
use std::fmt;

const EXPANDED_STACKED_VALID_CHARS: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', ' ', '!', '"', '%', '$', '\'', '(', ')', '*',
    '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '_', '{',
];

#[derive(Debug, Clone)]
pub enum GS1DataBarType {
    Stacked,
    StackedOmnidirectional,
    ExpandedStacked,
}

impl fmt::Display for GS1DataBarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GS1DataBarType::Stacked => write!(f, "GS1 DataBar Stacked"),
            GS1DataBarType::StackedOmnidirectional => write!(f, "GS1 DataBar Stacked Omnidirectional"),
            GS1DataBarType::ExpandedStacked => write!(f, "GS1 DataBar Expanded Stacked"),
        }
    }
}

impl From<GS1DataBarType> for u8 {
    fn from(value: GS1DataBarType) -> Self {
        match value {
            GS1DataBarType::Stacked => 72,
            GS1DataBarType::StackedOmnidirectional => 73,
            GS1DataBarType::ExpandedStacked => 76,
        }
    }
}

/// GS1 DataBar option
#[derive(Debug, Clone)]
pub struct GS1DataBarOption {
    pub code_type: GS1DataBarType,
}

impl GS1DataBarOption {
    /// Create a new `GS1DataBarType`
    pub fn new(code_type: GS1DataBarType) -> Self {
        Self { code_type }
    }
}

/// GS1 DataBar
#[derive(Debug)]
pub struct GS1DataBar {
    pub data: String,
    pub option: GS1DataBarOption,
}

impl GS1DataBar {
    /// Create a new `GS1DataBar`
    pub fn new(data: &str, option: GS1DataBarOption) -> Result<Self> {
        Self::check_data(data, &option.code_type)?;

        Ok(Self {
            data: data.to_string(),
            option,
        })
    }

    /// Check data
    fn check_data(data: &str, code_type: &GS1DataBarType) -> Result<()> {
        let data_len = data.len();
        let is_data_all_digits = data.chars().all(|c| c.is_ascii_digit());

        match code_type {
            GS1DataBarType::Stacked | GS1DataBarType::StackedOmnidirectional => {
                if is_data_all_digits && data_len == 13 {
                    Ok(())
                } else {
                    Err(PrinterError::Input(format!(
                        "invalid GS1 DataBar Stacked Omnidirectional data: {data}"
                    )))
                }
            }
            GS1DataBarType::ExpandedStacked => {
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
        assert!(GS1DataBar::check_data("1234560987654", &GS1DataBarType::Stacked).is_ok());
        assert!(GS1DataBar::check_data("123456098765", &GS1DataBarType::Stacked).is_err());
        assert!(GS1DataBar::check_data("123456098765d", &GS1DataBarType::Stacked).is_err());
        assert!(GS1DataBar::check_data("azs,rfT;YTfGq", &GS1DataBarType::Stacked).is_err());

        assert!(GS1DataBar::check_data("1234560987654", &GS1DataBarType::StackedOmnidirectional).is_ok());
        assert!(GS1DataBar::check_data("123456098765", &GS1DataBarType::StackedOmnidirectional).is_err());
        assert!(GS1DataBar::check_data("123456098765d", &GS1DataBarType::StackedOmnidirectional).is_err());
        assert!(GS1DataBar::check_data("azs,rfT;YTfGq", &GS1DataBarType::StackedOmnidirectional).is_err());
    }

    #[test]
    fn test_gs1_databar_expanded_check_data() {
        assert!(GS1DataBar::check_data("1234560987654", &GS1DataBarType::ExpandedStacked).is_ok());
        assert!(GS1DataBar::check_data("123456098765AC!,", &GS1DataBarType::ExpandedStacked).is_ok());
        assert!(GS1DataBar::check_data("123456098765d", &GS1DataBarType::ExpandedStacked).is_err());
        assert!(GS1DataBar::check_data("", &GS1DataBarType::ExpandedStacked).is_ok());
    }
}

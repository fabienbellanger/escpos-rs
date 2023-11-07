//! GS1 DataBar

use crate::errors::Result;
use std::fmt;

#[derive(Debug)]
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
#[derive(Debug)]
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
        Self::check_data(data)?;

        Ok(Self {
            data: data.to_string(),
            option,
        })
    }

    /// Check data
    fn check_data(data: &str) -> Result<()> {
        // TODO: Check data (ie. QR code)
        Ok(())
    }
}

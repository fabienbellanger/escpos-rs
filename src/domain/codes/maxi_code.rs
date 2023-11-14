//! MaxiCode

use std::fmt;

/// PDF417 correction level
#[derive(Debug, Default, Clone, Copy)]
pub enum MaxiCodeMode {
    #[default]
    Mode2,
    Mode3,
    Mode4,
    Mode5,
    Mode6,
}

impl fmt::Display for MaxiCodeMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaxiCodeMode::Mode2 => write!(f, "MaxiCode Mode 2"),
            MaxiCodeMode::Mode3 => write!(f, "MaxiCode Mode 3"),
            MaxiCodeMode::Mode4 => write!(f, "MaxiCode Mode 4"),
            MaxiCodeMode::Mode5 => write!(f, "MaxiCode Mode 5"),
            MaxiCodeMode::Mode6 => write!(f, "MaxiCode Mode 6"),
        }
    }
}

impl From<MaxiCodeMode> for u8 {
    fn from(value: MaxiCodeMode) -> Self {
        match value {
            MaxiCodeMode::Mode2 => 50,
            MaxiCodeMode::Mode3 => 51,
            MaxiCodeMode::Mode4 => 52,
            MaxiCodeMode::Mode5 => 53,
            MaxiCodeMode::Mode6 => 54,
        }
    }
}

#[derive(Debug)]
pub struct MaxiCode {
    pub data: String,
    pub mode: MaxiCodeMode,
}

impl MaxiCode {
    /// Create a new `MaxiCode`
    pub fn new(data: &str, mode: MaxiCodeMode) -> Self {
        Self {
            data: data.to_string(),
            mode,
        }
    }
}

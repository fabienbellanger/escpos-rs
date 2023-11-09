// TODO: Remove!
#![allow(dead_code, unused_variables)]

//! PDF417

use crate::errors::Result;
use std::fmt;

/// PDF417 correction level
#[derive(Debug)]
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

// TODO: From => TryFrom ?
impl From<Pdf417CorrectionLevel> for (u8, u8) {
    fn from(value: Pdf417CorrectionLevel) -> Self {
        match value {
            Pdf417CorrectionLevel::Level0 => (48, 48),
            Pdf417CorrectionLevel::Level1 => (48, 49),
            Pdf417CorrectionLevel::Level2 => (48, 50),
            Pdf417CorrectionLevel::Level3 => (48, 51),
            Pdf417CorrectionLevel::Level4 => (48, 52),
            Pdf417CorrectionLevel::Level5 => (48, 53),
            Pdf417CorrectionLevel::Level6 => (48, 54),
            Pdf417CorrectionLevel::Level7 => (48, 55),
            Pdf417CorrectionLevel::Level8 => (48, 56),
            Pdf417CorrectionLevel::Ratio(value) if (1..=40).contains(&value) => (49, value),
            _ => (49, 1),
        }
    }
}

/// PDF417 type
#[derive(Debug, Default)]
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
    pub(crate) columns: u8,    // Default: 0
    pub(crate) rows: u8,       // Default: 0
    pub(crate) width: u8,      // Default: ?
    pub(crate) row_height: u8, // Default: ?
    pub(crate) code_type: Pdf417Type,
    pub(crate) correction_level: Pdf417CorrectionLevel,
}

impl Pdf417Option {
    /// Create a new `Pdf417Option`
    // TODO: Add test
    pub fn new(
        columns: u8,
        rows: u8,
        width: u8,
        row_height: u8,
        code_type: Pdf417Type,
        correction_level: Pdf417CorrectionLevel,
    ) -> Result<Self> {
        todo!()
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

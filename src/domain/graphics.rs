//! Graphics

#![cfg(feature = "graphics")]

use std::fmt;

/// Graphic density
#[derive(Debug, Clone, Copy)]
pub enum GraphicDensity {
    /// 180dpi x 180dpi
    Low,
    /// 360dpi x 360dpi
    Hight,
}

impl From<GraphicDensity> for u8 {
    fn from(value: GraphicDensity) -> Self {
        match value {
            GraphicDensity::Low => 50,
            GraphicDensity::Hight => 51,
        }
    }
}

impl fmt::Display for GraphicDensity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphicDensity::Low => write!(f, "180dpi"),
            GraphicDensity::Hight => write!(f, "360dpi"),
        }
    }
}

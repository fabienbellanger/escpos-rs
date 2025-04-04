//! Graphics

#![cfg(feature = "graphics")]

use crate::errors::{PrinterError, Result};
use image::{DynamicImage, GenericImageView, Rgba};
use std::fmt;

/// Graphic density
#[derive(Debug, Clone, Copy)]
pub enum GraphicDensity {
    /// 180dpi x 180dpi
    Low,
    /// 360dpi x 360dpi
    High,
}

impl From<GraphicDensity> for u8 {
    fn from(value: GraphicDensity) -> Self {
        match value {
            GraphicDensity::Low => 50,
            GraphicDensity::High => 51,
        }
    }
}

impl fmt::Display for GraphicDensity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphicDensity::Low => write!(f, "180dpi"),
            GraphicDensity::High => write!(f, "360dpi"),
        }
    }
}

/// Graphic tone
#[derive(Debug, Clone, Copy)]
pub enum GraphicTone {
    Monochrome,
    Multiple,
}

impl From<GraphicTone> for u8 {
    fn from(value: GraphicTone) -> Self {
        match value {
            GraphicTone::Monochrome => 48,
            GraphicTone::Multiple => 52,
        }
    }
}

impl fmt::Display for GraphicTone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphicTone::Monochrome => write!(f, "Monochrome"),
            GraphicTone::Multiple => write!(f, "Multiple tone"),
        }
    }
}

/// Graphic color
#[derive(Debug, Clone, Copy)]
pub enum GraphicColor {
    Color1,
    Color2,
    Color3,
    Color4,
}

impl From<GraphicColor> for u8 {
    fn from(value: GraphicColor) -> Self {
        match value {
            GraphicColor::Color1 => 49,
            GraphicColor::Color2 => 50,
            GraphicColor::Color3 => 51,
            GraphicColor::Color4 => 52,
        }
    }
}

impl fmt::Display for GraphicColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphicColor::Color1 => write!(f, "Color 1"),
            GraphicColor::Color2 => write!(f, "Color 2"),
            GraphicColor::Color3 => write!(f, "Color 3"),
            GraphicColor::Color4 => write!(f, "Color 4"),
        }
    }
}

/// Graphic size
#[derive(Debug, Clone, Copy)]
pub enum GraphicSize {
    Normal,
    Double,
}

impl From<GraphicSize> for u8 {
    fn from(value: GraphicSize) -> Self {
        match value {
            GraphicSize::Normal => 1,
            GraphicSize::Double => 2,
        }
    }
}

impl fmt::Display for GraphicSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphicSize::Normal => write!(f, "Normal"),
            GraphicSize::Double => write!(f, "Double"),
        }
    }
}

#[derive(Debug)]
// TODO: Make fields private
pub struct GraphicOption {
    /// Image max width
    pub max_width: Option<u32>,
    /// Image max height
    pub max_height: Option<u32>,
    /// Image density
    pub density: GraphicDensity,
    /// Image tone
    pub tone: GraphicTone,
    /// Image color
    pub color: GraphicColor,
    /// Width size
    pub width_size: GraphicSize,
    /// Height size
    pub height_size: GraphicSize,
}

impl Default for GraphicOption {
    fn default() -> Self {
        Self {
            max_width: None,
            max_height: None,
            density: GraphicDensity::Low,
            tone: GraphicTone::Monochrome,
            color: GraphicColor::Color1,
            width_size: GraphicSize::Normal,
            height_size: GraphicSize::Normal,
        }
    }
}

impl GraphicOption {
    /// Create new `GraphicOption`
    pub fn new(
        density: GraphicDensity,
        tone: GraphicTone,
        color: GraphicColor,
        width_size: GraphicSize,
        height_size: GraphicSize,
        max_width: Option<u32>,
        max_height: Option<u32>,
    ) -> Self {
        Self {
            max_width,
            max_height,
            density,
            tone,
            color,
            width_size,
            height_size,
        }
    }
}

#[derive(Debug)]
pub struct Graphic {
    /// Image path
    path: String,
    /// Image option
    option: GraphicOption,
    image: DynamicImage,
}

impl Graphic {
    /// Create a new image
    pub fn new(path: &str, option: Option<GraphicOption>) -> Result<Self> {
        let img = image::open(path)?;
        let option = option.unwrap_or_default();

        // Resize image with max width and max height constraints and convert to grayscale
        let img = match (option.max_width, option.max_height) {
            (Some(max_width), None) => {
                let resized = img.resize(max_width, max_width, image::imageops::Nearest);
                resized.grayscale()
            }
            (None, Some(max_height)) => {
                let resized = img.resize(max_height, max_height, image::imageops::Nearest);
                resized.grayscale()
            }
            (Some(max_width), Some(max_height)) => {
                let resized = img.resize(max_width, max_height, image::imageops::Nearest);
                resized.grayscale()
            }
            _ => img.grayscale(),
        };

        Ok(Self {
            path: path.to_string(),
            option,
            image: img,
        })
    }

    /// Get image width
    pub fn width(&self) -> u32 {
        self.image.width()
    }

    /// Get image height
    pub fn height(&self) -> u32 {
        self.image.height()
    }

    /// Get dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    /// Get image width in bytes
    pub fn width_bytes(&self) -> u32 {
        self.width().div_ceil(8)
    }

    /// Get path
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Get image
    pub fn image(&self) -> &DynamicImage {
        &self.image
    }

    /// Get pixel
    pub fn pixel(&self, x: u32, y: u32) -> Rgba<u8> {
        self.image.get_pixel(x, y)
    }

    /// Is pixel transparent or white?
    pub fn is_blank_pixel(&self, x: u32, y: u32) -> bool {
        let pixel = self.pixel(x, y);
        // Full transparent or white
        pixel[3] == 0 || (pixel[0] & pixel[1] & pixel[2]) == 0xFF
    }

    /// Get density
    pub fn density(&self) -> u8 {
        self.option.density.into()
    }

    /// Get tone
    pub fn tone(&self) -> u8 {
        self.option.tone.into()
    }

    /// Get color
    pub fn color(&self) -> u8 {
        self.option.color.into()
    }

    /// Get width size
    pub fn width_size(&self) -> u8 {
        self.option.width_size.into()
    }

    /// Get height size
    pub fn height_size(&self) -> u8 {
        self.option.height_size.into()
    }

    /// Get (p1, p2, p3, p4)
    pub fn data_size(&self) -> Result<(u8, u8, u8, u8)> {
        let length = self.image.as_bytes().len() - 11;
        let p4 = length / 16_777_216;
        let p3 = length
            .checked_add_signed(-16_777_216 * isize::try_from(p4)?)
            .ok_or(PrinterError::Input("graphics invalid (pL, pH)".to_owned()))?
            / 65_536;
        let p2 = length
            .checked_add_signed(-16_777_216 * isize::try_from(p4)?)
            .ok_or(PrinterError::Input("graphics invalid (pL, pH)".to_owned()))?
            .checked_add_signed(-65_536 * isize::try_from(p3)?)
            .ok_or(PrinterError::Input("graphics invalid (pL, pH)".to_owned()))?
            / 256;
        let p1 = length
            .checked_add_signed(-256 * isize::try_from(p2)?)
            .ok_or(PrinterError::Input("graphics invalid (pL, pH)".to_owned()))?
            .checked_add_signed(-65_536 * isize::try_from(p3)?)
            .ok_or(PrinterError::Input("graphics invalid (pL, pH)".to_owned()))?
            .checked_add_signed(-16_777_216 * isize::try_from(p4)?)
            .ok_or(PrinterError::Input("graphics invalid (pL, pH)".to_owned()))?;

        Ok((
            u8::try_from(p1)?,
            u8::try_from(p2)?,
            u8::try_from(p3)?,
            u8::try_from(p4)?,
        ))
    }

    /// Get (xL, xH) or (yL, yH) number of dots
    // TODO: Use get_parameters_number_2 instead
    pub fn dots_per_direction(&self, length: usize) -> Result<(u8, u8)> {
        let ph = length / 256;
        let pl = length
            .checked_add_signed(-256 * isize::try_from(ph)?)
            .ok_or(PrinterError::Input("graphics invalid dots per direction".to_owned()))?;

        Ok((u8::try_from(pl)?, u8::try_from(ph)?))
    }

    /// Data in raster mode
    pub fn data(&self) -> Result<Vec<u8>> {
        let width = self.width_bytes();
        let height = self.height();

        let mut data = vec![0; (width * height) as usize];
        for y in 0..height {
            for x in 0..width {
                for b in 0..8 {
                    let i = x * 8 + b;
                    if i < self.width() && !self.is_blank_pixel(i, y) {
                        data[(y * width + x) as usize] += 0x80 >> (b & 0x7);
                    }
                }
            }
        }

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphic_width() {
        let graphic = Graphic::new("./resources/images/rust-logo-small.png", None).unwrap();
        assert_eq!(graphic.width(), 200);
    }

    #[test]
    fn test_graphic_height() {
        let graphic = Graphic::new("./resources/images/rust-logo.png", None).unwrap();
        assert_eq!(graphic.height(), 1_000);
    }
}

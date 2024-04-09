//! Bit Image

#![cfg(feature = "graphics")]

use crate::errors::{PrinterError, Result};
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use std::fmt;

/// BitImage size
#[derive(Debug, Default, Clone, Copy)]
pub enum BitImageSize {
    #[default]
    Normal,
    DoubleWidth,
    DoubleHeight,
    DoubleWidthAndHeight,
}

impl fmt::Display for BitImageSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BitImageSize::Normal => write!(f, "Normal"),
            BitImageSize::DoubleWidth => write!(f, "Double width"),
            BitImageSize::DoubleHeight => write!(f, "Double height"),
            BitImageSize::DoubleWidthAndHeight => write!(f, "Double width and height"),
        }
    }
}

impl From<&BitImageSize> for u8 {
    fn from(size: &BitImageSize) -> Self {
        match size {
            BitImageSize::Normal => 0,
            BitImageSize::DoubleWidth => 1,
            BitImageSize::DoubleHeight => 2,
            BitImageSize::DoubleWidthAndHeight => 3,
        }
    }
}

/// Bit image option
#[derive(Debug)]
pub struct BitImageOption {
    /// Image max width
    max_width: Option<u32>,
    /// Image max height
    max_height: Option<u32>,
    /// Image size
    size: BitImageSize,
}

impl Default for BitImageOption {
    fn default() -> Self {
        Self {
            max_width: Some(512),
            max_height: Some(512),
            size: BitImageSize::Normal,
        }
    }
}

impl BitImageOption {
    /// Create new `BitImageOption`
    pub fn new(max_width: Option<u32>, max_height: Option<u32>, size: BitImageSize) -> Result<Self> {
        if let Some(max_width) = max_width {
            if max_width % 8 != 0 {
                return Err(PrinterError::Input(
                    "bit image max width must be a multiple of 8".to_owned(),
                ));
            }
        }
        if let Some(max_height) = max_height {
            if max_height % 8 != 0 {
                return Err(PrinterError::Input(
                    "bit image max height must be a multiple of 8".to_owned(),
                ));
            }
        }

        Ok(Self {
            max_width,
            max_height,
            size,
        })
    }
}

#[derive(Debug)]
pub struct BitImage {
    path: String,
    image: DynamicImage,
    option: BitImageOption,
}

impl BitImage {
    /// Create a new image
    pub fn new(path: &str, option: BitImageOption) -> Result<Self> {
        let img = image::open(path)?;
        Self::from_dynamic_image(img, option, path)
    }

    /// Create a new image from bytes
    pub fn from_bytes(bytes: &[u8], option: BitImageOption) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_dynamic_image(img, option, "")
    }

    /// Create a new image from `DynamicImage`
    fn from_dynamic_image(img: DynamicImage, option: BitImageOption, path: &str) -> Result<Self> {
        // Resize image with max width and max height constraints and convert to grayscale
        let mut img = match (option.max_width, option.max_height) {
            (Some(max_width), None) => {
                if img.width() > max_width {
                    img.resize(max_width, max_width, image::imageops::Nearest)
                } else {
                    img
                }
            }
            (None, Some(max_height)) => {
                if img.height() > max_height {
                    img.resize(max_height, max_height, image::imageops::Nearest)
                } else {
                    img
                }
            }
            (Some(max_width), Some(max_height)) => {
                if img.width() > max_width || img.height() > max_height {
                    img.resize(max_width, max_height, image::imageops::Nearest)
                } else {
                    img
                }
            }
            _ => img,
        };

        // Remove alpha canal
        Self::remove_alpha(&mut img);

        // Make gray scale
        img = img.grayscale();

        Ok(Self {
            path: path.to_string(),
            image: img,
            option,
        })
    }

    /// Remove alpha canal in image
    fn remove_alpha(img: &mut DynamicImage) {
        for y in 0..img.height() {
            for x in 0..img.width() {
                let old = img.get_pixel(x, y);
                let alpha = old.0[3] as u32;
                let inverse_alpha = 255 - alpha;

                let new = Rgba::from([
                    ((old.0[0] as u32 * alpha + inverse_alpha * 255) / 255) as u8,
                    ((old.0[1] as u32 * alpha + inverse_alpha * 255) / 255) as u8,
                    ((old.0[2] as u32 * alpha + inverse_alpha * 255) / 255) as u8,
                    255,
                ]);
                img.put_pixel(x, y, new);
            }
        }
    }

    /// Get image width
    fn width(&self) -> Result<u16> {
        Ok(u16::try_from(self.image.width())?)
    }

    /// Get image height
    fn height(&self) -> Result<u16> {
        Ok(u16::try_from(self.image.height())?)
    }

    /// Get image width in bytes
    pub fn width_bytes(&self) -> Result<u16> {
        // TODO: Do better
        Ok(u16::try_from((f32::from(self.width()?) / 8.0).ceil() as usize)?)
    }

    /// Get image
    pub fn image(&self) -> &DynamicImage {
        &self.image
    }

    /// Get image pixel
    pub fn pixel(&self, x: u32, y: u32) -> Rgba<u8> {
        self.image.get_pixel(x, y)
    }

    /// Get image path
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Get size
    pub fn size(&self) -> &BitImageSize {
        &self.option.size
    }

    /// Get image width in bytes
    pub fn with_bytes_u8(&self) -> Result<Vec<u8>> {
        let width = self.width_bytes()?;
        let xh = width / 256;
        let xl = width
            .checked_add_signed(-256 * i16::try_from(xh)?)
            .ok_or(PrinterError::Input("Bit image width invalid data".to_owned()))?;

        Ok(vec![u8::try_from(xl)?, u8::try_from(xh)?])
    }

    /// Get image height
    pub fn height_u8(&self) -> Result<Vec<u8>> {
        let height = self.height()?;
        let yh = height / 256;
        let yl = height
            .checked_add_signed(-256 * i16::try_from(yh)?)
            .ok_or(PrinterError::Input("Bit image height invalid data".to_owned()))?;

        Ok(vec![u8::try_from(yl)?, u8::try_from(yh)?])
    }

    /// Is the pixel black?
    fn is_pixel_black(&self, x: u16, y: u16) -> bool {
        self.pixel(u32::from(x), u32::from(y)).0[0] <= 128
    }

    /// Get image raster data
    pub fn raster_data(&self) -> Result<Vec<u8>> {
        let width = self.width()?;
        let height = self.height()?;
        let mut data = Vec::new();

        for y in 0..height {
            for x in (0..width).step_by(8) {
                let mut byte = 0;

                // Processing 8 bits per byte
                for bit in 0..8 {
                    let x_offset = x + bit;

                    // Breaking the loop if x_offset exceeds the width
                    if x_offset >= width {
                        break;
                    }

                    // Shift byte to the left, adding the pixel value at the end
                    byte = (byte << 1) | u8::from(self.is_pixel_black(x_offset, y));
                }

                data.push(byte);
            }
        }

        Ok(data)
    }
}

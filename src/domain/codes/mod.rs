//! Barcodes and 2D codes

#[cfg(feature = "codes_2d")]
mod aztec;
#[cfg(feature = "barcodes")]
mod barcodes;
#[cfg(feature = "codes_2d")]
mod data_matrix;
#[cfg(feature = "codes_2d")]
mod gs1_databar_2d;
#[cfg(feature = "codes_2d")]
mod maxi_code;
#[cfg(feature = "codes_2d")]
mod pdf417;
#[cfg(feature = "codes_2d")]
mod qrcode;

#[cfg(feature = "barcodes")]
pub use barcodes::*;

#[cfg(feature = "codes_2d")]
pub use aztec::*;

#[cfg(feature = "codes_2d")]
pub use data_matrix::*;

#[cfg(feature = "codes_2d")]
pub use gs1_databar_2d::*;

#[cfg(feature = "codes_2d")]
pub use maxi_code::*;

#[cfg(feature = "codes_2d")]
pub use pdf417::*;

#[cfg(feature = "codes_2d")]
pub use qrcode::*;

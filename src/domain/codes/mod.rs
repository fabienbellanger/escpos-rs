//! Barcodes and 2D codes
mod aztec;
mod barcodes;
mod data_matrix;
mod gs1_databar_2d;
mod maxi_code;
mod pdf417;
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

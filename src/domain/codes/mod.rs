//! Barcodes and 2D codes
mod barcodes;
mod gs1_databar_2d;
mod qrcode;

#[cfg(feature = "gs1_databar_2d")]
pub use gs1_databar_2d::*;

#[cfg(feature = "qrcode")]
pub use qrcode::*;

#[cfg(feature = "barcodes")]
pub use barcodes::*;

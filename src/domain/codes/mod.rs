//! Barcodes and 2D codes
mod barcodes;
mod gs1_databar;
mod qrcode;

#[cfg(feature = "gs1_databar")]
pub use gs1_databar::*;

#[cfg(feature = "qrcode")]
pub use qrcode::*;

#[cfg(feature = "barcodes")]
pub use barcodes::*;

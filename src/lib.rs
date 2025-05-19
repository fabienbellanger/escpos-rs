#![cfg_attr(docsrs, feature(doc_cfg))]

//! # escpos - A ESCPOS implementation in Rust
//!
//! This crate implements a subset of Epson's ESC/POS protocol for thermal receipt printers.
//! It allows you to generate and print documents with basic text formatting, cutting, barcodes,
//! QR codes and raster images on a compatible printer.
//!
//! ## Examples
//! The `examples` folder contains various examples of how to use `escpos`.  
//! The [docs](https://docs.rs/escpos) (will) also provide lots of code snippets and examples.  
//! The list of all the examples can be found [here](examples/EXAMPLES.md).
//!
//! ### Simple text formatting
//!
//! ```rust
//! use escpos::printer::Printer;
//! use escpos::printer_options::PrinterOptions;
//! use escpos::utils::*;
//! use escpos::{driver::*, errors::Result};
//!
//! fn main() -> Result<()> {
//!     // env_logger::init();
//!
//!     // let driver = NetworkDriver::open("192.168.1.248", 9100, None)?;
//!     let driver = ConsoleDriver::open(true);
//!     Printer::new(driver, Protocol::default(), Some(PrinterOptions::default()))
//!         .debug_mode(Some(DebugMode::Dec))
//!         .init()?
//!         .smoothing(true)?
//!         .bold(true)?
//!         .underline(UnderlineMode::Single)?
//!         .writeln("Bold underline")?
//!         .justify(JustifyMode::CENTER)?
//!         .reverse(true)?
//!         .bold(false)?
//!         .writeln("Hello world - Reverse")?
//!         .feed()?
//!         .justify(JustifyMode::RIGHT)?
//!         .reverse(false)?
//!         .underline(UnderlineMode::None)?
//!         .size(2, 3)?
//!         .writeln("Hello world - Normal")?
//!         .print_cut()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### EAN13 (with `barcode` feature enabled)
//!
//! ```rust
//! use escpos::printer::Printer;
//! use escpos::utils::*;
//! use escpos::{driver::*, errors::Result};
//!
//! fn main() -> Result<()> {
//!     // env_logger::init();
//!
//!     // let driver = NetworkDriver::open("192.168.1.248", 9100, None)?;
//!     let driver = ConsoleDriver::open(true);
//!     Printer::new(driver, Protocol::default(), None)
//!         .debug_mode(Some(DebugMode::Hex))
//!         .init()?
//!         .ean13_option(
//!             "1234567890265",
//!             BarcodeOption::new(
//!                 BarcodeWidth::M,
//!                 BarcodeHeight::S,
//!                 BarcodeFont::A,
//!                 BarcodePosition::Below,
//!             )
//!         )?
//!         .feed()?
//!         .print_cut()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### QR Code (with `codes_2d` feature enabled)
//!
//! ```rust
//! use escpos::printer::Printer;
//! use escpos::utils::*;
//! use escpos::{driver::*, errors::Result};
//!
//! fn main() -> Result<()> {
//!     // env_logger::init();
//!
//!     // let driver = NetworkDriver::open("192.168.1.248", 9100, None)?;
//!     let driver = ConsoleDriver::open(true);
//!     Printer::new(driver, Protocol::default(), None)
//!         .debug_mode(Some(DebugMode::Hex))
//!         .init()?
//!         .qrcode_option(
//!             "https://www.google.com",
//!             QRCodeOption::new(QRCodeModel::Model1, 6, QRCodeCorrectionLevel::M),
//!         )?
//!         .feed()?
//!         .print_cut()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Bit image (with `graphics` feature enabled)
//!
//! ```rust
//! use escpos::printer::Printer;
//! use escpos::utils::*;
//! use escpos::{driver::*, errors::Result};
//!
//! fn main() -> Result<()> {
//!     // env_logger::init();
//!
//!     // let driver = NetworkDriver::open("192.168.1.248", 9100, None)?;
//!     let driver = ConsoleDriver::open(true);
//!     let mut printer = Printer::new(driver, Protocol::default(), None);
//!     printer.debug_mode(Some(DebugMode::Hex))
//!         .init()?
//!         .bit_image_option(
//!             "./resources/images/rust-logo-small.png",
//!             BitImageOption::new(Some(128), None, BitImageSize::Normal)?,
//!         )?
//!         .feed()?
//!         .print_cut()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Check printer status
//!
//! ```rust,ignore
//! use escpos::printer::Printer;
//! use escpos::utils::*;
//! use escpos::{driver::*, errors::Result};
//!
//! fn main() -> Result<()> {
//!     // env_logger::init();
//!
//!     // let driver = NetworkDriver::open("192.168.1.248", 9100, None)?;
//!     let driver = ConsoleDriver::open(true);
//!     Printer::new(driver.clone(), Protocol::default(), None)
//!         .debug_mode(Some(DebugMode::Dec))
//!         .real_time_status(RealTimeStatusRequest::Printer)?
//!         .real_time_status(RealTimeStatusRequest::RollPaperSensor)?
//!         .send_status()?;
//!
//!     let mut buf = [0; 1];
//!     driver.read(&mut buf)?;
//!
//!     let status = RealTimeStatusResponse::parse(RealTimeStatusRequest::Printer, buf[0])?;
//!     println!(
//!         "Printer online: {}",
//!         status.get(&RealTimeStatusResponse::Online).unwrap_or(&false)
//!     );
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Features list
//!
//! | Name          | Description                                                            | Default |
//! | ------------- | ---------------------------------------------------------------------- | :-----: |
//! | `barcodes`    | Print barcodes (UPC-A, UPC-E, EAN8, EAN13, CODE39, ITF or CODABAR)     |   ✅    |
//! | `codes_2d`    | Print 2D codes (QR Code, PDF417, GS1 DataBar, DataMatrix, Aztec, etc.) |   ✅    |
//! | `graphics`    | Print raster images                                                    |   ❌    |
//! | `usb`         | Enable USB feature                                                     |   ❌    |
//! | `native_usb`  | Enable native USB feature                                              |   ❌    |
//! | `hidapi`      | Enable HidApi feature                                                  |   ❌    |
//! | `serial_port` | Enable Serial port feature                                             |   ❌    |
//! | `ui`          | Enable ui feature (UI components)                                      |   ❌    |
//! | `full`        | Enable all features                                                    |   ❌    |
//!
//! ## External resources
//!
//! - [Epson documentation](https://download4.epson.biz/sec_pubs/pos/reference_en/escpos)

mod domain;

/// Error module
pub mod errors;
pub(crate) mod io;

/// Print document
pub mod printer;

/// Printer options
pub mod printer_options;

/// Utils module contains protocol and all needed constants and enums
pub mod utils {
    pub use super::domain::*;
    pub use super::io::encoder::*;
}

/// UI components like lines, tables, etc.
#[cfg(feature = "ui")]
pub mod ui {
    pub use super::domain::ui::*;
}

/// Drivers used to send data to the printer (Network or USB)
pub use io::driver;

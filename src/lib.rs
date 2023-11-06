//! escpos - A ESCPOS implementation in Rust
//!
//! ## Examples
//!
//! ```rust
//! use escpos::printer::Printer;
//! use escpos::utils::{protocol::Protocol, *};
//! use escpos::{driver::*, errors::Result};
//!
//! fn main() -> Result<()> {
//!     // env_logger::init();
//!
//!     let driver = ConsoleDriver::open();
//!     Printer::new(driver, Protocol::default())
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
//!         .write("")?
//!         .feed()?
//!         .print_cut()?;
//!
//!     Ok(())
//! }
//! ```
#![doc = include_str!("../README.md")]

mod domain;

/// Error module
pub mod errors;
pub(crate) mod io;

/// Printer module
pub mod printer;

/// Utils module contains protocol and all needed constants and enums
pub mod utils {
    pub use super::domain::*;
}

/// Drivers used to send data to the printer
pub use io::driver;

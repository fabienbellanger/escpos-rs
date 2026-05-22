//! Demonstrates how to use barcodes and 2D codes with a custom `Driver`
//! implementation. Every API used in this example is `no_std`-compatible.
//!
//! In a real `no_std` embedded context you would replace `MemoryDriver` with a
//! driver wrapping your hardware peripheral (UART, SPI, USB endpoint, ...).
//!
//! Run on the host (std target):
//!
//! ```shell
//! cargo run --example no_std_codes
//! ```
//!
//! Build the crate itself for `no_std` (lib only, this example needs std to run):
//!
//! ```shell
//! cargo build --no-default-features --features barcodes,codes_2d
//! ```

use core::cell::RefCell;
use escpos::driver::Driver;
use escpos::errors::Result;
use escpos::printer::Printer;
use escpos::utils::*;

/// In-memory driver that captures the bytes the printer would have sent on the wire.
#[derive(Default)]
struct MemoryDriver {
    buf: RefCell<Vec<u8>>,
}

impl Driver for MemoryDriver {
    fn name(&self) -> String {
        "memory".to_string()
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        self.buf.borrow_mut().extend_from_slice(data);
        Ok(())
    }

    fn read(&self, _buf: &mut [u8]) -> Result<usize> {
        Ok(0)
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    let driver = MemoryDriver::default();
    let mut printer = Printer::new(driver, Protocol::default(), None);

    printer
        .init()?
        .justify(JustifyMode::CENTER)?
        .writeln("EAN13")?
        .ean13_option(
            "1234567890265",
            BarcodeOption::new(
                BarcodeWidth::M,
                BarcodeHeight::S,
                BarcodeFont::A,
                BarcodePosition::Below,
            ),
        )?
        .writeln("QR Code")?
        .qrcode_option(
            "https://example.com",
            QRCodeOption::new(QRCodeModel::Model1, 6, QRCodeCorrectionLevel::M),
        )?
        .writeln("DataMatrix")?
        .data_matrix("test1245789658745")?
        .writeln("Aztec code")?
        .aztec("test1245789658745")?
        .feed()?
        .print_cut()?;

    let driver = printer.driver();
    let buf = driver.buf.borrow();
    println!("Captured {} bytes from the printer protocol", buf.len());
    println!("First 16 bytes: {:?}", &buf[..buf.len().min(16)]);

    Ok(())
}

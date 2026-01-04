//! Example: Printing Arabic and Hebrew text with bidirectional support
//!
//! This example demonstrates how to use the `bidi` feature to print
//! RTL (right-to-left) text correctly on ESC/POS thermal printers.
//!
//! Run with: `cargo run --example bidi --features "bidi"`

use escpos::driver::ConsoleDriver;
use escpos::errors::Result;
use escpos::printer::Printer;
use escpos::utils::*;

fn main() -> Result<()> {
    env_logger::init();

    let driver = ConsoleDriver::open(true);
    let mut printer = Printer::new(driver, Protocol::default(), None);

    printer
        .debug_mode(Some(DebugMode::Hex))
        .init()?
        // Set Arabic code page
        .page_code(PageCode::PC864)?
        .writeln("=== Arabic Text Demo ===")?
        .feed()?
        // Arabic text with automatic BiDi reordering
        .writeln_bidi("مرحبا بالعالم")?  // "Hello World" in Arabic
        .feed()?
        .writeln_bidi("السلام عليكم")?   // "Peace be upon you"
        .feed()?
        // Mixed LTR and RTL text
        .writeln_bidi("Price: 123 ريال")?  // Mixed numbers and Arabic
        .feed()?
        .feeds(2)?
        // Hebrew example (using PC862)
        .page_code(PageCode::PC862)?
        .writeln("=== Hebrew Text Demo ===")?
        .feed()?
        .writeln_bidi("שלום עולם")?  // "Hello World" in Hebrew
        .feed()?
        .print_cut()?;

    println!("\n--- Bidirectional text demo completed ---");
    println!("Note: The hex output above shows the reordered bytes sent to the printer.");
    println!("RTL text has been automatically reversed for correct visual display.");

    Ok(())
}

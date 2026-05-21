//! Example: Printing Arabic and Hebrew text with bidirectional support
//!
//! This example demonstrates how to use the `bidi` feature to print
//! RTL (right-to-left) text correctly on ESC/POS thermal printers.
//!
//! # Important caveat for Arabic
//!
//! `reorder_for_display` only reverses the logical order of characters;
//! it does **not** perform contextual shaping. Because PC864 only maps
//! Arabic Presentation Forms (U+FE70–U+FEFF) and not the base letters
//! (U+0600–U+06FF), you must feed pre-shaped strings to `write_bidi` /
//! `writeln_bidi`. Use a shaping library such as `rustybuzz` for real
//! sentences; this example uses isolated forms for simplicity.
//!
//! Hebrew has no contextual shaping in PC862, so base letters work as-is.
//!
//! Run with: `cargo run --example bidi --features "bidi"`

use escpos::driver::{ConsoleDriver, NetworkDriver};
use escpos::errors::Result;
use escpos::printer::Printer;
use escpos::utils::*;

fn main() -> Result<()> {
    env_logger::init();

    let driver = NetworkDriver::open("192.168.1.248", 9100, None)?;
    let mut printer = Printer::new(driver, Protocol::default(), None);

    // Arabic word "سلام" (salam / peace), written using isolated
    // presentation forms so every glyph is present in PC864:
    //   ﺱ (FEB1)  ﻝ (FEDD)  ﺍ (FE8D)  ﻡ (FEE1)
    let salam = "ﺱﻝﺍﻡ";

    printer
        .debug_mode(Some(DebugMode::Hex))
        .init()?
        .page_code(PageCode::PC864)?
        .writeln("=== Arabic Text Demo ===")?
        .feed()?
        .writeln_bidi(salam)?
        .feed()?
        // Mixed LTR + RTL: numbers stay LTR inside an RTL run.
        .writeln_bidi("Price: 123 ﺱﻝﺍﻡ")?
        .feeds(2)?
        .page_code(PageCode::PC862)?
        .writeln("=== Hebrew Text Demo ===")?
        .feed()?
        .writeln_bidi("שלום עולם")?
        .feed()?
        .print_cut()?;

    println!("\n--- Bidirectional text demo completed ---");
    println!("Note: the hex output shows the reordered bytes sent to the printer.");
    println!("Arabic input must use Presentation Forms (U+FE70–U+FEFF) because");
    println!("contextual shaping is not performed by this library.");

    Ok(())
}

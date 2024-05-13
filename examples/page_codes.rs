use escpos::printer::Printer;
use escpos::printer_options::PrinterOptions;
use escpos::utils::*;
use escpos::{driver::*, errors::Result};

const EURO: &[u8] = &[0xD5]; // '€' in code page PC858

fn main() -> Result<()> {
    env_logger::init();

    // let driver = NetworkDriver::open("192.168.1.248", 9100, None)?;
    let driver = ConsoleDriver::open(true);
    let printer_options = PrinterOptions::new(Some(PageCode::PC858), 42, None);
    Printer::new(driver, Protocol::default(), Some(printer_options))
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .writeln("Test with page code PC858:")?
        .writeln("€, é, à, À, Ô")?
        .feeds(2)?
        .page_code(PageCode::PC437)?
        .writeln("Test with page code PC437:")?
        .writeln("€, é, à, À, Ô")?
        .feeds(2)?
        .page_code(PageCode::ISO8859_2)?
        .writeln("Test with page code ISO8859_2:")?
        .writeln("Ś, š, ¤, À, Ô, a")?
        .feeds(2)?
        .page_code(PageCode::ISO8859_15)?
        .writeln("Test with page code ISO8859_15:")?
        .writeln("Ž, £, æ, þ, Ô, a")?
        .feeds(2)?
        .page_code(PageCode::PC858)?
        .writeln("Test with custom command:")?
        .custom(EURO)?
        .feeds(2)?
        .print_cut()?;

    Ok(())
}

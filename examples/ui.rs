use escpos::printer::Printer;
use escpos::printer_options::PrinterOptions;
use escpos::utils::*;
use escpos::{driver::*, errors::Result};

fn main() -> Result<()> {
    env_logger::init();

    // let driver = NetworkDriver::open("192.168.1.248", 9100, None)?;
    let driver = ConsoleDriver::open(true);
    let printer_options = PrinterOptions::new(Some(PageCode::PC858), Some(DebugMode::Dec), 42);
    let mut printer = Printer::new(driver, Protocol::default(), Some(printer_options));
    printer.init()?.debug()?.writeln("UI Components")?.feed()?.print_cut()?;

    Ok(())
}

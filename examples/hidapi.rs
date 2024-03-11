use escpos::driver::*;
use escpos::errors::Result;
use escpos::printer::Printer;
use escpos::utils::*;

fn main() -> Result<()> {
    env_logger::init();

    let driver = HidApiDriver::open(0x05ac, 0x0221)?;
    Printer::new(driver, Protocol::default(), None)
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .writeln("HidApi test")?
        .print_cut()?;

    Ok(())
}

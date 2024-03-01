use std::time::Duration;

use escpos::driver::*;
use escpos::errors::Result;
use escpos::printer::Printer;
use escpos::utils::*;

fn main() -> Result<()> {
    env_logger::init();

    let driver = SerialPortDriver::open("/dev/ttyUSB0", 115_200, Some(Duration::from_secs(5)))?;
    Printer::new(driver, Protocol::default(), None)
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .writeln("Serial port test")?
        .print_cut()?;

    Ok(())
}

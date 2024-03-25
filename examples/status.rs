use escpos::printer::Printer;
use escpos::utils::*;
use escpos::{driver::*, errors::Result};

fn main() -> Result<()> {
    env_logger::init();

    let driver = NetworkDriver::open("192.168.1.248", 9100)?;
    // let driver = ConsoleDriver::open(true);
    Printer::new(driver.clone(), Protocol::default(), None)
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .writeln("coucou")?
        .real_time_status()?
        .print_cut()?;

    let mut buf = [0; 1];
    driver.read(&mut buf)?;
    for (i, b) in buf.iter().enumerate() {
        println!(
            "Buffer {i}: {b:08b} => {:?}",
            RealTimeStatus::to_str(&RealTimeStatus::Printer, *b)
        );
    }

    Ok(())
}

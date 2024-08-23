use escpos::printer::Printer;
use escpos::utils::*;
use escpos::{driver::*, errors::Result};

fn main() -> Result<()> {
    env_logger::init();

    // Picture from URL
    let picture_bytes = reqwest::blocking::get(
        "https://github.com/fabienbellanger/escpos-rs/blob/main/resources/images/rustacean-flat-happy.png?raw=true",
    )
    .unwrap()
    .bytes()
    .unwrap();

    // let driver = NetworkDriver::open("192.168.1.248", 9100, None)?;
    let driver = ConsoleDriver::open(true);
    let mut printer = Printer::new(driver, Protocol::default(), None);
    printer
        .init()?
        .justify(JustifyMode::CENTER)?
        .bit_image("./resources/images/rust-logo-small.png")?
        .feed()?
        .bit_image_from_bytes_option(
            &picture_bytes,
            BitImageOption::new(Some(128), None, BitImageSize::Normal)?,
        )?
        .feed()?
        .print_cut()?;

    Ok(())
}

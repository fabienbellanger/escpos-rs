use escpos::printer::Printer;
use escpos::utils::*;
use escpos::{driver::*, errors::Result};

fn main() -> Result<()> {
    env_logger::init();

    let driver = NetworkDriver::open("192.168.1.248", 9100)?;
    // let driver = ConsoleDriver::open(true);
    let mut printer = Printer::new(driver, Protocol::default());
    printer
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .smoothing(true)?
        .page_code(PageCode::default())?
        .bold(true)?
        .underline(UnderlineMode::Single)?
        .writeln("Bold underline")?
        .justify(JustifyMode::CENTER)?
        .reverse(true)?
        .bold(false)?
        .writeln("Hello world - Reverse")?
        .feed()?
        .justify(JustifyMode::RIGHT)?
        .reverse(false)?
        .underline(UnderlineMode::None)?
        .size(2, 3)?
        .writeln("Hello world - Normal")?
        .write("")?
        .justify(JustifyMode::CENTER)?
        .ean13_option(
            "1234567890265",
            BarcodeOption::new("M", "S", "A", BarcodePosition::Below),
        )?
        .feed()?
        .qrcode_option(
            "https://www.google.com",
            QRCodeOption {
                model: QRCodeModel::Model1,
                size: 6,
                correction_level: QRCodeCorrectionLevel::M,
            },
        )?
        .feed()?
        .bit_image_option(
            "./resources/images/rust-logo-small.png",
            BitImageOption::new(Some(128), None, BitImageSize::Normal)?,
        )?
        .feed()?
        .print_cut()?;

    Ok(())
}

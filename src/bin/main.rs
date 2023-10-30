use escpos::printer::Printer;
use escpos::utils::{protocol::Protocol, *};
use escpos::{driver::*, errors::Result};

fn main() -> Result<()> {
    env_logger::init();

    // let driver = ConsoleDriver::open();
    let driver = NetworkDriver::open("192.168.1.248", 9100)?;
    Printer::new(driver, Protocol::default())
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .smoothing(true)?
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
        .qrcode_option(
            "https://www.google.com",
            QRCodeOption {
                model: QRCodeModel::Model1,
                size: 6,
                correction_level: QRCodeCorrectionLevel::M,
            },
        )?
        .write("")?
        .feed()?
        //.debug()
        .print_cut()?;

    Ok(())
}

use escpos::printer::Printer;
use escpos::ui::line::{LineBuilder, LineStyle};
use escpos::utils::*;
use escpos::{driver::*, errors::Result};

fn main() -> Result<()> {
    env_logger::init();
    let repo_root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string());

    // let driver = NetworkDriver::open("192.168.1.248", 9100, None)?;
    let driver = ConsoleDriver::open(true);
    let mut printer = Printer::new(driver, Protocol::default(), None);
    printer
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .smoothing(true)?
        .page_code(PageCode::default())?
        .bold(true)?
        .underline(UnderlineMode::Single)?
        .writeln("Bold underline left")?
        .justify(JustifyMode::CENTER)?
        .reverse(true)?
        .bold(false)?
        .writeln("Normal reverse center")?
        .feed()?
        .justify(JustifyMode::RIGHT)?
        .reverse(false)?
        .underline(UnderlineMode::None)?
        .draw_line(LineBuilder::new().style(LineStyle::Double).build())?
        .feed()?
        .size(2, 3)?
        .writeln("Bigger right")?
        .writeln("")?
        .justify(JustifyMode::CENTER)?
        .ean13_option(
            "1234567890265",
            BarcodeOption::new(
                BarcodeWidth::M,
                BarcodeHeight::S,
                BarcodeFont::A,
                BarcodePosition::Below,
            ),
        )?
        .feed()?
        .qrcode_option(
            "https://www.google.com",
            QRCodeOption::new(QRCodeModel::Model1, 6, QRCodeCorrectionLevel::M),
        )?
        .feed()?
        .gs1_databar_2d_option(
            "8245789658745",
            GS1DataBar2DOption::new(GS1DataBar2DWidth::S, GS1DataBar2DType::Stacked),
        )?
        .feed()?
        .pdf417("8245789658745")?
        .feed()?
        .maxi_code("1245789658745")?
        .feed()?
        .data_matrix("test1245789658745")?
        .feed()?
        .aztec("test1245789658745")?
        .feed()?
        .bit_image_option(
            &(repo_root_dir + "/resources/images/rust-logo-small.png"),
            BitImageOption::new(Some(128), None, BitImageSize::Normal)?,
        )?
        .feed()?
        .print_cut()?;

    Ok(())
}

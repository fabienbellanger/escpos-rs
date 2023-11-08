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
        .gs1_databar_2d("8245789658745")?
        .gs1_databar_2d_option(
            "8245789658745",
            GS1DataBar2DOption::new(GS1DataBar2DWidth::M, GS1DataBar2DType::Stacked),
        )?
        .feed()?
        .gs1_databar_2d_option(
            "1245789658745",
            GS1DataBar2DOption::new(GS1DataBar2DWidth::L, GS1DataBar2DType::StackedOmnidirectional),
        )?
        .feed()?
        .gs1_databar_2d_option(
            "1245789658745AC!4545A5151C12457896",
            GS1DataBar2DOption::new(GS1DataBar2DWidth::S, GS1DataBar2DType::ExpandedStacked),
        )?
        .feed()?
        .print_cut()?;

    Ok(())
}

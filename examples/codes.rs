use escpos::printer::Printer;
use escpos::utils::*;
use escpos::{driver::*, errors::Result};

fn main() -> Result<()> {
    env_logger::init();

    // let driver = NetworkDriver::open("192.168.1.248", 9100)?;
    let driver = ConsoleDriver::open(true);
    let mut printer = Printer::new(driver, Protocol::default());
    printer
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .smoothing(true)?
        .justify(JustifyMode::CENTER)?
        // EAN13
        .writeln("EAN13")?
        .ean13_option(
            "1234567890265",
            BarcodeOption::new(
                BarcodeWidth::M,
                BarcodeHeight::S,
                BarcodeFont::A,
                BarcodePosition::Below,
            ),
        )?
        // QR Code
        .writeln("QR Code")?
        .qrcode_option(
            "https://www.google.com",
            QRCodeOption::new(QRCodeModel::Model1, 6, QRCodeCorrectionLevel::M),
        )?
        // GS1 DataBar
        .writeln("GS1 DataBar Expanded")?
        .gs1_databar_2d("8245789658745")?
        .writeln("GS1 DataBar ExpandedStacked")?
        .gs1_databar_2d_option(
            "1245789658745",
            GS1DataBar2DOption::new(GS1DataBar2DWidth::L, GS1DataBar2DType::StackedOmnidirectional),
        )?
        .writeln("GS1 DataBar StackedOmnidirectional")?
        .gs1_databar_2d_option(
            "1245789658745AC!4545A5151C12457896",
            GS1DataBar2DOption::new(GS1DataBar2DWidth::S, GS1DataBar2DType::ExpandedStacked),
        )?
        // PDF417
        .writeln("PDF417")?
        .pdf417_option(
            "1245789658745",
            Pdf417Option::new(16, 16, 4, 2, Pdf417Type::Standard, Pdf417CorrectionLevel::Ratio(32))?,
        )?
        // MaxiCode
        .writeln("MaxiCode")?
        .maxi_code_option("1245789658745", MaxiCodeMode::Mode2)?
        // DataMatrix
        .writeln("DataMatrix")?
        .data_matrix("test1245789658745")?
        // Aztec code
        .writeln("Aztec code")?
        .aztec("test1245789658745")?
        .feed()?
        .print_cut()?;

    Ok(())
}

use escpos::printer::Printer;
use escpos::printer_options::PrinterOptions;
use escpos::utils::ui::line::{LineBuilder, LineStyle};
use escpos::utils::*;
use escpos::{driver::*, errors::Result};

fn main() -> Result<()> {
    env_logger::init();

    // Line
    let line_double = LineBuilder::new().style(LineStyle::Double).build();
    let line_simple = LineBuilder::new().style(LineStyle::Simple).offset(4).build();
    let line_dotted = LineBuilder::new().style(LineStyle::Dotted).offset(8).build();
    let line_dashed = LineBuilder::new()
        .style(LineStyle::Dashed)
        .align(JustifyMode::CENTER)
        .size((2, 1))
        .width(8)
        .build();
    let line_custom = LineBuilder::new().style(LineStyle::Custom("=-")).build();
    // let line_custom_utf8 = LineBuilder::new().style(LineStyle::Custom("😀")).width(8).build();

    let driver = NetworkDriver::open("192.168.1.248", 9100, None)?;
    // let driver = ConsoleDriver::open(true);
    let printer_options = PrinterOptions::new(Some(PageCode::PC858), Some(DebugMode::Dec), 42);
    let mut printer = Printer::new(driver, Protocol::default(), Some(printer_options));
    printer
        .init()?
        .debug()?
        .writeln("UI Components")?
        .feed()?
        .writeln("Lines")?
        .draw_line(line_double)?
        .draw_line(line_simple)?
        .draw_line(line_dashed)?
        .draw_line(line_custom)?
        .draw_line(line_dotted)?
        // .draw_line(line_custom_utf8)?
        .print_cut()?;

    Ok(())
}

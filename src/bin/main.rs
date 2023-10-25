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
        .text("Hello world - Bold underline")?
        .feed()?
        .justify(JustifyMode::CENTER)?
        .reverse(true)?
        .bold(false)?
        .text("Hello world - Reverse")?
        .feeds(2)?
        .justify(JustifyMode::RIGHT)?
        .reverse(false)?
        .underline(UnderlineMode::None)?
        .text_size(2, 3)?
        .text("Hello world - Normal")?
        .feed()?
        .text("")?
        .text("")?
        .feed()?
        //.debug()
        .print_cut()?;

    Ok(())
}

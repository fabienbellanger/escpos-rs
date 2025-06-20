use escpos::printer::Printer;
use escpos::ui::line::{LineBuilder, LineStyle};
use escpos::utils::*;
use escpos::{driver::*, errors::Result};

const CHARS_BY_LINE: usize = 42;
const EURO: &[u8] = &[0xD5]; // €
const NUM: &[u8] = &[0xF8]; // °

fn main() -> Result<()> {
    env_logger::init();
    let repo_root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string());

    let items = vec![
        Item::new("Macbook Pro", Some(1), 2500.00, false),
        Item::new("Macbook Air", Some(5), 1500.00, false),
        Item::new("iMac", Some(2), 3000.00, false),
        Item::new("AirPods", Some(1), 200.00, false),
        Item::new("iPhone", Some(1), 1000.00, false),
        Item::new("iPad", Some(3), 800.00, false),
        Item::new("Apple Watch", Some(1), 400.00, false),
    ];

    let subtotal = Item::new(
        "Subtotal",
        None,
        items.iter().fold(0.0, |acc, item| acc + item.price),
        true,
    );
    let tax = Item::new("Tax (20%)", None, subtotal.price * 0.20, true);
    let total = Item::new("Total", None, subtotal.price + tax.price, true);

    // let driver = NetworkDriver::open("192.168.1.248", 9100, None)?;
    let driver = ConsoleDriver::open(true);
    let mut printer = Printer::new(driver, Protocol::default(), None);
    printer.init()?.justify(JustifyMode::CENTER)?;

    // Logo
    #[cfg(feature = "graphics")]
    printer.bit_image(&(repo_root_dir + "/resources/images/rust-logo-small.png"))?;

    // Line
    let simple_line = LineBuilder::new().style(LineStyle::Simple).build();
    let double_line = LineBuilder::new().style(LineStyle::Double).build();

    // Name + address
    printer
        .bold(true)?
        .size(2, 2)?
        .writeln("My Shop")?
        .reset_size()?
        .bold(false)?
        .writeln("1, rue des Gloutons")?
        .writeln("75000 Paris")?
        .feed()?
        .justify(JustifyMode::LEFT)?
        .writeln("2023-11-13 13:22")?
        .draw_line(simple_line.clone())?
        .write("Ticket n")?
        .custom_with_page_code(NUM, PageCode::PC858)?
        .size(2, 2)?
        .writeln("23")?
        .reset_size()?
        .draw_line(simple_line)?;

    // Items
    for item in items {
        item.print(&mut printer, 1)?;
    }

    // Total
    printer.draw_line(double_line)?;
    subtotal.print(&mut printer, 1)?;
    tax.print(&mut printer, 1)?;
    printer.size(2, 2)?;
    total.print(&mut printer, 2)?;
    printer.reset_size()?;

    printer.print_cut()?;

    Ok(())
}

#[derive(Clone)]
struct Item {
    name: String,
    quantity: Option<u8>,
    price: f32,
    symbol: bool,
}

impl Item {
    fn new(name: &str, quantity: Option<u8>, price: f32, symbol: bool) -> Item {
        let name = name.to_string();
        Item {
            name,
            quantity,
            price,
            symbol,
        }
    }

    fn print<D: Driver>(&self, printer: &mut Printer<D>, size: u8) -> Result<()> {
        // Length of characters
        let mut characters_length = self.name.len() + self.price.to_string().len() + 3;
        if self.quantity.is_some() {
            characters_length += 2;
        }
        if self.symbol {
            characters_length += 2;
        }
        characters_length *= size as usize;

        // Number of spaces between name and price
        let spaces = " ".repeat((CHARS_BY_LINE - characters_length) / size as usize);

        // Print item
        if let Some(quantity) = self.quantity {
            printer.write(&format!("{} ", quantity))?;
        }
        printer.write(&format!("{}{}{:.2}", self.name, spaces, self.price))?;
        if self.symbol {
            printer.write(" ")?;
            printer.custom_with_page_code(EURO, PageCode::PC858)?;
        }
        printer.feed()?;

        Ok(())
    }
}

use escpos::printer::Printer;
use escpos::utils::*;
use escpos::{driver::*, errors::Result};

const CHARS_BY_LINE: usize = 42;
const EURO: &[u8] = &[0xD5]; // €
const NUM: &[u8] = &[0xF8]; // °

fn main() -> Result<()> {
    env_logger::init();

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

    let driver = NetworkDriver::open("192.168.1.248", 9100)?;
    // let driver = ConsoleDriver::open(true);
    let mut printer = Printer::new(driver, Protocol::default());

    printer
        .init()?
        .page_code(PageCode::PC437)?
        .justify(JustifyMode::CENTER)?;

    // Logo
    #[cfg(feature = "graphics")]
    printer.bit_image("./resources/images/rust-logo-small.png")?;

    // Name + address
    printer
        .bold(true)?
        .size(2, 2)?
        .writeln("My Shop")?
        .reset_size()?
        .bold(false)?
        .writeln("1, rue des gloutons")?
        .writeln("75000 Paris")?
        .feed()?
        .justify(JustifyMode::LEFT)?
        .writeln("2023-11-13 13:22")?
        .writeln("-".repeat(42).as_str())?
        .write("Ticket n")?
        .custom_with_page_code(NUM, PageCode::PC858)?
        .size(2, 2)?
        .writeln("23")?
        .reset_size()?
        .writeln("-".repeat(42).as_str())?;

    // Items
    for item in items {
        let label: String = item.clone().into();
        printer.writeln(&label)?;
    }

    // Total
    let subtotal: String = subtotal.into();
    let tax: String = tax.into();
    let total: String = total.into();
    printer.writeln("-".repeat(42).as_str())?;
    printer.bold(true)?;
    printer.write(&subtotal)?;
    printer.custom_with_page_code(EURO, PageCode::PC858)?;
    printer.feed()?;
    printer.write(&tax)?;
    printer.custom_with_page_code(EURO, PageCode::PC858)?;
    printer.feed()?;
    printer.write(&total)?;
    printer.custom_with_page_code(EURO, PageCode::PC858)?;
    printer.feed()?;

    printer.print_cut()?;

    Ok(())
}

#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub quantity: Option<u8>,
    pub price: f32,
    pub symbol: bool,
}

impl Item {
    pub fn new(name: &str, quantity: Option<u8>, price: f32, symbol: bool) -> Item {
        let name = name.to_string();
        Item {
            name,
            quantity,
            price,
            symbol,
        }
    }
}

impl From<Item> for String {
    fn from(item: Item) -> Self {
        let right_cols = CHARS_BY_LINE / 4;
        let left_cols = if item.quantity.is_some() {
            CHARS_BY_LINE - right_cols - 2
        } else {
            CHARS_BY_LINE - right_cols
        };
        let right_cols = if item.symbol { right_cols - 2 } else { right_cols };

        let qty = if let Some(quantity) = item.quantity {
            format!("{} ", quantity)
        } else {
            String::new()
        };
        let left = format!("{: <width$}", item.name, width = left_cols);
        let right = if item.symbol {
            format!("{: >width$.2} ", item.price, width = right_cols)
        } else {
            format!("{: >width$.2}", item.price, width = right_cols)
        };

        format!("{}{}{}", qty, left, right)
    }
}

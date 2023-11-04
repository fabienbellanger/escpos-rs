# ESC/POS Rust implementation

[![Build status](https://github.com/fabienbellanger/escpos-rs/actions/workflows/CI.yml/badge.svg?branch=main)](https://github.com/fabienbellanger/escpos-rs/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/escpos)](https://crates.io/crates/escpos)
[![Documentation](https://docs.rs/escpos/badge.svg)](https://docs.rs/escpos)

This project implements a subset of Epson's ESC/POS protocol for thermal receipt printers. It allows you to generate and print receipts with basic formatting, cutting, barcodes, QR codes and image on a compatible printer.

It is strongly inspired by [recibo](https://github.com/jamhall/recibo/tree/main) _(Rust)_, [escposify](https://github.com/local-group/rust-escposify) _(Rust)_ and [escpos](https://github.com/hennedo/escpos) _(Go)_.

## Installation

For standard functionalities (e.g. printing text), no additional dependencies are required:

```toml
[dependencies]
escpos = "0.1.0"
```

If you would like to raster images, you will need to enable the image feature:

```toml
[dependencies]
escpos = { version = "0.1.0", features = ["graphics"] }
```

If you need all features, you can use the `full` feature:

```toml
[dependencies]
escpos = { version = "0.1.0", features = ["full"] }
```

Or you can use `cargo add` command:

```bash
cargo add escpos
cargo add escpos -F barcode qrcode
cargo add escpos -F full
```

## Features

| Name     | Description                               |
| -------- | ----------------------------------------- |
| barcode  | Print barcodes like EAN8, EAN13 or CODE39 |
| qrcode   | Print QR codes                            |
| graphics | Print images                              |
| full     | Enable all features                       |

## Examples

The examples folder contains various examples of how to use `escpos`. The [docs](https://docs.rs/escpos) (will) also provide lots of code snippets and examples.

To launch an example, use the following command:

```bash
cargo run --example full --features "full"
```

### Simple text formatting

```rust
use escpos::printer::Printer;
use escpos::utils::{protocol::Protocol, *};
use escpos::{driver::*, errors::Result};

fn main() -> Result<()> {
    env_logger::init();

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
        .print_cut()?;

    Ok(())
}
```

### EAN13 (with `barcode` feature)

```rust
use escpos::printer::Printer;
use escpos::utils::{protocol::Protocol, *};
use escpos::{driver::*, errors::Result};

fn main() -> Result<()> {
    env_logger::init();

    let driver = ConsoleDriver::open();
    Printer::new(driver, Protocol::default())
        .debug_mode(Some(DebugMode::Hex))
        .init()?
        .ean13_option(
            "1234567890265",
            BarcodeOption::new("M", "S", "A", BarcodePosition::Below),
        )?
        .feed()?
        .print_cut()?;

    Ok(())
}
```

### QR Code (with `qrcode` feature)

```rust
use escpos::printer::Printer;
use escpos::utils::{protocol::Protocol, *};
use escpos::{driver::*, errors::Result};

fn main() -> Result<()> {
    env_logger::init();

    let driver = ConsoleDriver::open();
    Printer::new(driver, Protocol::default())
        .debug_mode(Some(DebugMode::Hex))
        .init()?
        .qrcode_option(
            "https://www.google.com",
            QRCodeOption {
                model: QRCodeModel::Model1,
                size: 6,
                correction_level: QRCodeCorrectionLevel::M,
            },
        )?
        .feed()?
        .print_cut()?;

    Ok(())
}
```

### Bit image (with `graphics` feature)

```rust
use escpos::printer::Printer;
use escpos::utils::{protocol::Protocol, *};
use escpos::{driver::*, errors::Result};

fn main() -> Result<()> {
    env_logger::init();

    let driver = ConsoleDriver::open();
    Printer::new(driver, Protocol::default())
        .debug_mode(Some(DebugMode::Hex))
        .init()?
        .bit_image_option(
            "./resources/images/rust-logo-small.png",
            BitImageOption::new(Some(128), None, BitImageSize::Normal)?,
        )?
        .feed()?
        .print_cut()?;

    Ok(())
}
```

## Implemented commands

| Command | Descrption |
| ------- | ---------- |
| TODO    | TODO       |

## External resources

- [Epson documentation](https://download4.epson.biz/sec_pubs/pos/reference_en/escpos/ref_escpos_en/introduction.html)

## Todo

- [ ] Complete `README.md`
- [ ] Complete documentation
- [ ] Add examples
- [ ] Add other graphic commands (Ex.: `GS 8 L`)

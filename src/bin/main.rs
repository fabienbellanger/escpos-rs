use escpos::errors::Result;
use escpos::protocol::*;
use std::{io::Write, net::TcpStream};

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("192.168.1.248:9100")?;

    let protocol = Protocol::default();
    let instructions: Vec<Vec<u8>> = vec![
        // Initialisation
        protocol.init(),
        protocol.smoothing(true),
        // Bold
        protocol.bold(true),
        protocol.underline(UnderlineMode::Single),
        protocol.print("Hello world - Bold")?,
        protocol.feed(1),
        // Reverse
        protocol.justify(JustifyMode::CENTER),
        protocol.reverse_colours(true),
        protocol.print("Hello world - reverse")?,
        protocol.feed(2),
        // Normal
        protocol.justify(JustifyMode::RIGHT),
        protocol.reverse_colours(false),
        protocol.bold(false),
        protocol.underline(UnderlineMode::None),
        protocol.text_size(2, 3)?,
        protocol.print("Hello world - Normal")?,
        protocol.feed(1),
        // Cut
        protocol.print("")?,
        protocol.print("")?,
        protocol.feed(1),
        protocol.cut(false),
    ];
    println!("Instructions={:?}", &instructions);

    for instruction in instructions.into_iter() {
        stream.write(&instruction)?;
    }
    stream.flush()?;

    Ok(())
}

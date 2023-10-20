use escpos::domain::constants::*;
use std::{error::Error, io::Write, net::TcpStream};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("192.168.1.248:9100")?;

    let instructions: Vec<&[u8]> = vec![
        // Initialisation
        ESC_INIT,
        // Bold
        ESC_TEXT_EMPHASIS_ON,
        ESC_TEXT_UNDERLINE_SIMPLE,
        "Hello world - Bold".as_bytes(),
        &[LF],
        // Normal
        ESC_TEXT_EMPHASIS_OFF,
        ESC_TEXT_UNDERLINE_NONE,
        "Hello world - Normal".as_bytes(),
        &[LF],
        // Cut
        GS_PAPER_CUT_FULL,
    ];
    println!("Instructions={:?}", &instructions);

    for instruction in instructions.into_iter() {
        stream.write(instruction)?;
    }
    stream.flush()?;

    Ok(())
}

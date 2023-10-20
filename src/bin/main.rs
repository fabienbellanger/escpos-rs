use escpos::domain::constantes::*;
use std::{io::Write, net::TcpStream};

fn main() {
    println!("ESCPOS lib");

    let mut _instructions: Vec<&[u8]> = vec![];
    let mut stream = TcpStream::connect("192.168.1.248:9100").unwrap();

    // Initialisation
    stream.write(&[ESC, '@' as u8]).unwrap();

    // Bold
    stream.write(&[ESC, 'E' as u8, 1]).unwrap();
    stream.write("Hello world - Bold".as_bytes()).unwrap();
    stream.write(&[LF]).unwrap();

    // Normal
    stream.write(&[ESC, 'E' as u8, 0]).unwrap();
    stream.write("Hello world - Normal".as_bytes()).unwrap();
    stream.write(&[LF]).unwrap();

    // Cut
    stream.write(&[GS, 'V' as u8, 'A' as u8, NIL]).unwrap();

    // Print
    stream.flush().unwrap();
}

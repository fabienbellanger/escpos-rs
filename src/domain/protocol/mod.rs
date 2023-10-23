//! Protocol used to communicate with the printer

pub mod text;

use crate::{
    constants::{ESC_INIT, GS_PAPER_CUT_FULL, GS_PAPER_CUT_PARTIAL},
    encoder::Encoder,
    errors::Result,
};

// #[derive(Debug)]
// pub struct Instruction {
//     pub name: String,
//     pub command: Vec<u8>,
// }

// impl Instruction {
//     /// Create a new instruction
//     pub fn new(name: &str, command: &[u8]) -> Self {
//         Self {
//             name: name.to_string(),
//             command: command.to_vec(),
//         }
//     }
// }

pub type Command = Vec<u8>;

pub struct Protocol {
    encoder: Encoder,
}

impl Protocol {
    /// Create new protocol
    pub fn new(encoder: Encoder) -> Self {
        Self { encoder }
    }

    /// Initialization
    pub fn init(&self) -> Command {
        ESC_INIT.to_vec()
    }

    /// Paper cut
    pub fn cut(&self, partial: bool) -> Command {
        match partial {
            true => GS_PAPER_CUT_PARTIAL.to_vec(),
            false => GS_PAPER_CUT_FULL.to_vec(),
        }
    }

    /// Print text
    pub fn print(&self, text: &str) -> Result<Command> {
        self.encoder.encode(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.init(), vec![27, 64]);
    }

    #[test]
    fn test_cut() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.cut(false), vec![29, 86, 65, 0]);
        assert_eq!(protocol.cut(true), vec![29, 86, 65, 1]);
    }

    #[test]
    fn test_print() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.print("My text").unwrap(), "My text".as_bytes());
    }
}

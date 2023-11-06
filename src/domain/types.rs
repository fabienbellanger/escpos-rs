//! Types

use std::fmt;

/// Cash drawer pin
#[derive(Debug)]
pub enum CashDrawer {
    Pin2,
    Pin5,
}

impl fmt::Display for CashDrawer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CashDrawer::Pin2 => write!(f, "cash drawer pin 2"),
            CashDrawer::Pin5 => write!(f, "cash drawer pin 5"),
        }
    }
}

/// Underline mode
#[derive(Debug)]
pub enum UnderlineMode {
    None,
    Single,
    Double,
}

impl fmt::Display for UnderlineMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnderlineMode::None => write!(f, "none"),
            UnderlineMode::Single => write!(f, "single"),
            UnderlineMode::Double => write!(f, "double"),
        }
    }
}

/// Text font
#[derive(Debug)]
pub enum Font {
    A,
    B,
    C,
}

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Font::A => write!(f, "font A"),
            Font::B => write!(f, "font B"),
            Font::C => write!(f, "font C"),
        }
    }
}

/// Justify mode
#[derive(Debug)]
pub enum JustifyMode {
    LEFT,
    CENTER,
    RIGHT,
}

impl fmt::Display for JustifyMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JustifyMode::LEFT => write!(f, "Text justify left"),
            JustifyMode::CENTER => write!(f, "Text justify center"),
            JustifyMode::RIGHT => write!(f, "Text justify right"),
        }
    }
}

/// Debug mode (decimal or hexadecimal)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebugMode {
    Hex,
    Dec,
}

/// ESC command
pub(crate) type Command = Vec<u8>;

/// Instruction
#[derive(Clone, PartialEq)]
pub(crate) struct Instruction {
    pub(crate) name: String,
    pub(crate) command: Command,
    pub(crate) debug_mode: Option<DebugMode>,
}

impl Instruction {
    pub(crate) fn new(name: &str, cmd: &[u8], debug_mode: Option<DebugMode>) -> Self {
        Instruction {
            name: name.to_string(),
            command: cmd.to_vec(),
            debug_mode,
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.debug_mode {
            Some(DebugMode::Dec) => write!(f, "{} {:?}", &self.name, &self.command),
            Some(DebugMode::Hex) => write!(f, "{} {:02X?}", &self.name, &self.command),
            None => Ok(()),
        }
    }
}

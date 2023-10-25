//! Types

use std::fmt;

#[derive(Debug)]
pub enum CashDrawer {
    Pin2 = 0,
    Pin5 = 1,
}

impl fmt::Display for CashDrawer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CashDrawer::Pin2 => write!(f, "cash drawer pin 2"),
            CashDrawer::Pin5 => write!(f, "cash drawer pin 5"),
        }
    }
}

#[derive(Debug)]
pub enum UnderlineMode {
    None = 0,
    Single = 1,
    Double = 2,
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

#[derive(Debug)]
pub enum JustifyMode {
    LEFT = 0,
    CENTER = 1,
    RIGHT = 2,
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

#[derive(Debug, Clone, Copy)]
pub enum DebugMode {
    Hex,
    Dec,
    Char,
}

/// ESC command
pub(crate) type Command = Vec<u8>;

#[derive(Clone)]
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

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Manage DebugMode
        match self.debug_mode {
            _ => write!(f, "[{}] {:?}", &self.name, &self.command),
        }
    }
}

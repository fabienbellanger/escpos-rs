//! Font control

use std::fmt;

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

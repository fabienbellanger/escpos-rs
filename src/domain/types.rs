//! Types

use std::fmt;

/// Cash drawer pin
#[derive(Debug, Clone, Copy)]
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

/// Justify mode
#[derive(Debug, Clone, Copy)]
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
    pub(crate) commands: Vec<Command>,
    pub(crate) debug_mode: Option<DebugMode>,
}

impl Instruction {
    /// Create a new instruction
    pub(crate) fn new(name: &str, commands: &[Command], debug_mode: Option<DebugMode>) -> Self {
        Instruction {
            name: name.to_string(),
            commands: commands.to_vec(),
            debug_mode,
        }
    }

    /// Get list of commands in the same Vec (flat)
    pub(crate) fn flatten_commands(&self) -> Vec<u8> {
        self.commands.iter().flatten().cloned().collect()
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.debug_mode {
            Some(DebugMode::Dec) => write!(f, "{} {:?}", &self.name, &self.commands),
            Some(DebugMode::Hex) => write!(f, "{} {:02X?}", &self.name, &self.commands),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_flatten_commands() {
        let instruction = Instruction::new("test", &[vec![29, 119, 4], vec![29, 104, 4]], None);

        assert_eq!(instruction.flatten_commands(), vec![29, 119, 4, 29, 104, 4]);
    }
}

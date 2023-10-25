//! Printer

use super::errors::Result;
use crate::{domain::*, driver::Driver, utils::protocol::Protocol};
use log::debug;

pub struct Printer<D: Driver> {
    debug_mode: Option<DebugMode>,
    driver: D,
    protocol: Protocol,
    instructions: Vec<Instruction>,
}

impl<D: Driver> Printer<D> {
    /// Create a new `Printer`
    pub fn new(driver: D, protocol: Protocol) -> Self {
        Self {
            debug_mode: None,
            driver,
            protocol,
            instructions: vec![],
        }
    }

    /// Set debug mode
    pub fn debug_mode(mut self, mode: Option<DebugMode>) -> Self {
        self.debug_mode = mode;
        self
    }

    /// Add command to instrcutions, write data and display debug information
    // TODO: Add unit test
    fn command(mut self, label: &str, cmd: Command) -> Result<Self> {
        let instruction = Instruction::new(label, &cmd, self.debug_mode);

        match self.debug_mode {
            Some(DebugMode::Dec) => debug!("{:?}", instruction.clone()),
            _ => (),
        }

        self.instructions.push(instruction);
        self.driver.write(&cmd)?;

        Ok(self)
    }

    /// Display logs of instructions if debug mode is enabled
    pub fn debug(self) -> Self {
        match self.debug_mode {
            Some(DebugMode::Dec) => debug!("[debug] instructions={:?}", self.instructions),
            _ => (),
        }

        self
    }

    /// Print
    pub fn print(mut self) -> Result<Self> {
        self.driver.flush()?;
        self.instructions = vec![];

        if self.debug_mode.is_some() {
            debug!("[print]");
        }

        Ok(self)
    }

    /// Hardware initialization
    pub fn init(self) -> Result<Self> {
        let cmd = self.protocol.init();
        self.command("initialization", cmd)
    }

    /// Hardware reset
    pub fn reset(self) -> Result<Self> {
        let cmd = self.protocol.reset();
        self.command("reset", cmd)
    }

    /// Paper full cut
    pub fn cut(self) -> Result<Self> {
        let cmd = self.protocol.cut(false);
        self.command("full paper cut", cmd)
    }

    /// Paper partial cut
    pub fn partial_cut(self) -> Result<Self> {
        let cmd = self.protocol.cut(true);
        self.command("partial paper cut", cmd)
    }

    /// Print and paper full cut
    pub fn print_cut(self) -> Result<Self> {
        let cmd = self.protocol.cut(false);
        self.command("full paper cut", cmd)?.print()
    }

    /// Text bold
    pub fn bold(self, enabled: bool) -> Result<Self> {
        let cmd = self.protocol.bold(enabled);
        self.command("text bold", cmd)
    }

    /// Text underline
    pub fn underline(self, mode: UnderlineMode) -> Result<Self> {
        let cmd = self.protocol.underline(mode);
        self.command("text underline", cmd)
    }

    /// Text double strike
    pub fn double_strike(self, enabled: bool) -> Result<Self> {
        let cmd = self.protocol.double_strike(enabled);
        self.command("text double strike", cmd)
    }

    /// Text font
    pub fn font(self, font: Font) -> Result<Self> {
        let cmd = self.protocol.font(font);
        self.command("text font", cmd)
    }

    /// Text flip
    pub fn flip(self, enabled: bool) -> Result<Self> {
        let cmd = self.protocol.flip(enabled);
        self.command("text flip", cmd)
    }

    /// Text justify
    pub fn justify(self, mode: JustifyMode) -> Result<Self> {
        let cmd = self.protocol.justify(mode);
        self.command("text justify", cmd)
    }

    /// Text reverse colour
    pub fn reverse(self, enabled: bool) -> Result<Self> {
        let cmd = self.protocol.reverse_colours(enabled);
        self.command("text reverse colour", cmd)
    }

    /// Text size
    pub fn text_size(self, width: u8, height: u8) -> Result<Self> {
        let cmd = self.protocol.text_size(width, height)?;
        self.command("text size", cmd)
    }

    /// Smoothing mode
    pub fn smoothing(self, enabled: bool) -> Result<Self> {
        let cmd = self.protocol.smoothing(enabled);
        self.command("smoothing mode", cmd)
    }

    /// Line feed
    pub fn feed(self) -> Result<Self> {
        let cmd = self.protocol.feed(1);
        self.command("line feed", cmd)
    }

    /// Custom line feed
    pub fn feeds(self, lines: u8) -> Result<Self> {
        let cmd = self.protocol.feed(lines);
        self.command("line feeds", cmd)
    }

    /// Line spacing
    pub fn line_spacing(self, value: u8) -> Result<Self> {
        let cmd = self.protocol.line_spacing(value);
        self.command("line spacing", cmd)
    }

    /// Reset line spacing
    pub fn reset_line_spacing(self) -> Result<Self> {
        let cmd = self.protocol.reset_line_spacing();
        self.command("reset line spacing", cmd)
    }

    /// Upside-down mode
    pub fn upside_down(self, enabled: bool) -> Result<Self> {
        let cmd = self.protocol.upside_down(enabled);
        self.command("upside-down mode", cmd)
    }

    /// Cash drawer
    pub fn cash_drawer(self, pin: CashDrawer) -> Result<Self> {
        let cmd = self.protocol.cash_drawer(pin);
        self.command("cash drawer", cmd)
    }

    /// Text
    pub fn text(self, text: &str) -> Result<Self> {
        let cmd = self.protocol.text(text)?;
        self.command("text", cmd)
    }
}

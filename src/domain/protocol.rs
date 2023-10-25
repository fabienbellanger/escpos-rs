//! Protocol used to communicate with the printer

use super::{constants::*, types::*};
use crate::{
    errors::{PrinterError, Result},
    io::encoder::Encoder,
};

pub struct Protocol {
    encoder: Encoder,
}

impl Default for Protocol {
    fn default() -> Self {
        Self {
            encoder: Encoder::default(),
        }
    }
}

impl Protocol {
    /// Create new protocol
    pub fn new(encoder: Encoder) -> Self {
        Self { encoder }
    }

    /// Initialization
    pub(crate) fn init(&self) -> Command {
        ESC_HARDWARE_INIT.to_vec()
    }

    /// Reset
    pub(crate) fn reset(&self) -> Command {
        ESC_HARDWARE_RESET.to_vec()
    }

    /// Paper cut
    pub(crate) fn cut(&self, partial: bool) -> Command {
        match partial {
            true => GS_PAPER_CUT_PARTIAL.to_vec(),
            false => GS_PAPER_CUT_FULL.to_vec(),
        }
    }

    /// Emphasis
    pub(crate) fn bold(&self, enabled: bool) -> Command {
        match enabled {
            true => ESC_TEXT_EMPHASIS_ON.to_vec(),
            false => ESC_TEXT_EMPHASIS_OFF.to_vec(),
        }
    }

    /// Underline
    pub(crate) fn underline(&self, mode: UnderlineMode) -> Command {
        match mode {
            UnderlineMode::None => ESC_TEXT_UNDERLINE_NONE.to_vec(),
            UnderlineMode::Single => ESC_TEXT_UNDERLINE_SIMPLE.to_vec(),
            UnderlineMode::Double => ESC_TEXT_UNDERLINE_DOUBLE.to_vec(),
        }
    }

    /// Double strike
    pub(crate) fn double_strike(&self, enabled: bool) -> Command {
        match enabled {
            true => ESC_TEXT_DOUBLESTRIKE_ON.to_vec(),
            false => ESC_TEXT_DOUBLESTRIKE_OFF.to_vec(),
        }
    }

    /// Fonts
    pub(crate) fn font(&self, font: Font) -> Command {
        match font {
            Font::A => ESC_TEXT_FONT_A.to_vec(),
            Font::B => ESC_TEXT_FONT_B.to_vec(),
            Font::C => ESC_TEXT_FONT_C.to_vec(),
        }
    }

    /// Flip
    pub(crate) fn flip(&self, enabled: bool) -> Command {
        match enabled {
            true => ESC_TEXT_FLIP_ON.to_vec(),
            false => ESC_TEXT_FLIP_OFF.to_vec(),
        }
    }

    /// Justify
    pub(crate) fn justify(&self, mode: JustifyMode) -> Command {
        match mode {
            JustifyMode::LEFT => ESC_TEXT_JUSTIFY_LEFT.to_vec(),
            JustifyMode::CENTER => ESC_TEXT_JUSTIFY_CENTER.to_vec(),
            JustifyMode::RIGHT => ESC_TEXT_JUSTIFY_RIGHT.to_vec(),
        }
    }

    /// Reverse colours
    pub(crate) fn reverse_colours(&self, enabled: bool) -> Command {
        match enabled {
            true => GS_TEXT_REVERSE_COLOURS_ON.to_vec(),
            false => GS_TEXT_REVERSE_COLOURS_OFF.to_vec(),
        }
    }

    /// Smoothing mode
    pub(crate) fn smoothing(&self, enabled: bool) -> Command {
        match enabled {
            true => GS_TEXT_SMOOTHING_MODE_ON.to_vec(),
            false => GS_TEXT_SMOOTHING_MODE_OFF.to_vec(),
        }
    }

    /// Feed lines
    pub(crate) fn feed(&self, lines: u8) -> Command {
        let mut cmd = ESC_PAPER_FEED.to_vec();
        cmd.push(lines);
        cmd
    }

    /// Reset line spacing
    pub(crate) fn reset_line_spacing(&self) -> Command {
        ESC_TEXT_RESET_LINESPACING.to_vec()
    }

    /// Line spacing
    pub(crate) fn line_spacing(&self, value: u8) -> Command {
        let mut cmd = ESC_TEXT_LINESPACING.to_vec();
        cmd.push(value);
        cmd
    }

    /// Set text size
    pub(crate) fn text_size(&self, width: u8, height: u8) -> Result<Command> {
        if !(1..=8).contains(&width) {
            return Err(PrinterError::Input(format!("invalid text_size width: {width}")));
        }
        if !(1..=8).contains(&height) {
            return Err(PrinterError::Input(format!("invalid text_size height: {height}")));
        }

        let mut cmd = GS_TEXT_SIZE_SELECT.to_vec();
        cmd.push(((width - 1) << 4) | (height - 1));
        Ok(cmd)
    }

    /// Upside-down mode
    pub(crate) fn upside_down(&self, enabled: bool) -> Command {
        match enabled {
            true => ESC_TEXT_UPSIDE_DOWN_ON.to_vec(),
            false => ESC_TEXT_UPSIDE_DOWN_OFF.to_vec(),
        }
    }

    /// Cash drawer
    pub(crate) fn cash_drawer(&self, pin: CashDrawer) -> Command {
        match pin {
            CashDrawer::Pin2 => ESC_CASH_DRAWER_2.to_vec(),
            CashDrawer::Pin5 => ESC_CASH_DRAWER_5.to_vec(),
        }
    }

    /// Print text
    pub(crate) fn text(&self, text: &str) -> Result<Command> {
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
    fn test_reset() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.reset(), vec![27, 63, 10, 0]);
    }

    #[test]
    fn test_cut() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.cut(false), vec![29, 86, 65, 0]);
        assert_eq!(protocol.cut(true), vec![29, 86, 65, 1]);
    }

    #[test]
    fn test_bold() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.bold(false), vec![27, 69, 0]);
        assert_eq!(protocol.bold(true), vec![27, 69, 1]);
    }

    #[test]
    fn test_underline() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.underline(UnderlineMode::None), vec![27, 45, 0]);
        assert_eq!(protocol.underline(UnderlineMode::Single), vec![27, 45, 1]);
        assert_eq!(protocol.underline(UnderlineMode::Double), vec![27, 45, 2]);
    }

    #[test]
    fn test_double_strike() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.double_strike(false), vec![27, 71, 0]);
        assert_eq!(protocol.double_strike(true), vec![27, 71, 1]);
    }

    #[test]
    fn test_font() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.font(Font::A), vec![27, 77, 0]);
        assert_eq!(protocol.font(Font::B), vec![27, 77, 1]);
        assert_eq!(protocol.font(Font::C), vec![27, 77, 2]);
    }

    #[test]
    fn test_flip() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.flip(false), vec![27, 86, 0]);
        assert_eq!(protocol.flip(true), vec![27, 86, 1]);
    }

    #[test]
    fn test_justify() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.justify(JustifyMode::LEFT), vec![27, 97, 0]);
        assert_eq!(protocol.justify(JustifyMode::CENTER), vec![27, 97, 1]);
        assert_eq!(protocol.justify(JustifyMode::RIGHT), vec![27, 97, 2]);
    }

    #[test]
    fn test_reverse_colours() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.reverse_colours(false), vec![29, 66, 0]);
        assert_eq!(protocol.reverse_colours(true), vec![29, 66, 1]);
    }

    #[test]
    fn test_smoothing() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.smoothing(false), vec![29, 98, 0]);
        assert_eq!(protocol.smoothing(true), vec![29, 98, 1]);
    }

    #[test]
    fn test_feed() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.feed(0), vec![27, 100, 0]);
        assert_eq!(protocol.feed(1), vec![27, 100, 1]);
        assert_eq!(protocol.feed(255), vec![27, 100, 255]);
    }

    #[test]
    fn test_line_spacing() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.line_spacing(0), vec![27, 51, 0]);
        assert_eq!(protocol.line_spacing(1), vec![27, 51, 1]);
        assert_eq!(protocol.line_spacing(255), vec![27, 51, 255]);
        assert_eq!(protocol.reset_line_spacing(), vec![27, 50]);
    }

    #[test]
    fn test_text_size() {
        let protocol = Protocol::new(Encoder::default());
        assert!(protocol.text_size(0, 0).is_err());
        assert!(protocol.text_size(0, 2).is_err());
        assert!(protocol.text_size(2, 0).is_err());
        assert!(protocol.text_size(9, 2).is_err());
        assert!(protocol.text_size(2, 9).is_err());
        assert!(protocol.text_size(9, 9).is_err());

        assert_eq!(protocol.text_size(1, 1).unwrap(), vec![29, 33, 0]);
        assert_eq!(protocol.text_size(2, 1).unwrap(), vec![29, 33, 16]);
        assert_eq!(protocol.text_size(2, 2).unwrap(), vec![29, 33, 17]);
        assert_eq!(protocol.text_size(8, 8).unwrap(), vec![29, 33, 119]);
    }

    #[test]
    fn test_upside_down() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.upside_down(false), vec![27, 123, 0]);
        assert_eq!(protocol.upside_down(true), vec![27, 123, 1]);
    }

    #[test]
    fn test_cash_drawer() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.cash_drawer(CashDrawer::Pin2), vec![27, 112, 0]);
        assert_eq!(protocol.cash_drawer(CashDrawer::Pin5), vec![27, 112, 1]);
    }

    #[test]
    fn test_text() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.text("My text").unwrap(), "My text".as_bytes());
    }
}

//! Constants

pub const _EOL: &str = "\n";
pub const LF: u8 = 0x0A; // Print and line feed
pub const _CR: u8 = 0x0D; // Print and carriage return
pub const GS: u8 = 0x1D;
pub const ESC: u8 = 0x1B;
pub const _NIL: u8 = 0x00;

// Hardware
pub const ESC_HARDWARE_INIT: &[u8] = &[ESC, b'@'];
pub const ESC_HARDWARE_RESET: &[u8] = &[ESC, b'?', LF, 0];
pub const _ESC_HARDWARE_SELECT: &[u8] = &[ESC, b'=', 1]; // Unused

// Cash drawer
pub const ESC_CASH_DRAWER_2: &[u8] = &[ESC, b'p', 0]; // Sends a pulse to pin 2
pub const ESC_CASH_DRAWER_5: &[u8] = &[ESC, b'p', 1]; // Sends a pulse to pin 5

// Paper cut
pub const GS_PAPER_CUT_FULL: &[u8] = &[GS, b'V', b'A', 0];
pub const GS_PAPER_CUT_PARTIAL: &[u8] = &[GS, b'V', b'A', 1];
pub const ESC_PAPER_FEED: &[u8] = &[ESC, b'd'];

// Text
pub const ESC_TEXT_EMPHASIS_OFF: &[u8] = &[ESC, b'E', 0];
pub const ESC_TEXT_EMPHASIS_ON: &[u8] = &[ESC, b'E', 1];

pub const ESC_TEXT_UNDERLINE_NONE: &[u8] = &[ESC, b'-', 0];
pub const ESC_TEXT_UNDERLINE_SIMPLE: &[u8] = &[ESC, b'-', 1];
pub const ESC_TEXT_UNDERLINE_DOUBLE: &[u8] = &[ESC, b'-', 2];

pub const ESC_TEXT_DOUBLESTRIKE_OFF: &[u8] = &[ESC, b'G', 0];
pub const ESC_TEXT_DOUBLESTRIKE_ON: &[u8] = &[ESC, b'G', 1];

pub const ESC_TEXT_FONT_A: &[u8] = &[ESC, b'M', 0];
pub const ESC_TEXT_FONT_B: &[u8] = &[ESC, b'M', 1];
pub const ESC_TEXT_FONT_C: &[u8] = &[ESC, b'M', 2];

pub const ESC_TEXT_FLIP_OFF: &[u8] = &[ESC, b'V', 0];
pub const ESC_TEXT_FLIP_ON: &[u8] = &[ESC, b'V', 1];

pub const ESC_TEXT_JUSTIFY_LEFT: &[u8] = &[ESC, b'a', 0];
pub const ESC_TEXT_JUSTIFY_CENTER: &[u8] = &[ESC, b'a', 1];
pub const ESC_TEXT_JUSTIFY_RIGHT: &[u8] = &[ESC, b'a', 2];

pub const GS_TEXT_REVERSE_COLOURS_OFF: &[u8] = &[GS, b'B', 0];
pub const GS_TEXT_REVERSE_COLOURS_ON: &[u8] = &[GS, b'B', 1];

pub const GS_TEXT_SMOOTHING_MODE_OFF: &[u8] = &[GS, b'b', 0];
pub const GS_TEXT_SMOOTHING_MODE_ON: &[u8] = &[GS, b'b', 1];

pub const ESC_TEXT_RESET_LINESPACING: &[u8] = &[ESC, b'2'];
pub const ESC_TEXT_LINESPACING: &[u8] = &[ESC, b'3'];

pub const GS_TEXT_SIZE_SELECT: &[u8] = &[GS, b'!'];

pub const ESC_TEXT_UPSIDE_DOWN_OFF: &[u8] = &[ESC, b'{', 0];
pub const ESC_TEXT_UPSIDE_DOWN_ON: &[u8] = &[ESC, b'{', 1];

// pub const GS_TEXT_MARGIN_LEFT: &[u8] = &[GS, b'L'];
// pub const GS_TEXT_PRINTABLE_AREA: &[u8] = &[GS, 0x57];

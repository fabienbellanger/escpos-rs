pub const EOL: &str = "\n";
pub const LF: u8 = 0x0A; // Print and line feed
pub const CR: u8 = 0x0D; // Print and carriage return
pub const GS: u8 = 0x1D;
pub const ESC: u8 = 0x1B;
pub const NIL: u8 = 0x00;

// Initialization
pub const ESC_INIT: &[u8] = &[ESC, '@' as u8];

// Paper cut
pub const GS_PAPER_CUT_FULL: &[u8] = &[GS, 'V' as u8, 'A' as u8, 0];
pub const GS_PAPER_CUT_PARTIAL: &[u8] = &[GS, 'V' as u8, 'A' as u8, 1];

// Fonts
pub const ESC_TEXT_EMPHASIS_OFF: &[u8] = &[ESC, 'E' as u8, 0];
pub const ESC_TEXT_EMPHASIS_ON: &[u8] = &[ESC, 'E' as u8, 1];
pub const ESC_TEXT_UNDERLINE_NONE: &[u8] = &[ESC, '-' as u8, 0];
pub const ESC_TEXT_UNDERLINE_SIMPLE: &[u8] = &[ESC, '-' as u8, 1];
pub const ESC_TEXT_UNDERLINE_DOUBLE: &[u8] = &[ESC, '-' as u8, 2];
pub const ESC_TEXT_DOUBLESTRIKE_OFF: &[u8] = &[ESC, 'G' as u8, 0];
pub const ESC_TEXT_DOUBLESTRIKE_ON: &[u8] = &[ESC, 'G' as u8, 1];

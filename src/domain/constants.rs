//! Constants

pub const _EOL: &str = "\n";
pub const NUL: u8 = 0x00; // Null
pub const LF: u8 = 0x0A; // Line feed
pub const _VT: u8 = 0x0B; // Vertical tab
pub const _CR: u8 = 0x0D; // Carriage return
pub const ESC: u8 = 0x1B;
pub const GS: u8 = 0x1D; // Group separator
pub const CAN: u8 = 0x18; // Cancel

// Hardware
pub const ESC_HARDWARE_INIT: &[u8] = &[ESC, b'@'];
pub const ESC_HARDWARE_RESET: &[u8] = &[ESC, b'?', LF, 0];
pub const _ESC_HARDWARE_SELECT: &[u8] = &[ESC, b'=', 1]; // Unused

// Cash drawer
pub const ESC_CASH_DRAWER_2: &[u8] = &[ESC, b'p', 0]; // Sends a pulse to pin 2
pub const ESC_CASH_DRAWER_5: &[u8] = &[ESC, b'p', 1]; // Sends a pulse to pin 5

// Paper
pub const GS_PAPER_CUT_FULL: &[u8] = &[GS, b'V', b'A', 0];
pub const GS_PAPER_CUT_PARTIAL: &[u8] = &[GS, b'V', b'A', 1];

pub const ESC_PAPER_FEED: &[u8] = &[ESC, b'd'];

// Text
pub const ESC_CHARACTER_PAGE_CODE: &[u8] = &[ESC, b't'];
pub const ESC_CHARACTER_SET: &[u8] = &[ESC, b'R'];

pub const ESC_TEXT_EMPHASIS_OFF: &[u8] = &[ESC, b'E', 0];
pub const ESC_TEXT_EMPHASIS_ON: &[u8] = &[ESC, b'E', 1];

pub const ESC_TEXT_UNDERLINE_NONE: &[u8] = &[ESC, b'-', 0];
pub const ESC_TEXT_UNDERLINE_SIMPLE: &[u8] = &[ESC, b'-', 1];
pub const ESC_TEXT_UNDERLINE_DOUBLE: &[u8] = &[ESC, b'-', 2];

pub const ESC_TEXT_DOUBLE_STRIKE_OFF: &[u8] = &[ESC, b'G', 0];
pub const ESC_TEXT_DOUBLE_STRIKE_ON: &[u8] = &[ESC, b'G', 1];

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

pub const ESC_TEXT_RESET_LINE_SPACING: &[u8] = &[ESC, b'2'];
pub const ESC_TEXT_LINE_SPACING: &[u8] = &[ESC, b'3'];

pub const GS_TEXT_SIZE_SELECT: &[u8] = &[GS, b'!'];

pub const ESC_TEXT_UPSIDE_DOWN_OFF: &[u8] = &[ESC, b'{', 0];
pub const ESC_TEXT_UPSIDE_DOWN_ON: &[u8] = &[ESC, b'{', 1];

// Barcodes
#[cfg(feature = "barcodes")]
pub const GS_BARCODE_POSITION: &[u8] = &[GS, b'H'];
#[cfg(feature = "barcodes")]
pub const GS_BARCODE_FONT: &[u8] = &[GS, b'f'];
#[cfg(feature = "barcodes")]
pub const GS_BARCODE_HEIGHT: &[u8] = &[GS, b'h'];
#[cfg(feature = "barcodes")]
pub const GS_BARCODE_WIDTH: &[u8] = &[GS, b'w'];
#[cfg(feature = "barcodes")]
pub const GS_BARCODE_PRINT: &[u8] = &[GS, b'k'];

// QR codes
#[cfg(feature = "qrcode")]
pub const GS_2D: &[u8] = &[GS, b'(', b'k'];
#[cfg(feature = "qrcode")]
pub const GS_2D_QRCODE_MODEL: &[u8] = &[GS, b'(', b'k', 4, 0, 49, 65];
#[cfg(feature = "qrcode")]
pub const GS_2D_QRCODE_SIZE: &[u8] = &[GS, b'(', b'k', 3, 0, 49, 67];
#[cfg(feature = "qrcode")]
pub const GS_2D_QRCODE_CORRECTION_LEVEL: &[u8] = &[GS, b'(', b'k', 3, 0, 49, 69];
#[cfg(feature = "qrcode")]
pub const GS_2D_QRCODE_PRINT_SYMBOL_DATA: &[u8] = &[GS, b'(', b'k', 3, 0, 49, 81, 48];

// GS1 DataBar
#[cfg(feature = "gs1_databar_2d")]
pub const GS_2D_GS1_DATABAR_WIDTH: &[u8] = &[GS, b'(', b'k', 3, 0, 51, 67];
#[cfg(feature = "gs1_databar_2d")]
pub const GS_2D_GS1_DATABAR_WIDTH_EXTENDED: &[u8] = &[GS, b'(', b'k', 3, 0, 51, 71];
#[cfg(feature = "gs1_databar_2d")]
pub const GS_2D_GS1_DATABAR_PRINT: &[u8] = &[GS, b'(', b'k', 3, 0, 51, 81, 48];

// Image

#[cfg(feature = "graphics")]
pub const GS_IMAGE_BITMAP_PREFIX: &[u8] = &[GS, b'v', b'0'];
#[cfg(feature = "graphics")]
pub const GS_IMAGE_LOW_PREFIX: &[u8] = &[GS, b'(', b'L'];
#[cfg(feature = "graphics")]
pub const GS_IMAGE_HIGHT_PREFIX: &[u8] = &[GS, b'8', b'L'];
#[cfg(feature = "graphics")]
pub const GS_IMAGE_DENSITY: &[u8] = &[GS, b'(', b'L', 4, 0, 48, 49];
#[cfg(feature = "graphics")]
pub const GS_IMAGE_PRINT: &[u8] = &[GS, b'(', b'L', 2, 0, 48, 50];

// Others
pub const GS_SET_MOTION_UNITS: &[u8] = &[GS, b'P'];

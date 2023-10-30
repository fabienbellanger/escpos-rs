//! Protocol used to communicate with the printer

use super::{barcodes::*, constants::*, graphics::*, qrcode::*, types::*};
use crate::{
    errors::{PrinterError, Result},
    io::encoder::Encoder,
};

#[derive(Default)]
pub struct Protocol {
    encoder: Encoder,
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

    /// Set horizontal and vertical motion units
    pub(crate) fn motion_units(&self, x: u8, y: u8) -> Command {
        let mut cmd = GS_SET_MOTION_UNITS.to_vec();
        cmd.push(x);
        cmd.push(y);
        cmd
    }

    #[cfg(feature = "barcode")]
    /// Set barcode font
    pub(crate) fn barcode_font(&self, font: BarcodeFont) -> Command {
        let mut cmd = GS_BARCODE_FONT.to_vec();
        cmd.push(font.into());
        cmd
    }

    #[cfg(feature = "barcode")]
    /// Set barcode height
    pub(crate) fn barcode_height(&self, height: u8) -> Result<Command> {
        if height == 0 {
            return Err(PrinterError::Input("barcode height cannot be equal to 0".to_owned()));
        }
        let mut cmd = GS_BARCODE_HEIGHT.to_vec();
        cmd.push(height);
        Ok(cmd)
    }

    #[cfg(feature = "barcode")]
    /// Set barcode width (1 - 5)
    pub(crate) fn barcode_width(&self, width: u8) -> Result<Command> {
        if width == 0 {
            return Err(PrinterError::Input("barcode width cannot be equal to 0".to_owned()));
        }
        let width = if width > 5 { 5 } else { width };
        let mut cmd = GS_BARCODE_WIDTH.to_vec();
        cmd.push(width);
        Ok(cmd)
    }

    #[cfg(feature = "barcode")]
    /// Set barcode position
    pub(crate) fn barcode_position(&self, position: BarcodePosition) -> Command {
        let mut cmd = GS_BARCODE_POSITION.to_vec();
        cmd.push(position.into());
        cmd
    }

    #[cfg(feature = "barcode")]
    /// Print barcode
    pub(crate) fn barcode_print(&self, system: BarcodeSystem, data: &str) -> Command {
        let mut cmd = GS_BARCODE_PRINT.to_vec();
        cmd.push(system.into());
        cmd.append(&mut data.as_bytes().to_vec());
        cmd.push(NUL);
        cmd
    }

    #[cfg(feature = "qrcode")]
    /// QR code model
    pub(crate) fn qrcode_model(&self, model: QRCodeModel) -> Command {
        let mut cmd = GS_2D_QRCODE_MODEL.to_vec();
        cmd.push(model.into());
        cmd.push(0);
        cmd
    }

    #[cfg(feature = "qrcode")]
    /// QR code error correction level
    pub(crate) fn qrcode_correction_level(&self, level: QRCodeCorrectionLevel) -> Command {
        let mut cmd = GS_2D_QRCODE_CORRECTION_LEVEL.to_vec();
        cmd.push(level.into());
        cmd
    }

    #[cfg(feature = "qrcode")]
    /// QR code size (0 <= size <= 15, 0 <=> 4)
    pub(crate) fn qrcode_size(&self, size: u8) -> Command {
        let size = if size > 15 { 15 } else { size };
        let mut cmd = GS_2D_QRCODE_SIZE.to_vec();
        cmd.push(size);
        cmd
    }

    #[cfg(feature = "qrcode")]
    /// QR code data
    pub(crate) fn qrcode_data(&self, data: &str) -> Result<Command> {
        let mut cmd = GS_2D.to_vec();
        let (pl, ph) = QRCode::get_size_values(data)?;
        cmd.append(&mut vec![pl, ph, 49, 80, 48]);
        cmd.append(&mut data.as_bytes().to_vec());
        Ok(cmd)
    }

    #[cfg(feature = "qrcode")]
    /// QR code print
    pub(crate) fn qrcode_print(&self) -> Command {
        GS_2D_QRCODE_PRINT_SYMBOL_DATA.to_vec()
    }

    #[cfg(feature = "graphics")]
    /// Graphic density
    pub(crate) fn graphic_density(&self, density: GraphicDensity) -> Command {
        let mut cmd = GS_IMAGE_DENSITY.to_vec();
        cmd.push(density.into());
        cmd.push(density.into());
        cmd
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

    #[test]
    fn test_motion_units() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.motion_units(0, 255), vec![29, 80, 0, 255]);
        assert_eq!(protocol.motion_units(4, 122), vec![29, 80, 4, 122]);
    }

    #[cfg(feature = "barcode")]
    #[test]
    fn test_barcode_font() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.barcode_font(BarcodeFont::A), vec![29, 102, 0]);
        assert_eq!(protocol.barcode_font(BarcodeFont::B), vec![29, 102, 1]);
        assert_eq!(protocol.barcode_font(BarcodeFont::C), vec![29, 102, 2]);
        assert_eq!(protocol.barcode_font(BarcodeFont::D), vec![29, 102, 3]);
        assert_eq!(protocol.barcode_font(BarcodeFont::E), vec![29, 102, 4]);
    }

    #[cfg(feature = "barcode")]
    #[test]
    fn test_barcode_height() {
        let protocol = Protocol::new(Encoder::default());
        assert!(protocol.barcode_height(0).is_err());
        assert_eq!(protocol.barcode_height(5).unwrap(), vec![29, 104, 5]);
    }

    #[cfg(feature = "barcode")]
    #[test]
    fn test_barcode_width() {
        let protocol = Protocol::new(Encoder::default());
        assert!(protocol.barcode_width(0).is_err());
        assert_eq!(protocol.barcode_width(5).unwrap(), vec![29, 119, 5]);
        assert_eq!(protocol.barcode_width(1).unwrap(), vec![29, 119, 1]);
        assert_eq!(protocol.barcode_width(18).unwrap(), vec![29, 119, 5]);
    }

    #[cfg(feature = "barcode")]
    #[test]
    fn test_barcode_position() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.barcode_position(BarcodePosition::None), vec![29, 72, 0]);
        assert_eq!(protocol.barcode_position(BarcodePosition::Above), vec![29, 72, 1]);
        assert_eq!(protocol.barcode_position(BarcodePosition::Below), vec![29, 72, 2]);
        assert_eq!(protocol.barcode_position(BarcodePosition::Both), vec![29, 72, 3]);
    }

    #[cfg(feature = "barcode")]
    #[test]
    fn test_barcode_print() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.barcode_print(BarcodeSystem::UPCA, "12587458745"),
            vec![29, 107, 0, b'1', b'2', b'5', b'8', b'7', b'4', b'5', b'8', b'7', b'4', b'5', 0]
        );
        assert_eq!(
            protocol.barcode_print(BarcodeSystem::UPCE, "02587458745"),
            vec![29, 107, 1, b'0', b'2', b'5', b'8', b'7', b'4', b'5', b'8', b'7', b'4', b'5', 0]
        );
        assert_eq!(
            protocol.barcode_print(BarcodeSystem::EAN13, "025874587456"),
            vec![29, 107, 2, b'0', b'2', b'5', b'8', b'7', b'4', b'5', b'8', b'7', b'4', b'5', b'6', 0]
        );
        assert_eq!(
            protocol.barcode_print(BarcodeSystem::EAN8, "0587456"),
            vec![29, 107, 3, b'0', b'5', b'8', b'7', b'4', b'5', b'6', 0]
        );
        assert_eq!(
            protocol.barcode_print(BarcodeSystem::CODE39, "05A$"),
            vec![29, 107, 4, b'0', b'5', b'A', b'$', 0]
        );
        assert_eq!(
            protocol.barcode_print(BarcodeSystem::ITF, "0585"),
            vec![29, 107, 5, b'0', b'5', b'8', b'5', 0]
        );
        assert_eq!(
            protocol.barcode_print(BarcodeSystem::CODABAR, "A05A$C"),
            vec![29, 107, 6, b'A', b'0', b'5', b'A', b'$', b'C', 0]
        );
    }

    #[cfg(feature = "qrcode")]
    #[test]
    fn test_qrcode_model() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.qrcode_model(QRCodeModel::Model1),
            vec![29, 40, 107, 4, 0, 49, 65, 49, 0]
        );
        assert_eq!(
            protocol.qrcode_model(QRCodeModel::Model2),
            vec![29, 40, 107, 4, 0, 49, 65, 50, 0]
        );
        assert_eq!(
            protocol.qrcode_model(QRCodeModel::Micro),
            vec![29, 40, 107, 4, 0, 49, 65, 51, 0]
        );
    }

    #[cfg(feature = "qrcode")]
    #[test]
    fn test_qrcode_correction_level() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.qrcode_correction_level(QRCodeCorrectionLevel::L),
            vec![29, 40, 107, 3, 0, 49, 69, 48]
        );
        assert_eq!(
            protocol.qrcode_correction_level(QRCodeCorrectionLevel::M),
            vec![29, 40, 107, 3, 0, 49, 69, 49]
        );
        assert_eq!(
            protocol.qrcode_correction_level(QRCodeCorrectionLevel::Q),
            vec![29, 40, 107, 3, 0, 49, 69, 50]
        );
        assert_eq!(
            protocol.qrcode_correction_level(QRCodeCorrectionLevel::H),
            vec![29, 40, 107, 3, 0, 49, 69, 51]
        );
    }

    #[cfg(feature = "qrcode")]
    #[test]
    fn test_qrcode_size() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.qrcode_size(0), vec![29, 40, 107, 3, 0, 49, 67, 0]);
        assert_eq!(protocol.qrcode_size(1), vec![29, 40, 107, 3, 0, 49, 67, 1]);
        assert_eq!(protocol.qrcode_size(8), vec![29, 40, 107, 3, 0, 49, 67, 8]);
        assert_eq!(protocol.qrcode_size(16), vec![29, 40, 107, 3, 0, 49, 67, 16]);
        assert_eq!(protocol.qrcode_size(128), vec![29, 40, 107, 3, 0, 49, 67, 128]);
        assert_eq!(protocol.qrcode_size(255), vec![29, 40, 107, 3, 0, 49, 67, 255]);
    }

    #[cfg(feature = "qrcode")]
    #[test]
    fn test_qrcode_data() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.qrcode_data("test data qrcode").unwrap(),
            vec![
                29, 40, 107, 19, 0, 49, 80, 48, 116, 101, 115, 116, 32, 100, 97, 116, 97, 32, 113, 114, 99, 111, 100,
                101
            ]
        );
        assert_eq!(protocol.qrcode_data("").unwrap(), vec![29, 40, 107, 3, 0, 49, 80, 48]);
    }

    #[cfg(feature = "qrcode")]
    #[test]
    fn test_qrcode_print() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.qrcode_print(), vec![29, 40, 107, 3, 0, 49, 81, 48]);
    }

    #[cfg(feature = "graphics")]
    #[test]
    fn test_graphic_density() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.graphic_density(GraphicDensity::Low),
            vec![29, 40, 76, 4, 0, 48, 49, 50, 50]
        );
        assert_eq!(
            protocol.graphic_density(GraphicDensity::Hight),
            vec![29, 40, 76, 4, 0, 48, 49, 51, 51]
        );
    }
}

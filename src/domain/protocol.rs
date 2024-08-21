//! Protocol used to communicate with the printer

#[cfg(feature = "graphics")]
use super::bit_image::*;
use super::{character::*, codes::*, common::get_parameters_number_2, constants::*, types::*, RealTimeStatusRequest};
#[cfg(feature = "ui")]
use crate::domain::ui::{line::Line, UIComponent};
#[cfg(feature = "ui")]
use crate::printer::PrinterStyleState;
#[cfg(feature = "ui")]
use crate::printer_options::PrinterOptions;
use crate::{
    domain::page_codes::PageCodeTable,
    errors::{PrinterError, Result},
    io::encoder::Encoder,
};

/// Protocol used to communicate with the printer
#[derive(Default, Clone)]
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

    #[allow(dead_code)]
    /// Cancel
    pub(crate) fn cancel(&self) -> Command {
        vec![CAN]
    }

    /// Paper cut
    pub(crate) fn cut(&self, partial: bool) -> Command {
        match partial {
            true => GS_PAPER_CUT_PARTIAL.to_vec(),
            false => GS_PAPER_CUT_FULL.to_vec(),
        }
    }

    /// Character page code
    pub(crate) fn page_code(&self, code: PageCode) -> Command {
        let mut cmd = ESC_CHARACTER_PAGE_CODE.to_vec();
        cmd.push(code.into());
        cmd
    }

    /// International character set
    pub(crate) fn character_set(&self, code: CharacterSet) -> Command {
        let mut cmd = ESC_CHARACTER_SET.to_vec();
        cmd.push(code.into());
        cmd
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
            true => ESC_TEXT_DOUBLE_STRIKE_ON.to_vec(),
            false => ESC_TEXT_DOUBLE_STRIKE_OFF.to_vec(),
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
        ESC_TEXT_RESET_LINE_SPACING.to_vec()
    }

    /// Line spacing
    pub(crate) fn line_spacing(&self, value: u8) -> Command {
        let mut cmd = ESC_TEXT_LINE_SPACING.to_vec();
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
    pub(crate) fn text(&self, text: &str, page_code: Option<PageCode>, max_length: Option<usize>) -> Result<Command> {
        match page_code {
            Some(page_code) => {
                let table: PageCodeTable = page_code.try_into()?;
                let table = table.get_table();
                let mut cmd = Vec::new();

                let mut i = 0;
                for c in text.chars() {
                    if let Some(max_length) = max_length {
                        if i >= max_length {
                            break;
                        }
                    }

                    if let Some(&n) = table.get(&c) {
                        cmd.push(n);

                        i += 1;
                    } else {
                        let mut s = self.encoder.encode(&c.to_string())?;

                        i += s.len();

                        cmd.append(&mut s);
                    }
                }

                Ok(cmd)
            }
            None => match max_length {
                Some(max_length) => self.encoder.encode(text).map(|mut cmd| {
                    cmd.truncate(max_length);
                    cmd
                }),
                _ => self.encoder.encode(text),
            },
        }
    }

    /// Set horizontal and vertical motion units
    pub(crate) fn motion_units(&self, x: u8, y: u8) -> Command {
        let mut cmd = GS_SET_MOTION_UNITS.to_vec();
        cmd.push(x);
        cmd.push(y);
        cmd
    }

    /// Transmit real-time status
    pub(crate) fn real_time_status(&self, status: RealTimeStatusRequest) -> Command {
        let mut cmd = DLE_REAL_TIME_STATUS.to_vec();
        let (n, a) = status.into();
        cmd.push(n);
        cmd.push(a);
        cmd
    }

    #[cfg(feature = "barcodes")]
    /// Set barcode font
    fn barcode_font(&self, font: BarcodeFont) -> Command {
        let mut cmd = GS_BARCODE_FONT.to_vec();
        cmd.push(font.into());
        cmd
    }

    #[cfg(feature = "barcodes")]
    /// Set barcode height
    fn barcode_height(&self, height: u8) -> Result<Command> {
        if height == 0 {
            return Err(PrinterError::Input("barcode height cannot be equal to 0".to_owned()));
        }
        let mut cmd = GS_BARCODE_HEIGHT.to_vec();
        cmd.push(height);
        Ok(cmd)
    }

    #[cfg(feature = "barcodes")]
    /// Set barcode width (1 - 5)
    fn barcode_width(&self, width: u8) -> Result<Command> {
        if width == 0 {
            return Err(PrinterError::Input("barcode width cannot be equal to 0".to_owned()));
        }
        let width = if width > 5 { 5 } else { width };
        let mut cmd = GS_BARCODE_WIDTH.to_vec();
        cmd.push(width);
        Ok(cmd)
    }

    #[cfg(feature = "barcodes")]
    /// Set barcode position
    fn barcode_position(&self, position: BarcodePosition) -> Command {
        let mut cmd = GS_BARCODE_POSITION.to_vec();
        cmd.push(position.into());
        cmd
    }

    #[cfg(feature = "barcodes")]
    /// Print barcode
    fn barcode_print(&self, system: BarcodeSystem, data: &str) -> Command {
        let mut cmd = GS_BARCODE_PRINT.to_vec();
        cmd.push(system.into());
        cmd.append(&mut data.as_bytes().to_vec());
        cmd.push(NUL);
        cmd
    }

    #[cfg(feature = "barcodes")]
    /// Configure and print barcode
    pub(crate) fn barcode(&self, data: &str, system: BarcodeSystem, option: BarcodeOption) -> Result<Vec<Command>> {
        Ok(vec![
            self.barcode_width(option.width().into())?,
            self.barcode_height(option.height().into())?,
            self.barcode_font(option.font()),
            self.barcode_position(option.position()),
            self.barcode_print(system, data),
        ])
    }

    #[cfg(feature = "codes_2d")]
    /// QR code model
    fn qrcode_model(&self, model: QRCodeModel) -> Command {
        let mut cmd = GS_2D_QRCODE_MODEL.to_vec();
        cmd.push(model.into());
        cmd.push(0);
        cmd
    }

    #[cfg(feature = "codes_2d")]
    /// QR code error correction level
    fn qrcode_correction_level(&self, level: QRCodeCorrectionLevel) -> Command {
        let mut cmd = GS_2D_QRCODE_CORRECTION_LEVEL.to_vec();
        cmd.push(level.into());
        cmd
    }

    #[cfg(feature = "codes_2d")]
    /// QR code size (0 <= size <= 15, 0 <=> 4)
    fn qrcode_size(&self, size: u8) -> Command {
        let size = if size > 15 { 15 } else { size };
        let mut cmd = GS_2D_QRCODE_SIZE.to_vec();
        cmd.push(size);
        cmd
    }

    #[cfg(feature = "codes_2d")]
    /// QR code data
    fn qrcode_data(&self, data: &str) -> Result<Command> {
        let mut cmd = GS_2D.to_vec();
        let (pl, ph) = get_parameters_number_2(data, 3)?;
        cmd.append(&mut vec![pl, ph, 49, 80, 48]);
        cmd.append(&mut data.as_bytes().to_vec());
        Ok(cmd)
    }

    #[cfg(feature = "codes_2d")]
    /// QR code print
    fn qrcode_print(&self) -> Command {
        GS_2D_QRCODE_PRINT_SYMBOL_DATA.to_vec()
    }

    #[cfg(feature = "codes_2d")]
    /// QR code print
    pub(crate) fn qrcode(&self, data: &str, option: QRCodeOption) -> Result<Vec<Command>> {
        Ok(vec![
            self.qrcode_model(option.model()),
            self.qrcode_size(option.size()),
            self.qrcode_correction_level(option.correction_level()),
            self.qrcode_data(data)?,
            self.qrcode_print(),
        ])
    }

    #[cfg(feature = "codes_2d")]
    /// 2D GS1 DataBar width
    fn gs1_databar_2d_width(&self, size: GS1DataBar2DWidth) -> Command {
        let mut cmd = GS_2D_GS1_DATABAR_WIDTH.to_vec();
        cmd.push(size.into());
        cmd
    }

    #[cfg(feature = "codes_2d")]
    /// 2D GS1 DataBar expanded max width
    // TODO: To implement
    fn gs1_databar_2d_expanded_width(&self, _max: u8) -> Command {
        let mut cmd = GS_2D_GS1_DATABAR_WIDTH_EXTENDED.to_vec();
        cmd.append(&mut vec![0, 0]);
        cmd
    }

    #[cfg(feature = "codes_2d")]
    /// 2D GS1 DataBar data
    fn gs1_databar_2d_data(&self, data: &str, code_type: GS1DataBar2DType) -> Result<Command> {
        let mut cmd = GS_2D.to_vec();
        let (pl, ph) = get_parameters_number_2(data, 4)?;
        cmd.push(pl);
        cmd.push(ph);
        cmd.append(&mut vec![51, 80, 48]);
        cmd.push(code_type.into());
        cmd.append(&mut data.as_bytes().to_vec());

        Ok(cmd)
    }

    #[cfg(feature = "codes_2d")]
    /// 2D GS1 DataBar print
    fn gs1_databar_2d_print(&self) -> Command {
        GS_2D_GS1_DATABAR_PRINT.to_vec()
    }

    #[cfg(feature = "codes_2d")]
    /// 2D GS1 DataBar
    pub(crate) fn gs1_databar_2d(&self, data: &str, option: GS1DataBar2DOption) -> Result<Vec<Command>> {
        Ok(vec![
            self.gs1_databar_2d_width(option.width()),
            self.gs1_databar_2d_expanded_width(0),
            self.gs1_databar_2d_data(data, option.code_type())?,
            self.gs1_databar_2d_print(),
        ])
    }

    #[cfg(feature = "codes_2d")]
    /// PDF417 number of columns
    fn pdf417_columns(&self, option: &Pdf417Option) -> Command {
        let mut cmd = GS_2D_PDF417_COLUMNS.to_vec();
        cmd.push(option.columns());
        cmd
    }

    #[cfg(feature = "codes_2d")]
    /// PDF417 number of rows
    fn pdf417_rows(&self, option: &Pdf417Option) -> Command {
        let mut cmd = GS_2D_PDF417_ROWS.to_vec();
        cmd.push(option.rows());
        cmd
    }

    #[cfg(feature = "codes_2d")]
    /// PDF417 width
    fn pdf417_width(&self, option: &Pdf417Option) -> Command {
        let mut cmd = GS_2D_PDF417_WIDTH.to_vec();
        cmd.push(option.width());
        cmd
    }

    #[cfg(feature = "codes_2d")]
    /// PDF417 row height
    fn pdf417_row_height(&self, option: &Pdf417Option) -> Command {
        let mut cmd = GS_2D_PDF417_ROW_HEIGHT.to_vec();
        cmd.push(option.row_height());
        cmd
    }

    #[cfg(feature = "codes_2d")]
    /// PDF417 error correction level
    fn pdf417_correction_level(&self, option: &Pdf417Option) -> Result<Command> {
        let mut cmd = GS_2D_PDF417_CORRECTION_LEVEL.to_vec();
        let (m, n) = option.correction_level().try_into()?;
        cmd.push(m);
        cmd.push(n);
        Ok(cmd)
    }

    #[cfg(feature = "codes_2d")]
    /// PDF417 type
    fn pdf417_type(&self, option: &Pdf417Option) -> Command {
        let mut cmd = GS_2D_PDF417_TYPE.to_vec();
        cmd.push(option.code_type().into());
        cmd
    }

    #[cfg(feature = "codes_2d")]
    /// PDF417 data
    fn pdf417_data(&self, data: &str) -> Result<Command> {
        let mut cmd = GS_2D.to_vec();
        let (pl, ph) = get_parameters_number_2(data, 3)?;
        cmd.push(pl);
        cmd.push(ph);
        cmd.append(&mut vec![48, 80, 48]);
        cmd.append(&mut data.as_bytes().to_vec());
        Ok(cmd)
    }

    #[cfg(feature = "codes_2d")]
    /// PDF417 print
    fn pdf417_print(&self) -> Command {
        GS_2D_PDF417_PRINT.to_vec()
    }

    #[cfg(feature = "codes_2d")]
    /// PDF417
    pub(crate) fn pdf417(&self, data: &str, option: Pdf417Option) -> Result<Vec<Command>> {
        Ok(vec![
            self.pdf417_columns(&option),
            self.pdf417_rows(&option),
            self.pdf417_width(&option),
            self.pdf417_row_height(&option),
            self.pdf417_correction_level(&option)?,
            self.pdf417_type(&option),
            self.pdf417_data(data)?,
            self.pdf417_print(),
        ])
    }

    #[cfg(feature = "codes_2d")]
    /// MaxiCode mode
    fn maxi_code_mode(&self, mode: MaxiCodeMode) -> Command {
        let mut cmd = GS_2D_MAXI_CODE_MODE.to_vec();
        cmd.push(mode.into());
        cmd
    }

    #[cfg(feature = "codes_2d")]
    /// MaxiCode data
    fn maxi_code_data(&self, data: &str) -> Result<Command> {
        let mut cmd = GS_2D.to_vec();
        let (pl, ph) = get_parameters_number_2(data, 3)?;
        cmd.push(pl);
        cmd.push(ph);
        cmd.append(&mut vec![50, 80, 48]);
        cmd.append(&mut data.as_bytes().to_vec());
        Ok(cmd)
    }

    #[cfg(feature = "codes_2d")]
    /// MaxiCode print
    fn maxi_code_print(&self) -> Command {
        GS_2D_MAXI_CODE_PRINT.to_vec()
    }

    #[cfg(feature = "codes_2d")]
    /// MaxiCode
    pub(crate) fn maxi_code(&self, data: &str, mode: MaxiCodeMode) -> Result<Vec<Command>> {
        let code = MaxiCode::new(data, mode);

        Ok(vec![
            self.maxi_code_mode(code.mode),
            self.maxi_code_data(&code.data)?,
            self.maxi_code_print(),
        ])
    }

    #[cfg(feature = "codes_2d")]
    /// DataMatrix type, numbers of rows and columns
    fn data_matrix_type(&self, code_type: DataMatrixType) -> Result<Command> {
        let mut cmd = GS_2D_DATA_MATRIX_TYPE.to_vec();
        let (m, d1, d2) = code_type.try_into()?;
        cmd.push(m);
        cmd.push(d1);
        cmd.push(d2);
        Ok(cmd)
    }

    #[cfg(feature = "codes_2d")]
    /// DataMatrix size
    fn data_matrix_size(&self, size: u8) -> Command {
        let mut cmd = GS_2D_DATA_MATRIX_SIZE.to_vec();
        cmd.push(size);
        cmd
    }

    #[cfg(feature = "codes_2d")]
    /// DataMatrix data
    fn data_matrix_data(&self, data: &str) -> Result<Command> {
        let mut cmd = GS_2D.to_vec();
        let (pl, ph) = get_parameters_number_2(data, 3)?;
        cmd.push(pl);
        cmd.push(ph);
        cmd.append(&mut vec![54, 80, 48]);
        cmd.append(&mut data.as_bytes().to_vec());
        Ok(cmd)
    }

    #[cfg(feature = "codes_2d")]
    /// DataMatrix print
    fn data_matrix_print(&self) -> Command {
        GS_2D_DATA_MATRIX_PRINT.to_vec()
    }

    #[cfg(feature = "codes_2d")]
    /// DataMatrix
    pub(crate) fn data_matrix(&self, data: &str, option: DataMatrixOption) -> Result<Vec<Command>> {
        Ok(vec![
            self.data_matrix_type(option.code_type())?,
            self.data_matrix_size(option.size()),
            self.data_matrix_data(data)?,
            self.data_matrix_print(),
        ])
    }

    #[cfg(feature = "codes_2d")]
    /// Aztec code mode
    fn aztec_mode(&self, mode: AztecMode) -> Result<Command> {
        let mut cmd = GS_2D_AZTEC_CODE_MODE.to_vec();
        let (n1, n2) = mode.try_into()?;
        cmd.push(n1);
        cmd.push(n2);
        Ok(cmd)
    }

    #[cfg(feature = "codes_2d")]
    /// Aztec code size
    fn aztec_size(&self, size: u8) -> Command {
        let mut cmd = GS_2D_AZTEC_CODE_SIZE.to_vec();
        cmd.push(size);
        cmd
    }

    #[cfg(feature = "codes_2d")]
    /// Aztec code error correction level
    fn aztec_correction_level(&self, level: u8) -> Command {
        let mut cmd = GS_2D_AZTEC_CODE_CORRECTION_LEVEL.to_vec();
        cmd.push(level);
        cmd
    }

    #[cfg(feature = "codes_2d")]
    /// Aztec code data
    fn aztec_data(&self, data: &str) -> Result<Command> {
        let mut cmd = GS_2D.to_vec();
        let (pl, ph) = get_parameters_number_2(data, 3)?;
        cmd.push(pl);
        cmd.push(ph);
        cmd.append(&mut vec![53, 80, 48]);
        cmd.append(&mut data.as_bytes().to_vec());
        Ok(cmd)
    }

    #[cfg(feature = "codes_2d")]
    /// Aztec code print
    fn aztec_print(&self) -> Command {
        GS_2D_AZTEC_CODE_PRINT.to_vec()
    }

    #[cfg(feature = "codes_2d")]
    /// Aztec code
    pub(crate) fn aztec(&self, data: &str, option: AztecOption) -> Result<Vec<Command>> {
        let code = Aztec::new(data, option);
        Ok(vec![
            self.aztec_mode(code.option.mode())?,
            self.aztec_size(code.option.size()),
            self.aztec_correction_level(code.option.correction_level()),
            self.aztec_data(data)?,
            self.aztec_print(),
        ])
    }

    #[cfg(feature = "graphics")]
    /// Print bit image
    pub(crate) fn bit_image(&self, path: &str, option: BitImageOption) -> Result<Command> {
        let bit_image = BitImage::new(path, option)?;
        self.build_bit_image(bit_image)
    }

    #[cfg(feature = "graphics")]
    /// Print bit image from bytes
    pub(crate) fn bit_image_from_bytes(&self, bytes: &[u8], option: BitImageOption) -> Result<Command> {
        let bit_image = BitImage::from_bytes(bytes, option)?;
        self.build_bit_image(bit_image)
    }

    #[cfg(feature = "graphics")]
    fn build_bit_image(&self, bit_image: BitImage) -> Result<Command> {
        let mut cmd = GS_IMAGE_BITMAP_PREFIX.to_vec();

        // Size
        cmd.push(bit_image.size().into());

        // Width and height
        cmd.append(&mut bit_image.with_bytes_u8()?);
        cmd.append(&mut bit_image.height_u8()?);

        // Data
        cmd.append(&mut bit_image.raster_data()?);

        Ok(cmd)
    }

    #[cfg(feature = "ui")]
    pub(crate) fn draw_line(
        &self,
        line: Line,
        options: PrinterOptions,
        style_state: PrinterStyleState,
    ) -> Result<Vec<Command>> {
        let commands = line.render(self.clone(), options, style_state)?;
        Ok(commands)
    }

    // #[cfg(feature = "graphics")]
    // /// Graphic density
    // pub(crate) fn graphic_density(&self, density: GraphicDensity) -> Command {
    //     let mut cmd = GS_IMAGE_DENSITY.to_vec();
    //     cmd.push(density.into());
    //     cmd.push(density.into());
    //     cmd
    // }

    // #[cfg(feature = "graphics")]
    // /// Print graphic
    // pub(crate) fn graphic_print(&self) -> Command {
    //     GS_IMAGE_PRINT.to_vec()
    // }

    // #[cfg(feature = "graphics")]
    // /// Print graphic
    // pub(crate) fn graphic_data(&self, path: &str) -> Result<Command> {
    //     let mut cmd = GS_IMAGE_HIGH_PREFIX.to_vec();
    //
    //     // pL, pH => Parameters
    //     let graphic = Graphic::new(path, None)?;
    //     let (p1, p2, p3, p4) = graphic.data_size()?;
    //     cmd.push(p1);
    //     cmd.push(p2);
    //     cmd.push(p3);
    //     cmd.push(p4);
    //     cmd.push(48);
    //     cmd.push(112);
    //
    //     // Tone
    //     cmd.push(graphic.tone());
    //
    //     // bx, by
    //     cmd.push(1);
    //     cmd.push(1);
    //
    //     // Color
    //     cmd.push(graphic.color());
    //
    //     // Number of dots
    //     let (xl, xh) = graphic.dots_per_direction(graphic.width() as usize)?;
    //     let (yl, yh) = graphic.dots_per_direction(graphic.height() as usize)?;
    //     cmd.push(xl);
    //     cmd.push(xh);
    //     cmd.push(yl);
    //     cmd.push(yh);
    //
    //     dbg!(p1, p2, p3, p4, xl, xh, yl, yh);
    //
    //     // Image data
    //     for i in graphic.data()? {
    //         cmd.push(i);
    //     }
    //
    //     Ok(cmd)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "ui")]
    use crate::domain::ui::line::{LineBuilder, LineStyle};

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
    fn test_cancel() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.cancel(), vec![24]);
    }

    #[test]
    fn test_cut() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.cut(false), vec![29, 86, 65, 0]);
        assert_eq!(protocol.cut(true), vec![29, 86, 65, 1]);
    }

    #[test]
    fn test_page_code() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.page_code(PageCode::default()), vec![27, 116, 0]);
        assert_eq!(protocol.page_code(PageCode::PC858), vec![27, 116, 19]);
    }

    #[test]
    fn test_character_set() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.character_set(CharacterSet::USA), vec![27, 82, 0]);
        assert_eq!(protocol.character_set(CharacterSet::France), vec![27, 82, 1]);
        assert_eq!(protocol.character_set(CharacterSet::IndiaMarathi), vec![27, 82, 82]);
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
    fn test_text_without_page_code() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.text("My text", None, None).unwrap(), "My text".as_bytes());
    }

    #[test]
    fn test_text_with_max_length() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.text("My text with a long content", None, Some(16)).unwrap(),
            "My text with a l".as_bytes()
        );
        assert_eq!(
            protocol.text("My text with ┼ character", None, Some(19)).unwrap(),
            "My text with ┼ ch".as_bytes() // 19 - 2 (┼ 3 bytes) = 17
        );
        assert_eq!(
            protocol
                .text("My text with ┼ character", Some(PageCode::PC437), Some(19))
                .unwrap(),
            vec![77, 121, 32, 116, 101, 120, 116, 32, 119, 105, 116, 104, 32, 197, 32, 99, 104, 97, 114]
        );
        assert_eq!(
            protocol
                .text("My text with ώ character", Some(PageCode::PC437), Some(19))
                .unwrap(),
            "My text with ώ cha".as_bytes() // 19 - 1 (ώ 2 bytes) = 18
        );
        assert_eq!(
            protocol
                .text("My text with 😊 character", Some(PageCode::PC437), Some(19))
                .unwrap(),
            "My text with 😊 c".as_bytes() // 19 - 3 (😊 4 bytes) = 16
        );
        assert_eq!(
            protocol.text("My text with 😊 character", None, Some(19)).unwrap(),
            "My text with 😊 c".as_bytes() // 19 - 3 (😊 4 bytes) = 16
        );
    }

    #[test]
    fn test_text_with_page_code() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.text("My text é €", Some(PageCode::PC858), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 130, 32, 213]
        );
        assert_eq!(
            protocol.text("My text é ã ç õ", Some(PageCode::PC860), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 130, 32, 132, 32, 135, 32, 148]
        );
        assert_eq!(
            protocol.text("My text ø ¤", Some(PageCode::PC865), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 155, 32, 175]
        );
        assert_eq!(
            protocol.text("My text č Ž š Đ", Some(PageCode::PC852), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 159, 32, 166, 32, 231, 32, 209]
        );
        assert_eq!(
            protocol.text("My text Ś š ¤", Some(PageCode::ISO8859_2), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 166, 32, 185, 32, 164]
        );
        assert_eq!(
            protocol.text("My text Ψ π Θ", Some(PageCode::ISO8859_7), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 216, 32, 240, 32, 200]
        );
        assert_eq!(
            protocol
                .text("My text Ž £ æ þ", Some(PageCode::ISO8859_15), None)
                .unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 180, 32, 163, 32, 230, 32, 254]
        );
        assert_eq!(
            protocol.text("My text € Ç ÿ", Some(PageCode::WPC1252), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 128, 32, 199, 32, 255]
        );
        assert_eq!(
            protocol.text("My text ｦ ﾎ ﾟ", Some(PageCode::Katakana), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 166, 32, 206, 32, 223]
        );
        assert_eq!(
            protocol.text("My text × § Ð", Some(PageCode::PC850), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 158, 32, 245, 32, 209]
        );
        assert_eq!(
            protocol.text("My text ¶ ¦ ¾", Some(PageCode::PC863), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 134, 32, 160, 32, 173]
        );
        assert_eq!(
            protocol.text("My text ψ Ώ Φ", Some(PageCode::PC851), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0xF6, 32, 0x98, 32, 0xD2]
        );
        assert_eq!(
            protocol.text("My text ℓ Ħ ş", Some(PageCode::PC853), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0xF2, 32, 0xE7, 32, 0xAD]
        );
        assert_eq!(
            protocol.text("My text Ğ ª ÿ", Some(PageCode::PC857), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0xA6, 32, 0xD1, 32, 0xED]
        );
        assert_eq!(
            protocol.text("My text Ή ς Λ", Some(PageCode::PC737), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0xEC, 32, 0xAA, 32, 0x8A]
        );
        assert_eq!(
            protocol.text("My text ю Щ Є", Some(PageCode::PC866), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0xEE, 32, 0x99, 32, 0xF2]
        );
        assert_eq!(
            protocol.text("My text Ķ č Ł", Some(PageCode::WPC775), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0xE8, 32, 0xD1, 32, 0xAD]
        );
        assert_eq!(
            protocol.text("My text Њ Ж Й", Some(PageCode::PC855), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0x93, 32, 0xEA, 32, 0xBE]
        );
        assert_eq!(
            protocol.text("My text þ Þ Σ", Some(PageCode::PC861), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0x95, 32, 0x8D, 32, 0xE4]
        );
        assert_eq!(
            protocol.text("My text א ש √", Some(PageCode::PC862), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0x80, 32, 0x99, 32, 0xFB]
        );
        assert_eq!(
            protocol.text("My text ώ © Λ", Some(PageCode::PC869), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0xFD, 32, 0x97, 32, 0xB6]
        );
        assert_eq!(
            protocol.text("My text Š ž “", Some(PageCode::PC1118), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0xBE, 32, 0xD8, 32, 0xF5]
        );
        assert_eq!(
            protocol.text("My text Ą Ū ž", Some(PageCode::PC1119), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0xB5, 32, 0xC7, 32, 0xD8]
        );
        assert_eq!(
            protocol.text("My text Ґ Є №", Some(PageCode::PC1125), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0xF2, 32, 0xF4, 32, 0xFC]
        );
        assert_eq!(
            protocol.text("My text ‡ ť ¶", Some(PageCode::WPC1250), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0x87, 32, 0x9D, 32, 0xB6]
        );
        assert_eq!(
            protocol.text("My text Ђ џ Ъ", Some(PageCode::WPC1251), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0x80, 32, 0x9F, 32, 0xDA]
        );
        assert_eq!(
            protocol.text("My text ƒ ‰ ¥", Some(PageCode::WPC1253), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0x83, 32, 0x89, 32, 0xA5]
        );
        assert_eq!(
            protocol.text("My text Ğ Ş ş", Some(PageCode::WPC1254), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0xD0, 32, 0xDE, 32, 0xFE]
        );
        assert_eq!(
            protocol.text("My text Æ Ģ ų", Some(PageCode::WPC1257), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0xAF, 32, 0xCC, 32, 0xF8]
        );
        assert_eq!(
            protocol.text("My text Ђ Ә ғ", Some(PageCode::KZ1048), None).unwrap(),
            &[77, 121, 32, 116, 101, 120, 116, 32, 0x80, 32, 0xA3, 32, 0xBA]
        );

        // With page code table not yet implemented
        assert!(protocol.text("My text", Some(PageCode::Hiragana), None).is_err());
    }

    #[test]
    fn test_motion_units() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.motion_units(0, 255), vec![29, 80, 0, 255]);
        assert_eq!(protocol.motion_units(4, 122), vec![29, 80, 4, 122]);
    }

    #[test]
    fn test_real_time_status() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.real_time_status(RealTimeStatusRequest::Printer),
            vec![16, 4, 1, 0]
        );
        assert_eq!(
            protocol.real_time_status(RealTimeStatusRequest::OfflineCause),
            vec![16, 4, 2, 0]
        );
        assert_eq!(
            protocol.real_time_status(RealTimeStatusRequest::ErrorCause),
            vec![16, 4, 3, 0]
        );
        assert_eq!(
            protocol.real_time_status(RealTimeStatusRequest::RollPaperSensor),
            vec![16, 4, 4, 0]
        );
        assert_eq!(
            protocol.real_time_status(RealTimeStatusRequest::InkA),
            vec![16, 4, 7, 1]
        );
        assert_eq!(
            protocol.real_time_status(RealTimeStatusRequest::InkB),
            vec![16, 4, 7, 2]
        );
        assert_eq!(
            protocol.real_time_status(RealTimeStatusRequest::Peeler),
            vec![16, 4, 8, 3]
        );
        assert_eq!(
            protocol.real_time_status(RealTimeStatusRequest::Interface),
            vec![16, 4, 18, 1]
        );
        assert_eq!(
            protocol.real_time_status(RealTimeStatusRequest::DMD),
            vec![16, 4, 18, 2]
        );
    }

    #[cfg(feature = "barcodes")]
    #[test]
    fn test_barcode_font() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.barcode_font(BarcodeFont::A), vec![29, 102, 0]);
        assert_eq!(protocol.barcode_font(BarcodeFont::B), vec![29, 102, 1]);
        assert_eq!(protocol.barcode_font(BarcodeFont::C), vec![29, 102, 2]);
        assert_eq!(protocol.barcode_font(BarcodeFont::D), vec![29, 102, 3]);
        assert_eq!(protocol.barcode_font(BarcodeFont::E), vec![29, 102, 4]);
    }

    #[cfg(feature = "barcodes")]
    #[test]
    fn test_barcode_height() {
        let protocol = Protocol::new(Encoder::default());
        assert!(protocol.barcode_height(0).is_err());
        assert_eq!(protocol.barcode_height(5).unwrap(), vec![29, 104, 5]);
    }

    #[cfg(feature = "barcodes")]
    #[test]
    fn test_barcode_width() {
        let protocol = Protocol::new(Encoder::default());
        assert!(protocol.barcode_width(0).is_err());
        assert_eq!(protocol.barcode_width(5).unwrap(), vec![29, 119, 5]);
        assert_eq!(protocol.barcode_width(1).unwrap(), vec![29, 119, 1]);
        assert_eq!(protocol.barcode_width(18).unwrap(), vec![29, 119, 5]);
    }

    #[cfg(feature = "barcodes")]
    #[test]
    fn test_barcode_position() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.barcode_position(BarcodePosition::None), vec![29, 72, 0]);
        assert_eq!(protocol.barcode_position(BarcodePosition::Above), vec![29, 72, 1]);
        assert_eq!(protocol.barcode_position(BarcodePosition::Below), vec![29, 72, 2]);
        assert_eq!(protocol.barcode_position(BarcodePosition::Both), vec![29, 72, 3]);
    }

    #[cfg(feature = "barcodes")]
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

    #[cfg(feature = "barcodes")]
    #[test]
    fn test_barcode() {
        let protocol = Protocol::new(Encoder::default());
        let expected: Vec<Command> = vec![
            [29, 119, 4].to_vec(),
            [29, 104, 102].to_vec(),
            [29, 102, 0].to_vec(),
            [29, 72, 0].to_vec(),
            [29, 107, 2, 49, 50, 51, 52, 53, 54, 55, 56, 57, 48, 49, 50, 0].to_vec(),
        ];

        assert_eq!(
            protocol
                .barcode(
                    "123456789012",
                    BarcodeSystem::EAN13,
                    BarcodeOption::new(BarcodeWidth::L, BarcodeHeight::S, BarcodeFont::A, BarcodePosition::None),
                )
                .unwrap(),
            expected
        );
    }

    #[cfg(feature = "codes_2d")]
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

    #[cfg(feature = "codes_2d")]
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

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_qrcode_size() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.qrcode_size(0), vec![29, 40, 107, 3, 0, 49, 67, 0]);
        assert_eq!(protocol.qrcode_size(1), vec![29, 40, 107, 3, 0, 49, 67, 1]);
        assert_eq!(protocol.qrcode_size(8), vec![29, 40, 107, 3, 0, 49, 67, 8]);
        assert_eq!(protocol.qrcode_size(15), vec![29, 40, 107, 3, 0, 49, 67, 15]);
        assert_eq!(protocol.qrcode_size(128), vec![29, 40, 107, 3, 0, 49, 67, 15]);
        assert_eq!(protocol.qrcode_size(255), vec![29, 40, 107, 3, 0, 49, 67, 15]);
    }

    #[cfg(feature = "codes_2d")]
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

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_qrcode_print() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.qrcode_print(), vec![29, 40, 107, 3, 0, 49, 81, 48]);
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_qrcode() {
        let protocol = Protocol::new(Encoder::default());
        let expected: Vec<Command> = vec![
            [29, 40, 107, 4, 0, 49, 65, 49, 0].to_vec(),
            [29, 40, 107, 3, 0, 49, 67, 4].to_vec(),
            [29, 40, 107, 3, 0, 49, 69, 48].to_vec(),
            [29, 40, 107, 7, 0, 49, 80, 48, 116, 101, 115, 116].to_vec(),
            [29, 40, 107, 3, 0, 49, 81, 48].to_vec(),
        ];
        assert_eq!(
            protocol
                .qrcode(
                    "test",
                    QRCodeOption::new(QRCodeModel::Model1, 4, QRCodeCorrectionLevel::L)
                )
                .unwrap(),
            expected
        );
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_gs1_databar_2d_width() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.gs1_databar_2d_width(GS1DataBar2DWidth::S),
            vec![29, 40, 107, 3, 0, 51, 67, 2]
        );
        assert_eq!(
            protocol.gs1_databar_2d_width(GS1DataBar2DWidth::M),
            vec![29, 40, 107, 3, 0, 51, 67, 1]
        );
        assert_eq!(
            protocol.gs1_databar_2d_width(GS1DataBar2DWidth::L),
            vec![29, 40, 107, 3, 0, 51, 67, 4]
        );
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_gs1_databar_2d_expanded_width() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.gs1_databar_2d_expanded_width(0),
            vec![29, 40, 107, 3, 0, 51, 71, 0, 0]
        );
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_gs1_databar_2d_data() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol
                .gs1_databar_2d_data("8245789658745", GS1DataBar2DOption::default().code_type())
                .unwrap(),
            vec![29, 40, 107, 17, 0, 51, 80, 48, 72, 56, 50, 52, 53, 55, 56, 57, 54, 53, 56, 55, 52, 53]
        );
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_gs1_databar_2d_print() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.gs1_databar_2d_print(), vec![29, 40, 107, 3, 0, 51, 81, 48]);
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_gs1_databar_2d() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol
                .gs1_databar_2d("8245789658745", GS1DataBar2DOption::default())
                .unwrap(),
            vec![
                vec![29, 40, 107, 3, 0, 51, 67, 1],
                vec![29, 40, 107, 3, 0, 51, 71, 0, 0],
                vec![29, 40, 107, 17, 0, 51, 80, 48, 72, 56, 50, 52, 53, 55, 56, 57, 54, 53, 56, 55, 52, 53],
                vec![29, 40, 107, 3, 0, 51, 81, 48]
            ]
        );
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_pdf417_columns() {
        let protocol = Protocol::new(Encoder::default());
        let option = Pdf417Option::new(16, 0, 0, 0, Pdf417Type::default(), Pdf417CorrectionLevel::Level0).unwrap();
        assert_eq!(protocol.pdf417_columns(&option), vec![29, 40, 107, 3, 0, 48, 65, 16]);
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_pdf417_rows() {
        let protocol = Protocol::new(Encoder::default());
        let option = Pdf417Option::new(0, 16, 0, 0, Pdf417Type::default(), Pdf417CorrectionLevel::Level0).unwrap();
        assert_eq!(protocol.pdf417_rows(&option), vec![29, 40, 107, 3, 0, 48, 66, 16]);
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_pdf417_width() {
        let protocol = Protocol::new(Encoder::default());
        let option = Pdf417Option::new(0, 0, 2, 0, Pdf417Type::default(), Pdf417CorrectionLevel::Level0).unwrap();
        assert_eq!(protocol.pdf417_width(&option), vec![29, 40, 107, 3, 0, 48, 67, 2]);
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_pdf417_row_height() {
        let protocol = Protocol::new(Encoder::default());
        let option = Pdf417Option::new(0, 0, 0, 2, Pdf417Type::default(), Pdf417CorrectionLevel::Level0).unwrap();
        assert_eq!(protocol.pdf417_row_height(&option), vec![29, 40, 107, 3, 0, 48, 68, 2]);
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_pdf417_correction_level() {
        let protocol = Protocol::new(Encoder::default());
        let option = Pdf417Option::new(0, 0, 0, 0, Pdf417Type::default(), Pdf417CorrectionLevel::Level5).unwrap();
        assert_eq!(
            protocol.pdf417_correction_level(&option).unwrap(),
            vec![29, 40, 107, 3, 0, 48, 69, 48, 53]
        );

        let option = Pdf417Option::new(0, 0, 0, 0, Pdf417Type::default(), Pdf417CorrectionLevel::Ratio(15)).unwrap();
        assert_eq!(
            protocol.pdf417_correction_level(&option).unwrap(),
            vec![29, 40, 107, 3, 0, 48, 69, 49, 15]
        );

        let option = Pdf417Option::new(0, 0, 0, 0, Pdf417Type::default(), Pdf417CorrectionLevel::Ratio(45)).unwrap();
        assert!(protocol.pdf417_correction_level(&option).is_err());
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_pdf417_type() {
        let protocol = Protocol::new(Encoder::default());

        let option = Pdf417Option::new(0, 0, 0, 0, Pdf417Type::Standard, Pdf417CorrectionLevel::Level5).unwrap();
        assert_eq!(protocol.pdf417_type(&option), vec![29, 40, 107, 3, 0, 48, 70, 0]);

        let option = Pdf417Option::new(0, 0, 0, 0, Pdf417Type::Truncated, Pdf417CorrectionLevel::Level5).unwrap();
        assert_eq!(protocol.pdf417_type(&option), vec![29, 40, 107, 3, 0, 48, 70, 1]);
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_pdf417_data() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.pdf417_data("test").unwrap(),
            vec![29, 40, 107, 7, 0, 48, 80, 48, 116, 101, 115, 116]
        );
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_pdf417_print() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.pdf417_print(), vec![29, 40, 107, 3, 0, 48, 81, 48])
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_pdf417() {
        let protocol = Protocol::new(Encoder::default());
        let option = Pdf417Option::default();
        assert_eq!(
            protocol.pdf417("test", option).unwrap(),
            vec![
                vec![29, 40, 107, 3, 0, 48, 65, 0],
                vec![29, 40, 107, 3, 0, 48, 66, 0],
                vec![29, 40, 107, 3, 0, 48, 67, 0],
                vec![29, 40, 107, 3, 0, 48, 68, 0],
                vec![29, 40, 107, 3, 0, 48, 69, 49, 1],
                vec![29, 40, 107, 3, 0, 48, 70, 0],
                vec![29, 40, 107, 7, 0, 48, 80, 48, 116, 101, 115, 116],
                vec![29, 40, 107, 3, 0, 48, 81, 48]
            ]
        );
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_maxi_code_mode() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.maxi_code_mode(MaxiCodeMode::default()),
            vec![29, 40, 107, 3, 0, 50, 65, 50]
        );
        assert_eq!(
            protocol.maxi_code_mode(MaxiCodeMode::Mode2),
            vec![29, 40, 107, 3, 0, 50, 65, 50]
        );
        assert_eq!(
            protocol.maxi_code_mode(MaxiCodeMode::Mode3),
            vec![29, 40, 107, 3, 0, 50, 65, 51]
        );
        assert_eq!(
            protocol.maxi_code_mode(MaxiCodeMode::Mode4),
            vec![29, 40, 107, 3, 0, 50, 65, 52]
        );
        assert_eq!(
            protocol.maxi_code_mode(MaxiCodeMode::Mode5),
            vec![29, 40, 107, 3, 0, 50, 65, 53]
        );
        assert_eq!(
            protocol.maxi_code_mode(MaxiCodeMode::Mode6),
            vec![29, 40, 107, 3, 0, 50, 65, 54]
        );
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_maxi_code_data() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.maxi_code_data("1245").unwrap(),
            vec![29, 40, 107, 7, 0, 50, 80, 48, 49, 50, 52, 53]
        );
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_maxi_code_print() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.maxi_code_print(), vec![29, 40, 107, 3, 0, 50, 81, 48]);
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_maxi_code() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.maxi_code("1245", MaxiCodeMode::default()).unwrap(),
            vec![
                vec![29, 40, 107, 3, 0, 50, 65, 50],
                vec![29, 40, 107, 7, 0, 50, 80, 48, 49, 50, 52, 53],
                vec![29, 40, 107, 3, 0, 50, 81, 48],
            ]
        );
        assert_eq!(
            protocol.maxi_code("test1245", MaxiCodeMode::default()).unwrap(),
            vec![
                vec![29, 40, 107, 3, 0, 50, 65, 50],
                vec![29, 40, 107, 11, 0, 50, 80, 48, 116, 101, 115, 116, 49, 50, 52, 53],
                vec![29, 40, 107, 3, 0, 50, 81, 48],
            ]
        );
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_data_matrix_type() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.data_matrix_type(DataMatrixType::default()).unwrap(),
            vec![29, 40, 107, 5, 0, 54, 66, 0, 0, 0]
        );
        assert_eq!(
            protocol.data_matrix_type(DataMatrixType::Square(144)).unwrap(),
            vec![29, 40, 107, 5, 0, 54, 66, 0, 144, 144]
        );
        assert_eq!(
            protocol.data_matrix_type(DataMatrixType::Rectangle(8, 0)).unwrap(),
            vec![29, 40, 107, 5, 0, 54, 66, 1, 8, 0]
        );
        assert!(protocol.data_matrix_type(DataMatrixType::Square(2)).is_err());
        assert!(protocol.data_matrix_type(DataMatrixType::Square(145)).is_err());
        assert!(protocol.data_matrix_type(DataMatrixType::Rectangle(0, 0)).is_err());
        assert!(protocol.data_matrix_type(DataMatrixType::Rectangle(16, 32)).is_err());
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_data_matrix_size() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.data_matrix_size(2), vec![29, 40, 107, 3, 0, 54, 67, 2]);
        assert_eq!(protocol.data_matrix_size(16), vec![29, 40, 107, 3, 0, 54, 67, 16]);
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_data_matrix_data() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.data_matrix_data("test123").unwrap(),
            vec![29, 40, 107, 10, 0, 54, 80, 48, 116, 101, 115, 116, 49, 50, 51]
        );
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_data_matrix_print() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.data_matrix_print(), vec![29, 40, 107, 3, 0, 54, 81, 48]);
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_data_matrix() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.data_matrix("test123", DataMatrixOption::default()).unwrap(),
            vec![
                vec![29, 40, 107, 5, 0, 54, 66, 0, 0, 0],
                vec![29, 40, 107, 3, 0, 54, 67, 3],
                vec![29, 40, 107, 10, 0, 54, 80, 48, 116, 101, 115, 116, 49, 50, 51],
                vec![29, 40, 107, 3, 0, 54, 81, 48],
            ]
        );
        let option = DataMatrixOption::new(DataMatrixType::Rectangle(8, 0), 16).unwrap();
        assert_eq!(
            protocol.data_matrix("test123", option).unwrap(),
            vec![
                vec![29, 40, 107, 5, 0, 54, 66, 1, 8, 0],
                vec![29, 40, 107, 3, 0, 54, 67, 16],
                vec![29, 40, 107, 10, 0, 54, 80, 48, 116, 101, 115, 116, 49, 50, 51],
                vec![29, 40, 107, 3, 0, 54, 81, 48],
            ]
        );
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_aztec_mode() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.aztec_mode(AztecMode::default()).unwrap(),
            vec![29, 40, 107, 4, 0, 53, 66, 0, 0]
        );
        assert_eq!(
            protocol.aztec_mode(AztecMode::FullRange(16)).unwrap(),
            vec![29, 40, 107, 4, 0, 53, 66, 0, 16]
        );
        assert_eq!(
            protocol.aztec_mode(AztecMode::Compact(2)).unwrap(),
            vec![29, 40, 107, 4, 0, 53, 66, 1, 2]
        );
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_aztec_size() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.aztec_size(2), vec![29, 40, 107, 3, 0, 53, 67, 2]);
        assert_eq!(protocol.aztec_size(16), vec![29, 40, 107, 3, 0, 53, 67, 16]);
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_aztec_correction_level() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.aztec_correction_level(5), vec![29, 40, 107, 4, 0, 53, 69, 5]);
        assert_eq!(protocol.aztec_correction_level(95), vec![29, 40, 107, 4, 0, 53, 69, 95]);
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_aztec_data() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.aztec_data("test123").unwrap(),
            vec![29, 40, 107, 10, 0, 53, 80, 48, 116, 101, 115, 116, 49, 50, 51]
        );
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_aztec_print() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(protocol.aztec_print(), vec![29, 40, 107, 3, 0, 53, 81, 48]);
    }

    #[cfg(feature = "codes_2d")]
    #[test]
    fn test_aztec() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol.aztec("test123", AztecOption::default()).unwrap(),
            vec![
                vec![29, 40, 107, 4, 0, 53, 66, 0, 0],
                vec![29, 40, 107, 3, 0, 53, 67, 3],
                vec![29, 40, 107, 4, 0, 53, 69, 23],
                vec![29, 40, 107, 10, 0, 53, 80, 48, 116, 101, 115, 116, 49, 50, 51],
                vec![29, 40, 107, 3, 0, 53, 81, 48],
            ]
        );
    }

    #[cfg(feature = "graphics")]
    #[test]
    fn test_bit_image() {
        let protocol = Protocol::new(Encoder::default());
        assert_eq!(
            protocol
                .bit_image(
                    "./resources/images/small.jpg",
                    BitImageOption::new(None, None, BitImageSize::default()).unwrap(),
                )
                .unwrap(),
            vec![
                29, 118, 48, 0, 2, 0, 16, 0, 1, 128, 1, 128, 1, 128, 1, 128, 1, 128, 1, 128, 1, 128, 255, 255, 255,
                255, 1, 128, 1, 128, 1, 128, 1, 128, 1, 128, 1, 128, 1, 128
            ]
        );
    }

    #[cfg(feature = "graphics")]
    #[test]
    fn test_bit_image_from_bytes() {
        let protocol = Protocol::new(Encoder::default());
        let bytes = std::fs::read("./resources/images/small.jpg").unwrap();

        assert_eq!(
            protocol
                .bit_image_from_bytes(
                    &bytes,
                    BitImageOption::new(None, None, BitImageSize::default()).unwrap(),
                )
                .unwrap(),
            vec![
                29, 118, 48, 0, 2, 0, 16, 0, 1, 128, 1, 128, 1, 128, 1, 128, 1, 128, 1, 128, 1, 128, 255, 255, 255,
                255, 1, 128, 1, 128, 1, 128, 1, 128, 1, 128, 1, 128, 1, 128
            ]
        );
    }

    #[cfg(feature = "ui")]
    #[test]
    fn test_draw_line() {
        let protocol = Protocol::new(Encoder::default());
        let line = LineBuilder::new()
            .style(LineStyle::Simple)
            .offset(4)
            .size((2, 2))
            .justify(JustifyMode::LEFT)
            .build();
        let options = PrinterOptions::default();
        let style_state = PrinterStyleState::default();
        assert_eq!(
            protocol.draw_line(line, options, style_state).unwrap(),
            vec![
                vec![29, 33, 17],
                vec![27, 97, 0],
                vec![32, 32, 32, 32, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45],
                vec![27, 100, 1],
                vec![29, 33, 0],
                vec![27, 97, 0],
            ]
        );
    }

    // #[cfg(feature = "graphics")]
    // #[test]
    // fn test_graphic_density() {
    //     let protocol = Protocol::new(Encoder::default());
    //     assert_eq!(
    //         protocol.graphic_density(GraphicDensity::Low),
    //         vec![29, 40, 76, 4, 0, 48, 49, 50, 50]
    //     );
    //     assert_eq!(
    //         protocol.graphic_density(GraphicDensity::High),
    //         vec![29, 40, 76, 4, 0, 48, 49, 51, 51]
    //     );
    // }

    // #[cfg(feature = "graphics")]
    // #[test]
    // fn test_graphic_print() {
    //     let protocol = Protocol::new(Encoder::default());
    //     assert_eq!(protocol.graphic_print(), vec![29, 40, 76, 2, 0, 48, 50]);
    // }

    // #[cfg(feature = "graphics")]
    // #[test]
    // fn test_graphic_data() {
    //     let protocol = Protocol::new(Encoder::default());
    //     assert_eq!(
    //         protocol.graphic_data("./resources/images/rust-logo-small.png").unwrap(),
    //         vec![29, 40, 76, 48, 1, 1, 49, 200, 0, 200, 0]
    //     );
    // }
}

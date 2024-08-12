//! Printer

use super::errors::Result;
use crate::domain::ui::line::Line;
use crate::printer_options::PrinterOptions;
use crate::{domain::*, driver::Driver, utils::Protocol};
use log::debug;

/// Printer
///
/// Print a document
///
/// # Example
///
/// ```rust
/// use escpos::printer::Printer;
/// use escpos::utils::*;
/// use escpos::{driver::*, errors::Result};
///
/// fn main() -> Result<()> {
///     let driver = ConsoleDriver::open(false);
///     let mut printer = Printer::new(driver, Protocol::default(), None);
///     printer.init()?
///         .debug_mode(Some(DebugMode::Dec))
///         .writeln("My example")?
///         .print_cut()?;
///
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct Printer<D: Driver> {
    driver: D,
    protocol: Protocol,
    options: PrinterOptions,
    instructions: Vec<Instruction>,
    style_state: PrinterStyleState,
}

impl<D: Driver> Printer<D> {
    /// Create a new `Printer`
    ///
    /// If no printer options are provided, the default options are used.
    ///
    /// # Example
    ///
    /// ```rust
    /// use escpos::printer::Printer;
    /// use escpos::utils::*;
    /// use escpos::{driver::*, errors::Result};
    ///
    /// fn main() -> Result<()> {
    ///     let driver = ConsoleDriver::open(false);
    ///     Printer::new(driver, Protocol::default(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(driver: D, protocol: Protocol, options: Option<PrinterOptions>) -> Self {
        Self {
            driver,
            protocol,
            options: options.unwrap_or_default(),
            instructions: vec![],
            style_state: PrinterStyleState::default(),
        }
    }

    /// Get the printer protocol
    pub fn protocol(&self) -> &Protocol {
        &self.protocol
    }

    /// Get the printer options
    pub fn options(&self) -> &PrinterOptions {
        &self.options
    }

    /// Get the printer style state
    ///
    /// # Examples
    /// ```
    /// use escpos::printer::{Printer, PrinterStyleState};
    /// use escpos::utils::*;
    /// use escpos::{driver::*, errors::Result};
    ///
    /// fn main() -> Result<()> {
    /// let driver = ConsoleDriver::open(false);
    ///     let mut printer = Printer::new(driver, Protocol::default(), None);
    ///     printer.bold(true)?
    ///         .flip(true)?
    ///         .font(Font::B)?;
    ///
    ///     let style_state = printer.style_state();
    ///
    ///     assert!(style_state.bold);
    ///     assert!(style_state.flip);
    ///     assert!(!style_state.reverse);
    ///     assert!(!style_state.double_strike);
    ///     assert_eq!(style_state.font, Font::B);
    ///     assert_eq!(style_state.justify_mode, JustifyMode::default());
    ///     assert_eq!(style_state.underline_mode, UnderlineMode::default());
    ///     assert_eq!(style_state.text_size, (1, 1));
    ///
    ///     printer.print()?;
    ///
    ///     // Style state is reset after flushing the buffer
    ///     assert_eq!(printer.style_state(), PrinterStyleState::default());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn style_state(&self) -> PrinterStyleState {
        self.style_state.clone()
    }

    /// Reset the printer style state
    pub fn reset_style_state(&mut self) -> &mut Self {
        self.style_state = PrinterStyleState::default();
        self
    }

    /// Flush the buffer, reset the style state and clean the instructions
    fn flush(&mut self) -> Result<&mut Self> {
        for instruction in self.instructions.iter() {
            self.driver.write(&instruction.flatten_commands())?
        }
        self.driver.flush()?;
        self.instructions = vec![];
        self.reset_style_state();

        Ok(self)
    }

    /// Set debug mode
    pub fn debug_mode(&mut self, mode: Option<DebugMode>) -> &mut Self {
        self.options.debug_mode(mode);
        self
    }

    /// Display logs of instructions if debug mode is enabled
    pub fn debug(&mut self) -> Result<&mut Self> {
        if self.options.get_debug_mode().is_some() {
            debug!("instructions = {:#?}", self.instructions);
        }

        Ok(self)
    }

    /// Print the data
    ///
    /// All the instructions are sent at the same time to avoid printing partial data
    /// if an error occurred before the `print` command.
    pub fn print(&mut self) -> Result<&mut Self> {
        self.flush()?;

        if self.options.get_debug_mode().is_some() {
            debug!("[print]");
        }

        Ok(self)
    }

    /// Add command to instructions, write data and display debug information
    fn command(&mut self, label: &str, cmd: &[Command]) -> Result<&mut Self> {
        let instruction = Instruction::new(label, cmd, self.options.get_debug_mode());

        if !label.is_empty() && self.options.get_debug_mode().is_some() {
            debug!("{:?}", instruction.clone());
        }

        self.instructions.push(instruction);

        Ok(self)
    }

    /// Hardware initialization
    pub fn init(&mut self) -> Result<&mut Self> {
        let cmd = self.protocol.init();
        self.command("initialization", &[cmd])?;

        // Set page code
        if let Some(page_code) = self.options.get_page_code() {
            let cmd = self.protocol.page_code(page_code);
            self.command("character page code", &[cmd])?;
        }

        Ok(self)
    }

    /// Hardware reset
    pub fn reset(&mut self) -> Result<&mut Self> {
        let cmd = self.protocol.reset();
        self.command("reset", &[cmd])
    }

    /// Paper full cut
    pub fn cut(&mut self) -> Result<&mut Self> {
        let cmd = self.protocol.cut(false);
        self.command("full paper cut", &[cmd])
    }

    /// Paper partial cut
    pub fn partial_cut(&mut self) -> Result<&mut Self> {
        let cmd = self.protocol.cut(true);
        self.command("partial paper cut", &[cmd])
    }

    /// Print and paper full cut
    pub fn print_cut(&mut self) -> Result<&mut Self> {
        let cmd = self.protocol.cut(false);
        self.command("full paper cut", &[cmd])?.print()
    }

    /// Character page code
    pub fn page_code(&mut self, code: PageCode) -> Result<&mut Self> {
        self.options.page_code(Some(code));

        let cmd = self.protocol.page_code(code);
        self.command("character page code", &[cmd])
    }

    /// International character set
    pub fn character_set(&mut self, code: CharacterSet) -> Result<&mut Self> {
        let cmd = self.protocol.character_set(code);
        self.command("international character set", &[cmd])
    }

    /// Text bold
    pub fn bold(&mut self, enabled: bool) -> Result<&mut Self> {
        let cmd = self.protocol.bold(enabled);
        self.style_state.bold = enabled;
        self.command("text bold", &[cmd])
    }

    /// Text underline
    pub fn underline(&mut self, mode: UnderlineMode) -> Result<&mut Self> {
        let cmd = self.protocol.underline(mode);
        self.style_state.underline_mode = mode;
        self.command("text underline", &[cmd])
    }

    /// Text double strike
    pub fn double_strike(&mut self, enabled: bool) -> Result<&mut Self> {
        let cmd = self.protocol.double_strike(enabled);
        self.style_state.double_strike = enabled;
        self.command("text double strike", &[cmd])
    }

    /// Text font
    pub fn font(&mut self, font: Font) -> Result<&mut Self> {
        let cmd = self.protocol.font(font);
        self.style_state.font = font;
        self.command("text font", &[cmd])
    }

    /// Text flip
    pub fn flip(&mut self, enabled: bool) -> Result<&mut Self> {
        let cmd = self.protocol.flip(enabled);
        self.style_state.flip = enabled;
        self.command("text flip", &[cmd])
    }

    /// Text justify
    pub fn justify(&mut self, mode: JustifyMode) -> Result<&mut Self> {
        let cmd = self.protocol.justify(mode);
        self.style_state.justify_mode = mode;
        self.command("text justify", &[cmd])
    }

    /// Text reverse colour
    pub fn reverse(&mut self, enabled: bool) -> Result<&mut Self> {
        let cmd = self.protocol.reverse_colours(enabled);
        self.style_state.reverse = enabled;
        self.command("text reverse colour", &[cmd])
    }

    /// Text size
    pub fn size(&mut self, width: u8, height: u8) -> Result<&mut Self> {
        let cmd = self.protocol.text_size(width, height)?;
        self.style_state.text_size = (width, height);
        self.command("text size", &[cmd])
    }

    /// Reset text size
    pub fn reset_size(&mut self) -> Result<&mut Self> {
        let cmd = self.protocol.text_size(1, 1)?;
        self.style_state.text_size = (1, 1);
        self.command("text size", &[cmd])
    }

    /// Smoothing mode
    pub fn smoothing(&mut self, enabled: bool) -> Result<&mut Self> {
        let cmd = self.protocol.smoothing(enabled);
        self.command("smoothing mode", &[cmd])
    }

    /// Line feed
    pub fn feed(&mut self) -> Result<&mut Self> {
        let cmd = self.protocol.feed(1);
        self.command("line feed", &[cmd])
    }

    /// Custom line feed
    pub fn feeds(&mut self, lines: u8) -> Result<&mut Self> {
        let cmd = self.protocol.feed(lines);
        self.command("line feeds", &[cmd])
    }

    /// Line spacing
    pub fn line_spacing(&mut self, value: u8) -> Result<&mut Self> {
        let cmd = self.protocol.line_spacing(value);
        self.command("line spacing", &[cmd])
    }

    /// Reset line spacing
    pub fn reset_line_spacing(&mut self) -> Result<&mut Self> {
        let cmd = self.protocol.reset_line_spacing();
        self.command("reset line spacing", &[cmd])
    }

    /// Upside-down mode
    pub fn upside_down(&mut self, enabled: bool) -> Result<&mut Self> {
        let cmd = self.protocol.upside_down(enabled);
        self.command("upside-down mode", &[cmd])
    }

    /// Cash drawer
    pub fn cash_drawer(&mut self, pin: CashDrawer) -> Result<&mut Self> {
        let cmd = self.protocol.cash_drawer(pin);
        self.command("cash drawer", &[cmd])
    }

    /// Text
    pub fn write(&mut self, text: &str) -> Result<&mut Self> {
        let cmd = self.protocol.text(text, self.options.get_page_code())?;
        self.command("text", &[cmd])
    }

    /// Text + Line feed
    pub fn writeln(&mut self, text: &str) -> Result<&mut Self> {
        self.write(text)?.feed()
    }

    /// Custom command
    ///
    /// ```rust
    /// use escpos::printer::Printer;
    /// use escpos::utils::*;
    /// use escpos::{driver::*, errors::Result};
    ///
    /// const EURO: &[u8] = &[0xD5]; // '€' in code page PC858
    ///
    /// fn main() -> Result<()> {
    ///     let driver = ConsoleDriver::open(false);
    ///     Printer::new(driver, Protocol::default(), None)
    ///         .init()?
    ///         .page_code(PageCode::PC858)?
    ///         .custom(EURO)?
    ///         .feed()?
    ///         .print_cut()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn custom(&mut self, cmd: &[u8]) -> Result<&mut Self> {
        self.command("custom command", &[cmd.to_vec()])
    }

    /// Custom command with page code
    ///
    /// ```rust
    /// use escpos::printer::Printer;
    /// use escpos::utils::*;
    /// use escpos::{driver::*, errors::Result};
    ///
    /// const EURO: &[u8] = &[0xD5]; // '€' in code page PC858
    ///
    /// fn main() -> Result<()> {
    ///     let driver = ConsoleDriver::open(false);
    ///     Printer::new(driver, Protocol::default(), None)
    ///         .init()?
    ///         .custom_with_page_code(EURO, PageCode::PC858)?
    ///         .page_code(PageCode::PC437)?
    ///         .writeln("My test")?
    ///         .print_cut()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn custom_with_page_code(&mut self, cmd: &[u8], page_code: PageCode) -> Result<&mut Self> {
        self.page_code(page_code)?;
        self.command(
            &format!("custom command width page code {}", page_code),
            &[cmd.to_vec()],
        )
    }

    /// Set horizontal and vertical motion units
    pub fn motion_units(&mut self, x: u8, y: u8) -> Result<&mut Self> {
        let cmd = self.protocol.motion_units(x, y);
        self.command("set motion units", &[cmd])
    }

    /// Ask printer to send real-time status
    pub fn real_time_status(&mut self, status: RealTimeStatusRequest) -> Result<&mut Self> {
        let cmd = self.protocol.real_time_status(status);
        self.command("real-time status", &[cmd])
    }

    /// Send printer status commands
    pub fn send_status(&mut self) -> Result<&mut Self> {
        self.flush()?;

        if self.options.get_debug_mode().is_some() {
            debug!("[send printer status]");
        }

        Ok(self)
    }

    #[cfg(feature = "barcodes")]
    /// Print barcode
    fn barcode(&mut self, barcode: Barcode) -> Result<&mut Self> {
        let commands = self.protocol.barcode(&barcode.data, barcode.system, barcode.option)?;
        self.command(&format!("print {} barcode", barcode.system), commands.as_slice())
    }

    #[cfg(feature = "barcodes")]
    /// Print EAN13 barcode with default option
    pub fn ean13(&mut self, data: &str) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::EAN13, data, BarcodeOption::default())?)
    }

    #[cfg(feature = "barcodes")]
    /// Print EAN13 barcode with option
    pub fn ean13_option(&mut self, data: &str, option: BarcodeOption) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::EAN13, data, option)?)
    }

    #[cfg(feature = "barcodes")]
    /// Print EAN8 barcode with default option
    pub fn ean8(&mut self, data: &str) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::EAN8, data, BarcodeOption::default())?)
    }

    #[cfg(feature = "barcodes")]
    /// Print EAN8 barcode with option
    pub fn ean8_option(&mut self, data: &str, option: BarcodeOption) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::EAN8, data, option)?)
    }

    #[cfg(feature = "barcodes")]
    /// Print UPC-A barcode with default option
    pub fn upca(&mut self, data: &str) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::UPCA, data, BarcodeOption::default())?)
    }

    #[cfg(feature = "barcodes")]
    /// Print UPC-A barcode with option
    pub fn upca_option(&mut self, data: &str, option: BarcodeOption) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::UPCA, data, option)?)
    }

    #[cfg(feature = "barcodes")]
    /// Print UPC-E barcode with default option
    pub fn upce(&mut self, data: &str) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::UPCE, data, BarcodeOption::default())?)
    }

    #[cfg(feature = "barcodes")]
    /// Print UPC-E barcode with option
    pub fn upce_option(&mut self, data: &str, option: BarcodeOption) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::UPCE, data, option)?)
    }

    #[cfg(feature = "barcodes")]
    /// Print CODE 39 barcode with default option
    pub fn code39(&mut self, data: &str) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::CODE39, data, BarcodeOption::default())?)
    }

    #[cfg(feature = "barcodes")]
    /// Print CODE 39 barcode with option
    pub fn code39_option(&mut self, data: &str, option: BarcodeOption) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::CODE39, data, option)?)
    }

    #[cfg(feature = "barcodes")]
    /// Print CODABAR barcode with default option
    pub fn codabar(&mut self, data: &str) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::CODABAR, data, BarcodeOption::default())?)
    }

    #[cfg(feature = "barcodes")]
    /// Print CODABAR barcode with option
    pub fn codabar_option(&mut self, data: &str, option: BarcodeOption) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::CODABAR, data, option)?)
    }

    #[cfg(feature = "barcodes")]
    /// Print ITF barcode with default option
    pub fn itf(&mut self, data: &str) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::ITF, data, BarcodeOption::default())?)
    }

    #[cfg(feature = "barcodes")]
    /// Print ITF barcode with option
    pub fn itf_option(&mut self, data: &str, option: BarcodeOption) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::ITF, data, option)?)
    }

    #[cfg(feature = "codes_2d")]
    /// Construct QR code
    fn qrcode_builder(&mut self, data: &str, option: Option<QRCodeOption>) -> Result<&mut Self> {
        let qrcode = QRCode::new(data, option)?;
        let commands = self.protocol.qrcode(&qrcode.data, qrcode.option)?;
        self.command("print qrcode", commands.as_slice())
    }

    #[cfg(feature = "codes_2d")]
    /// Print QR code with default option
    pub fn qrcode(&mut self, data: &str) -> Result<&mut Self> {
        self.qrcode_builder(data, None)
    }

    #[cfg(feature = "codes_2d")]
    /// Print QR code with option
    pub fn qrcode_option(&mut self, data: &str, option: QRCodeOption) -> Result<&mut Self> {
        self.qrcode_builder(data, Some(option))
    }

    #[cfg(feature = "codes_2d")]
    /// Construct 2D GS1 DataBar with custom option
    pub fn gs1_databar_2d_option(&mut self, data: &str, option: GS1DataBar2DOption) -> Result<&mut Self> {
        let code = GS1DataBar2D::new(data, option)?;
        let commands = self.protocol.gs1_databar_2d(&code.data, code.option)?;
        self.command("print 2D GS1 DataBar", commands.as_slice())
    }

    #[cfg(feature = "codes_2d")]
    /// Construct 2D GS1 DataBar
    pub fn gs1_databar_2d(&mut self, data: &str) -> Result<&mut Self> {
        self.gs1_databar_2d_option(data, GS1DataBar2DOption::default())
    }

    #[cfg(feature = "codes_2d")]
    /// PDF417
    pub fn pdf417_option(&mut self, data: &str, option: Pdf417Option) -> Result<&mut Self> {
        let code = Pdf417::new(data, option);
        let commands = self.protocol.pdf417(&code.data, code.option)?;
        self.command("print PDF417", commands.as_slice())
    }

    #[cfg(feature = "codes_2d")]
    /// PDF417
    pub fn pdf417(&mut self, data: &str) -> Result<&mut Self> {
        let code = Pdf417::new(data, Pdf417Option::default());
        self.pdf417_option(data, code.option)
    }

    #[cfg(feature = "codes_2d")]
    /// MaxiCode
    pub fn maxi_code_option(&mut self, data: &str, mode: MaxiCodeMode) -> Result<&mut Self> {
        let code = MaxiCode::new(data, mode);
        let commands = self.protocol.maxi_code(&code.data, code.mode)?;
        self.command("print MaxiCode", commands.as_slice())
    }

    #[cfg(feature = "codes_2d")]
    /// MaxiCode
    pub fn maxi_code(&mut self, data: &str) -> Result<&mut Self> {
        let code = MaxiCode::new(data, MaxiCodeMode::default());
        self.maxi_code_option(data, code.mode)
    }

    #[cfg(feature = "codes_2d")]
    /// DataMatrix
    pub fn data_matrix_option(&mut self, data: &str, option: DataMatrixOption) -> Result<&mut Self> {
        let code = DataMatrix::new(data, option);
        let commands = self.protocol.data_matrix(&code.data, code.option)?;
        self.command("print DataMatrix", commands.as_slice())
    }

    #[cfg(feature = "codes_2d")]
    /// DataMatrix
    pub fn data_matrix(&mut self, data: &str) -> Result<&mut Self> {
        let code = DataMatrix::new(data, DataMatrixOption::default());
        self.data_matrix_option(data, code.option)
    }

    #[cfg(feature = "codes_2d")]
    /// Aztec code
    pub fn aztec_option(&mut self, data: &str, option: AztecOption) -> Result<&mut Self> {
        let code = Aztec::new(data, option);
        let commands = self.protocol.aztec(&code.data, code.option)?;
        self.command("print Aztec", commands.as_slice())
    }

    #[cfg(feature = "codes_2d")]
    /// Aztec code
    pub fn aztec(&mut self, data: &str) -> Result<&mut Self> {
        let code = Aztec::new(data, AztecOption::default());
        self.aztec_option(data, code.option)
    }

    #[cfg(feature = "graphics")]
    /// Print image
    pub fn bit_image_option(&mut self, path: &str, option: BitImageOption) -> Result<&mut Self> {
        let cmd = self.protocol.cancel();
        self.command("cancel data", &[cmd])?;

        let cmd = self.protocol.bit_image(path, option)?;
        self.command("print bit image", &[cmd])
    }

    #[cfg(feature = "graphics")]
    /// Print image
    pub fn bit_image(&mut self, path: &str) -> Result<&mut Self> {
        self.bit_image_option(path, BitImageOption::default())
    }

    #[cfg(feature = "graphics")]
    /// Print image
    pub fn bit_image_from_bytes_option(&mut self, bytes: &[u8], option: BitImageOption) -> Result<&mut Self> {
        let cmd = self.protocol.cancel();
        self.command("cancel data", &[cmd])?;

        let cmd = self.protocol.bit_image_from_bytes(bytes, option)?;
        self.command("print bit image from bytes", &[cmd])
    }

    #[cfg(feature = "graphics")]
    /// Print image
    pub fn bit_image_from_bytes(&mut self, bytes: &[u8]) -> Result<&mut Self> {
        self.bit_image_from_bytes_option(bytes, BitImageOption::default())
    }

    #[cfg(feature = "ui")]
    /// Print image
    pub fn draw_line(&mut self, line: Line) -> Result<&mut Self> {
        let commands = self
            .protocol
            .draw_line(line, self.options.clone(), self.style_state.clone())?;
        self.command("draw line", commands.as_slice())
    }

    // #[cfg(feature = "graphics")]
    // /// Print image
    // fn _image(&mut self, path: &str) -> Result<&mut Self> {
    //     let cmd = self.protocol.graphic_density(GraphicDensity::Low);
    //     self.command("set graphic density", cmd)?;
    //
    //     let cmd = self.protocol.graphic_data(path)?;
    //     self.command("set graphic data", cmd)?;
    //
    //     // Print
    //     let cmd = self.protocol.graphic_print();
    //     self.command("print graphic", cmd)
    // }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrinterStyleState {
    pub text_size: (u8, u8),
    pub justify_mode: JustifyMode,
    pub font: Font,
    pub underline_mode: UnderlineMode,
    pub bold: bool,
    pub double_strike: bool,
    pub reverse: bool,
    pub flip: bool,
}

impl Default for PrinterStyleState {
    fn default() -> Self {
        Self {
            text_size: (1, 1),
            justify_mode: JustifyMode::default(),
            font: Font::default(),
            underline_mode: UnderlineMode::default(),
            bold: false,
            double_strike: false,
            reverse: false,
            flip: false,
        }
    }
}

impl PrinterStyleState {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::ConsoleDriver;

    #[test]
    fn test_command() {
        let driver = ConsoleDriver::open(false);
        let debug_mode = None;
        let mut printer = Printer::new(driver, Protocol::default(), None);
        printer.debug_mode(debug_mode).init().unwrap();
        let cmd = printer.protocol.cut(false);
        let printer = printer.command("test paper cut", &[cmd]).unwrap();

        let expected = vec![
            Instruction::new("initialization", &[vec![27, 64]], debug_mode),
            Instruction::new("test paper cut", &[vec![29, 86, 65, 0]], debug_mode),
        ];

        assert_eq!(printer.instructions, expected);
    }
}

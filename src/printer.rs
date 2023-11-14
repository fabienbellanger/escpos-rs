//! Printer

use super::errors::Result;
use crate::{domain::*, driver::Driver, utils::Protocol};
use log::debug;

/// Printer
///
/// Print a document
///
/// # Examples
///
/// ```rust
/// use escpos::printer::Printer;
/// use escpos::utils::*;
/// use escpos::{driver::*, errors::Result};
///
/// fn main() -> Result<()> {
///     let driver = ConsoleDriver::open(false);
///     let mut printer = Printer::new(driver, Protocol::default());
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
    instructions: Vec<Instruction>,
    debug_mode: Option<DebugMode>,
}

impl<D: Driver> Printer<D> {
    /// Create a new `Printer`
    ///
    /// ```rust
    /// use escpos::printer::Printer;
    /// use escpos::utils::*;
    /// use escpos::{driver::*, errors::Result};
    ///
    /// fn main() -> Result<()> {
    ///     let driver = ConsoleDriver::open(false);
    ///     Printer::new(driver, Protocol::default());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(driver: D, protocol: Protocol) -> Self {
        Self {
            driver,
            protocol,
            instructions: vec![],
            debug_mode: None,
        }
    }

    /// Set debug mode
    pub fn debug_mode(&mut self, mode: Option<DebugMode>) -> &mut Self {
        self.debug_mode = mode;
        self
    }

    /// Display logs of instructions if debug mode is enabled
    pub fn debug(&mut self) -> Result<&mut Self> {
        if self.debug_mode.is_some() {
            debug!("instructions = {:#?}", self.instructions);
        }

        Ok(self)
    }

    /// Print the data
    ///
    /// All the instructions are sent at the same time to avoid printing partial data
    /// if an error occurred before the `print` command.
    pub fn print(&mut self) -> Result<&mut Self> {
        for instruction in self.instructions.iter() {
            self.driver.write(&instruction.flatten_commands())?
        }
        self.driver.flush()?;
        self.instructions = vec![];

        if self.debug_mode.is_some() {
            debug!("[print]");
        }

        Ok(self)
    }

    /// Add command to instructions, write data and display debug information
    fn command(&mut self, label: &str, cmd: &[Command]) -> Result<&mut Self> {
        let instruction = Instruction::new(label, cmd, self.debug_mode);

        if !label.is_empty() && self.debug_mode.is_some() {
            debug!("{:?}", instruction.clone());
        }

        self.instructions.push(instruction);

        Ok(self)
    }

    /// Hardware initialization
    pub fn init(&mut self) -> Result<&mut Self> {
        let cmd = self.protocol.init();
        self.command("initialization", &[cmd])
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
        self.command("text bold", &[cmd])
    }

    /// Text underline
    pub fn underline(&mut self, mode: UnderlineMode) -> Result<&mut Self> {
        let cmd = self.protocol.underline(mode);
        self.command("text underline", &[cmd])
    }

    /// Text double strike
    pub fn double_strike(&mut self, enabled: bool) -> Result<&mut Self> {
        let cmd = self.protocol.double_strike(enabled);
        self.command("text double strike", &[cmd])
    }

    /// Text font
    pub fn font(&mut self, font: Font) -> Result<&mut Self> {
        let cmd = self.protocol.font(font);
        self.command("text font", &[cmd])
    }

    /// Text flip
    pub fn flip(&mut self, enabled: bool) -> Result<&mut Self> {
        let cmd = self.protocol.flip(enabled);
        self.command("text flip", &[cmd])
    }

    /// Text justify
    pub fn justify(&mut self, mode: JustifyMode) -> Result<&mut Self> {
        let cmd = self.protocol.justify(mode);
        self.command("text justify", &[cmd])
    }

    /// Text reverse colour
    pub fn reverse(&mut self, enabled: bool) -> Result<&mut Self> {
        let cmd = self.protocol.reverse_colours(enabled);
        self.command("text reverse colour", &[cmd])
    }

    /// Text size
    pub fn size(&mut self, width: u8, height: u8) -> Result<&mut Self> {
        let cmd = self.protocol.text_size(width, height)?;
        self.command("text size", &[cmd])
    }

    /// Reset text size
    pub fn reset_size(&mut self) -> Result<&mut Self> {
        let cmd = self.protocol.text_size(1, 1)?;
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
        let cmd = self.protocol.text(text)?;
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
    ///     Printer::new(driver, Protocol::default())
    ///         .init()?
    ///         .page_code(PageCode::PC858)?
    ///         .custom(EURO)?
    ///         .page_code(PageCode::PC437)?
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
    ///     Printer::new(driver, Protocol::default())
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

    #[cfg(feature = "barcodes")]
    /// Print barcode
    fn barcode(&mut self, barcode: Barcode) -> Result<&mut Self> {
        let commands = self.protocol.barcode(
            &barcode.data,
            barcode.system,
            barcode.option.width.into(),
            barcode.option.height.into(),
            barcode.option.font,
            barcode.option.position,
        )?;
        self.command(&format!("print {} barcode", barcode.system), commands.as_slice())
    }

    #[cfg(feature = "barcodes")]
    /// Print EAN13 barcode with default option
    pub fn ean13(&mut self, data: &str) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::EAN13, data, None)?)
    }

    #[cfg(feature = "barcodes")]
    /// Print EAN13 barcode with option
    pub fn ean13_option(&mut self, data: &str, option: BarcodeOption) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::EAN13, data, Some(option))?)
    }

    #[cfg(feature = "barcodes")]
    /// Print EAN8 barcode with default option
    pub fn ean8(&mut self, data: &str) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::EAN8, data, None)?)
    }

    #[cfg(feature = "barcodes")]
    /// Print EAN8 barcode with option
    pub fn ean8_option(&mut self, data: &str, option: BarcodeOption) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::EAN8, data, Some(option))?)
    }

    #[cfg(feature = "barcodes")]
    /// Print UPC-A barcode with default option
    pub fn upca(&mut self, data: &str) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::UPCA, data, None)?)
    }

    #[cfg(feature = "barcodes")]
    /// Print UPC-A barcode with option
    pub fn upca_option(&mut self, data: &str, option: BarcodeOption) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::UPCA, data, Some(option))?)
    }

    #[cfg(feature = "barcodes")]
    /// Print UPC-E barcode with default option
    pub fn upce(&mut self, data: &str) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::UPCE, data, None)?)
    }

    #[cfg(feature = "barcodes")]
    /// Print UPC-E barcode with option
    pub fn upce_option(&mut self, data: &str, option: BarcodeOption) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::UPCE, data, Some(option))?)
    }

    #[cfg(feature = "barcodes")]
    /// Print CODE 39 barcode with default option
    pub fn code39(&mut self, data: &str) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::CODE39, data, None)?)
    }

    #[cfg(feature = "barcodes")]
    /// Print CODE 39 barcode with option
    pub fn code39_option(&mut self, data: &str, option: BarcodeOption) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::CODE39, data, Some(option))?)
    }

    #[cfg(feature = "barcodes")]
    /// Print CODABAR barcode with default option
    pub fn codabar(&mut self, data: &str) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::CODABAR, data, None)?)
    }

    #[cfg(feature = "barcodes")]
    /// Print CODABAR barcode with option
    pub fn codabar_option(&mut self, data: &str, option: BarcodeOption) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::CODABAR, data, Some(option))?)
    }

    #[cfg(feature = "barcodes")]
    /// Print ITF barcode with default option
    pub fn itf(&mut self, data: &str) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::ITF, data, None)?)
    }

    #[cfg(feature = "barcodes")]
    /// Print ITF barcode with option
    pub fn itf_option(&mut self, data: &str, option: BarcodeOption) -> Result<&mut Self> {
        self.barcode(Barcode::new(BarcodeSystem::ITF, data, Some(option))?)
    }

    #[cfg(feature = "codes_2d")]
    /// Construct QR code
    fn qrcode_builder(&mut self, data: &str, option: Option<QRCodeOption>) -> Result<&mut Self> {
        let qrcode = QRCode::new(data, option)?;

        let commands = self.protocol.qrcode(
            &qrcode.data,
            qrcode.option.model,
            qrcode.option.correction_level,
            qrcode.option.size,
        )?;
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

    #[cfg(feature = "graphics")]
    /// Print image
    pub fn bit_image_option(&mut self, path: &str, option: BitImageOption) -> Result<&mut Self> {
        let cmd = self.protocol.cancel();
        self.command("cancel data", &[cmd])?;

        let cmd = self.protocol.bit_image(path, Some(option))?;
        self.command("print bit image", &[cmd])
    }

    #[cfg(feature = "graphics")]
    /// Print image
    pub fn bit_image(&mut self, path: &str) -> Result<&mut Self> {
        let cmd = self.protocol.cancel();
        self.command("cancel data", &[cmd])?;

        let cmd = self.protocol.bit_image(path, None)?;
        self.command("print bit image", &[cmd])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::ConsoleDriver;

    #[test]
    fn test_command() {
        let driver = ConsoleDriver::open(false);
        let debug_mode = None;
        let mut printer = Printer::new(driver, Protocol::default());
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

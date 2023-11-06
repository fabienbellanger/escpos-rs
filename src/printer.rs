//! Printer

use super::errors::Result;
use crate::{domain::*, driver::Driver, utils::protocol::Protocol};
use log::debug;

pub struct Printer<D: Driver> {
    driver: D,
    protocol: Protocol,
    instructions: Vec<Instruction>,
    debug_mode: Option<DebugMode>,
}

impl<D: Driver> Printer<D> {
    /// Create a new `Printer`
    pub fn new(driver: D, protocol: Protocol) -> Self {
        Self {
            driver,
            protocol,
            instructions: vec![],
            debug_mode: None,
        }
    }

    /// Set debug mode
    pub fn debug_mode(mut self, mode: Option<DebugMode>) -> Self {
        self.debug_mode = mode;
        self
    }

    /// Add command to instructions, write data and display debug information
    // TODO: Add unit test
    fn command(mut self, label: &str, cmd: Command) -> Result<Self> {
        let instruction = Instruction::new(label, &cmd, self.debug_mode);

        if self.debug_mode.is_some() {
            debug!("{:?}", instruction.clone());
        }

        self.instructions.push(instruction);
        self.driver.write(&cmd)?;

        Ok(self)
    }

    /// Display logs of instructions if debug mode is enabled
    pub fn debug(self) -> Self {
        if self.debug_mode.is_some() {
            debug!("[debug] instructions={:?}", self.instructions);
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
    pub fn size(self, width: u8, height: u8) -> Result<Self> {
        let cmd = self.protocol.text_size(width, height)?;
        self.command("text size", cmd)
    }

    /// Reset text size
    pub fn reset_size(self) -> Result<Self> {
        let cmd = self.protocol.text_size(1, 1)?;
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
    pub fn write(self, text: &str) -> Result<Self> {
        let cmd = self.protocol.text(text)?;
        self.command("text", cmd)
    }

    /// Text + Line feed
    pub fn writeln(self, text: &str) -> Result<Self> {
        self.write(text)?.feed()
    }

    /// Set horizontal and vertical motion units
    pub fn motion_units(self, x: u8, y: u8) -> Result<Self> {
        let cmd = self.protocol.motion_units(x, y);
        self.command("set motion units", cmd)
    }

    #[cfg(feature = "barcode")]
    /// Print barcode
    fn barcode(mut self, barcode: Barcode) -> Result<Self> {
        // Width
        let cmd = self.protocol.barcode_width(barcode.option.width.into())?;
        self = self.command("set barcode width", cmd)?;

        // Height
        let cmd = self.protocol.barcode_height(barcode.option.height.into())?;
        self = self.command("set barcode height", cmd)?;

        // Font
        let cmd = self.protocol.barcode_font(barcode.option.font);
        self = self.command("set barcode font", cmd)?;

        // Position
        let cmd = self.protocol.barcode_position(barcode.option.position);
        self = self.command("set barcode position", cmd)?;

        // Print
        let cmd = self.protocol.barcode_print(barcode.system, &barcode.data);
        self.command(&format!("print {} barcode", barcode.system), cmd)
    }

    #[cfg(feature = "barcode")]
    /// Print EAN13 barcode with default option
    pub fn ean13(self, data: &str) -> Result<Self> {
        self.barcode(Barcode::new(BarcodeSystem::EAN13, data, None)?)
    }

    #[cfg(feature = "barcode")]
    /// Print EAN13 barcode with option
    pub fn ean13_option(self, data: &str, option: BarcodeOption) -> Result<Self> {
        self.barcode(Barcode::new(BarcodeSystem::EAN13, data, Some(option))?)
    }

    #[cfg(feature = "barcode")]
    /// Print EAN8 barcode with default option
    pub fn ean8(self, data: &str) -> Result<Self> {
        self.barcode(Barcode::new(BarcodeSystem::EAN8, data, None)?)
    }

    #[cfg(feature = "barcode")]
    /// Print EAN8 barcode with option
    pub fn ean8_option(self, data: &str, option: BarcodeOption) -> Result<Self> {
        self.barcode(Barcode::new(BarcodeSystem::EAN8, data, Some(option))?)
    }

    #[cfg(feature = "barcode")]
    /// Print UPC-A barcode with default option
    pub fn upca(self, data: &str) -> Result<Self> {
        self.barcode(Barcode::new(BarcodeSystem::UPCA, data, None)?)
    }

    #[cfg(feature = "barcode")]
    /// Print UPC-A barcode with option
    pub fn upca_option(self, data: &str, option: BarcodeOption) -> Result<Self> {
        self.barcode(Barcode::new(BarcodeSystem::UPCA, data, Some(option))?)
    }

    #[cfg(feature = "barcode")]
    /// Print UPC-E barcode with default option
    pub fn upce(self, data: &str) -> Result<Self> {
        self.barcode(Barcode::new(BarcodeSystem::UPCE, data, None)?)
    }

    #[cfg(feature = "barcode")]
    /// Print UPC-E barcode with option
    pub fn upce_option(self, data: &str, option: BarcodeOption) -> Result<Self> {
        self.barcode(Barcode::new(BarcodeSystem::UPCE, data, Some(option))?)
    }

    #[cfg(feature = "barcode")]
    /// Print CODE 39 barcode with default option
    pub fn code39(self, data: &str) -> Result<Self> {
        self.barcode(Barcode::new(BarcodeSystem::CODE39, data, None)?)
    }

    #[cfg(feature = "barcode")]
    /// Print CODE 39 barcode with option
    pub fn code39_option(self, data: &str, option: BarcodeOption) -> Result<Self> {
        self.barcode(Barcode::new(BarcodeSystem::CODE39, data, Some(option))?)
    }

    #[cfg(feature = "barcode")]
    /// Print CODABAR barcode with default option
    pub fn codabar(self, data: &str) -> Result<Self> {
        self.barcode(Barcode::new(BarcodeSystem::CODABAR, data, None)?)
    }

    #[cfg(feature = "barcode")]
    /// Print CODABAR barcode with option
    pub fn codabar_option(self, data: &str, option: BarcodeOption) -> Result<Self> {
        self.barcode(Barcode::new(BarcodeSystem::CODABAR, data, Some(option))?)
    }

    #[cfg(feature = "barcode")]
    /// Print ITF barcode with default option
    pub fn itf(self, data: &str) -> Result<Self> {
        self.barcode(Barcode::new(BarcodeSystem::ITF, data, None)?)
    }

    #[cfg(feature = "barcode")]
    /// Print ITF barcode with option
    pub fn itf_option(self, data: &str, option: BarcodeOption) -> Result<Self> {
        self.barcode(Barcode::new(BarcodeSystem::ITF, data, Some(option))?)
    }

    #[cfg(feature = "qrcode")]
    /// Print QR code with default option
    pub fn qrcode(mut self, data: &str) -> Result<Self> {
        let qrcode = QRCode::new(data, None)?;

        // Model
        let cmd = self.protocol.qrcode_model(qrcode.option.model);
        self = self.command("set qrcode model", cmd)?;

        // Size
        let cmd = self.protocol.qrcode_size(qrcode.option.size);
        self = self.command("set qrcode size", cmd)?;

        // Error correction level
        let cmd = self.protocol.qrcode_correction_level(qrcode.option.correction_level);
        self = self.command("set qrcode error correction level", cmd)?;

        // Data
        let cmd = self.protocol.qrcode_data(data)?;
        self = self.command("set qrcode data", cmd)?;

        // Print
        let cmd = self.protocol.qrcode_print();
        self.command("print qrcode", cmd)
    }

    #[cfg(feature = "qrcode")]
    /// Print QR code with option
    pub fn qrcode_option(mut self, data: &str, option: QRCodeOption) -> Result<Self> {
        let qrcode = QRCode::new(data, Some(option))?;

        // Model
        let cmd = self.protocol.qrcode_model(qrcode.option.model);
        self = self.command("set qrcode model", cmd)?;

        // Size
        let cmd = self.protocol.qrcode_size(qrcode.option.size);
        self = self.command("set qrcode size", cmd)?;

        // Error correction level
        let cmd = self.protocol.qrcode_correction_level(qrcode.option.correction_level);
        self = self.command("set qrcode error correction level", cmd)?;

        // Data
        let cmd = self.protocol.qrcode_data(data)?;
        self = self.command("set qrcode data", cmd)?;

        // Print
        let cmd = self.protocol.qrcode_print();
        self.command("print qrcode", cmd)
    }

    // #[cfg(feature = "graphics")]
    // /// Print image
    // fn _image(mut self, path: &str) -> Result<Self> {
    //     let cmd = self.protocol.graphic_density(GraphicDensity::Low);
    //     self = self.command("set graphic density", cmd)?;
    //
    //     let cmd = self.protocol.graphic_data(path)?;
    //     self = self.command("set graphic data", cmd)?;
    //
    //     // Print
    //     let cmd = self.protocol.graphic_print();
    //     self.command("print graphic", cmd)
    // }

    #[cfg(feature = "graphics")]
    /// Print image
    pub fn bit_image_option(mut self, path: &str, option: BitImageOption) -> Result<Self> {
        let cmd = self.protocol.cancel();
        self = self.command("cancel data", cmd)?;

        let cmd = self.protocol.bit_image(path, Some(option))?;
        self.command("print bit image", cmd)
    }

    #[cfg(feature = "graphics")]
    /// Print image
    pub fn bit_image(mut self, path: &str) -> Result<Self> {
        let cmd = self.protocol.cancel();
        self = self.command("cancel data", cmd)?;

        let cmd = self.protocol.bit_image(path, None)?;
        self.command("print bit image", cmd)
    }
}

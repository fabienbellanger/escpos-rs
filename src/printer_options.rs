//! Printer options

use crate::domain::{DebugMode, PageCode};

/// Printer options
#[derive(Debug, Clone)]
pub struct PrinterOptions {
    /// Select the code page
    page_code: Option<PageCode>,

    /// Number of characters per line (default: 42)
    characters_per_line: u8,

    /// Enable or disable the debug mode
    debug_mode: Option<DebugMode>,
}

impl Default for PrinterOptions {
    /// Create a default printer options instance
    ///
    /// ```
    /// use escpos::printer_options::PrinterOptions;
    /// use escpos::utils::{DebugMode, PageCode};
    ///
    /// let options = PrinterOptions::default();
    ///
    /// assert_eq!(options.get_page_code(), None);
    /// assert_eq!(options.get_characters_per_line(), 42);
    /// assert_eq!(options.get_debug_mode(), None);
    /// ```
    fn default() -> Self {
        Self {
            page_code: None,
            characters_per_line: 42,
            debug_mode: None,
        }
    }
}

impl PrinterOptions {
    /// Create a new printer options instance
    ///
    /// ```
    /// use escpos::printer_options::PrinterOptions;
    /// use escpos::utils::{DebugMode, PageCode};
    ///
    /// let options = PrinterOptions::new(Some(PageCode::PC437), 44, Some(DebugMode::Hex));
    ///
    /// assert_eq!(options.get_page_code().unwrap(), PageCode::PC437);
    /// assert_eq!(options.get_characters_per_line(), 44);
    /// assert_eq!(options.get_debug_mode().unwrap(), DebugMode::Hex);
    /// ```
    pub fn new(page_code: Option<PageCode>, characters_per_line: u8, debug_mode: Option<DebugMode>) -> Self {
        Self {
            page_code,
            characters_per_line,
            debug_mode,
        }
    }

    /// Get the number of characters per line
    pub fn get_page_code(&self) -> Option<PageCode> {
        self.page_code
    }

    /// Set the code page
    ///
    /// ```
    /// use escpos::printer_options::PrinterOptions;
    /// use escpos::utils::PageCode;
    ///
    /// let mut printer_options = PrinterOptions::default();
    /// printer_options.page_code(Some(PageCode::PC858));
    ///
    /// assert_eq!(printer_options.get_page_code().unwrap(), PageCode::PC858);
    /// ```
    pub fn page_code(&mut self, page_code: Option<PageCode>) {
        self.page_code = page_code;
    }

    /// Get the number of characters per line
    pub fn get_characters_per_line(&self) -> u8 {
        self.characters_per_line
    }

    /// Set the number of characters per line
    ///
    /// ```
    /// use escpos::printer_options::PrinterOptions;
    ///
    /// let mut printer_options = PrinterOptions::default();
    /// printer_options.characters_per_line(48);
    ///
    /// assert_eq!(printer_options.get_characters_per_line(), 48);
    /// ```
    pub fn characters_per_line(&mut self, characters_per_line: u8) {
        self.characters_per_line = characters_per_line;
    }

    /// Get the debug mode
    pub fn get_debug_mode(&self) -> Option<DebugMode> {
        self.debug_mode
    }

    /// Set the debug mode
    ///
    /// ```
    /// use escpos::printer_options::PrinterOptions;
    /// use escpos::utils::DebugMode;
    ///
    /// let mut printer_options = PrinterOptions::default();
    /// printer_options.debug_mode(Some(DebugMode::Dec));
    ///
    /// assert_eq!(printer_options.get_debug_mode().unwrap(), DebugMode::Dec);
    /// ```
    pub fn debug_mode(&mut self, debug_mode: Option<DebugMode>) {
        self.debug_mode = debug_mode;
    }
}

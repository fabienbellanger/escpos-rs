//! UI components like lines, tables, etc.

use crate::domain::Command;
use crate::errors::Result;
use crate::printer::PrinterStyleState;
use crate::printer_options::PrinterOptions;
use crate::utils::Protocol;

pub mod line;

/// UIComponent trait
pub trait UIComponent {
    fn render(
        &self,
        protocol: Protocol,
        options: PrinterOptions,
        style_state: PrinterStyleState,
    ) -> Result<Vec<Command>>;
}

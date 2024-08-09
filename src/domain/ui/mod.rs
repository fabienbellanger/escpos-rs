//! UI components like lines, tables, etc.

use crate::domain::Command;
use crate::driver::Driver;
use crate::errors::Result;
use crate::printer::Printer;

pub mod line;

/// UIComponent trait
pub trait UIComponent {
    fn render<D: Driver>(&self, printer: Printer<D>) -> Result<Vec<Command>>;
}

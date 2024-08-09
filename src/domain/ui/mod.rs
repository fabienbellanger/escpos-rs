//! UI components like lines, tables, etc.

use crate::driver::Driver;
use crate::printer::Printer;

pub mod line;

/// UIComponent trait
pub trait UIComponent {
    fn render<D: Driver>(&self, printer: &mut Printer<D>);
}

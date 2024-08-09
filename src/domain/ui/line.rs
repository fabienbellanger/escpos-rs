//! Line component
// TODO: Add doc + examples
// Problématiques :
// - la taille du text influence la taille de la ligne

use crate::domain::ui::UIComponent;
use crate::domain::{Font, JustifyMode, TextSize, DEFAULT_CHARACTERS_PER_LINE};
use crate::driver::Driver;
use crate::printer::Printer;

/// Line style
#[derive(Debug, Clone, Default, PartialEq)]
pub enum LineStyle {
    /// No line
    Blank,

    /// Simple line with "-" pattern (Ex.: "----------------")
    #[default]
    Simple,

    /// Double line with "=" pattern (Ex.: "================")
    Double,

    /// Dotted line with "." pattern (Ex.: "...............")
    Dotted,

    /// Dashed line with "- " pattern (Ex.: "- - - - - - - -")
    Dashed,

    /// Custom line with a given pattern (Ex.: "################")
    Custom(String),
}

/// Line builder
///
/// The max width is set to 42 by default.
///
/// # Example
/// ```
/// use escpos::utils::{Font, JustifyMode};
/// use escpos::utils::ui::line::{Line, LineBuilder, LineStyle};
///
/// let line = LineBuilder::new(32)
///     .font(Font::A)
///     .size((1, 1))
///     .align(JustifyMode::CENTER)
///     .style(LineStyle::Double)
///     .width(16)
///     .offset(8)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct LineBuilder {
    font: Option<Font>,
    size: Option<TextSize>,
    align: Option<JustifyMode>,
    style: LineStyle,
    width: Option<u8>,
    max_width: u8,
    offset: u8,
}

impl Default for LineBuilder {
    fn default() -> Self {
        Self {
            font: None,
            size: None,
            align: None,
            style: LineStyle::default(),
            width: None,
            max_width: DEFAULT_CHARACTERS_PER_LINE,
            offset: 0,
        }
    }
}

impl LineBuilder {
    /// Initialize a new `LineBuilder`
    pub fn new(max_width: u8) -> Self {
        Self {
            font: None,
            size: None,
            align: None,
            style: LineStyle::default(),
            width: None,
            offset: 0,
            max_width,
        }
    }

    /// Set font
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Set size
    pub fn size(mut self, size: TextSize) -> Self {
        self.size = Some(size);
        self
    }

    /// Set horizontal alignment
    pub fn align(mut self, align: JustifyMode) -> Self {
        self.align = Some(align);
        self
    }

    /// Set line width
    pub fn width(mut self, width: u8) -> Self {
        self.width = Some(width);
        self
    }

    /// Set line offset
    pub fn offset(mut self, offset: u8) -> Self {
        self.offset = offset;
        self
    }

    /// Set style
    pub fn style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }

    /// Build a [line](Line)
    pub fn build(self) -> Line {
        Line {
            font: self.font,
            size: self.size,
            align: self.align,
            style: self.style,
            width: self.width,
            max_width: self.max_width,
            offset: self.offset,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    font: Option<Font>,
    size: Option<TextSize>,
    align: Option<JustifyMode>,
    style: LineStyle,
    width: Option<u8>,
    max_width: u8,
    offset: u8,
}

impl UIComponent for Line {
    fn render<D: Driver>(&self, printer: &mut Printer<D>) {
        let _saved_style_state = printer.style_state().clone();

        // Render line

        // Restore initial style state
    }
}

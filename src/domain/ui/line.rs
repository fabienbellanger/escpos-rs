//! Line component
// TODO: Add doc + examples

use crate::domain::ui::UIComponent;
use crate::domain::{chars_number, Command, Font, JustifyMode, TextSize};
use crate::errors::Result;
use crate::printer::PrinterStyleState;
use crate::printer_options::PrinterOptions;
use crate::utils::Protocol;

/// Line style
#[derive(Debug, Clone, Default, PartialEq)]
pub enum LineStyle<'a> {
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
    Custom(&'a str),
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
/// let line = LineBuilder::new()
///     .font(Font::A)
///     .size((1, 1))
///     .justify(JustifyMode::CENTER)
///     .style(LineStyle::Double)
///     .width(16)
///     .offset(8)
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct LineBuilder<'a> {
    font: Option<Font>,
    size: Option<TextSize>,
    justify: Option<JustifyMode>,
    style: LineStyle<'a>,
    width: Option<u8>,
    offset: u8,
}

impl<'a> LineBuilder<'a> {
    /// Initialize a new `LineBuilder`
    pub fn new() -> Self {
        Self {
            font: None,
            size: None,
            justify: None,
            style: LineStyle::default(),
            width: None,
            offset: 0,
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
    pub fn justify(mut self, align: JustifyMode) -> Self {
        self.justify = Some(align);
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
    pub fn style(mut self, style: LineStyle<'a>) -> Self {
        self.style = style;
        self
    }

    /// Build a [line](Line)
    pub fn build(self) -> Line<'a> {
        Line {
            font: self.font,
            size: self.size,
            justify: self.justify,
            style: self.style,
            width: self.width,
            offset: self.offset,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line<'a> {
    font: Option<Font>,
    size: Option<TextSize>,
    justify: Option<JustifyMode>,
    style: LineStyle<'a>,
    width: Option<u8>,
    offset: u8,
}

impl<'a> Line<'a> {
    /// Set the style for the line
    fn set_style(
        &self,
        protocol: Protocol,
        text_size: &mut TextSize,
        justify_mode: &mut JustifyMode,
        commands: &mut Vec<Command>,
    ) -> Result<()> {
        if let Some(font) = self.font {
            commands.push(protocol.font(font));
        }
        if let Some(size) = self.size {
            *text_size = size;
            commands.push(protocol.text_size(size.0, size.1)?);
        }
        if let Some(justify) = self.justify {
            *justify_mode = justify;
            commands.push(protocol.justify(justify));
        }

        Ok(())
    }

    fn draw(
        &self,
        protocol: Protocol,
        chars_per_line: u8,
        text_size: u8,
        justify_mode: JustifyMode,
        commands: &mut Vec<Command>,
    ) -> Result<()> {
        let line_max_width = chars_number(chars_per_line, text_size)?;
        let line_width = self.width.unwrap_or(line_max_width).min(line_max_width - self.offset);
        let line_pattern = match &self.style {
            LineStyle::Simple => "-",
            LineStyle::Double => "=",
            LineStyle::Dotted => ".",
            LineStyle::Dashed => "- ",
            LineStyle::Custom(pattern) => pattern,
        };
        let mut line = line_pattern.repeat(line_width as usize);

        if self.offset > 0 {
            if justify_mode == JustifyMode::LEFT {
                line = format!("{}{line}", " ".repeat(self.offset as usize));
            } else {
                line = format!("{line}{}", " ".repeat(self.offset as usize));
            }
        }

        if line.as_bytes().len() > line_max_width as usize {
            line = String::from_utf8(line.as_bytes()[..line_max_width as usize].to_vec())?;
        }

        if !line.is_empty() {
            commands.push(protocol.text(line.as_str(), None)?);
            commands.push(protocol.feed(1));
        }

        Ok(())
    }

    /// Restore the initial style state
    fn restore_style(
        &self,
        protocol: Protocol,
        style_state: PrinterStyleState,
        commands: &mut Vec<Command>,
    ) -> Result<()> {
        if self.font.is_some() {
            commands.push(protocol.font(style_state.font));
        }
        if self.size.is_some() {
            commands.push(protocol.text_size(style_state.text_size.0, style_state.text_size.1)?);
        }
        if self.justify.is_some() {
            commands.push(protocol.justify(style_state.justify_mode));
        }

        Ok(())
    }
}

impl<'a> UIComponent for Line<'a> {
    fn render(
        &self,
        protocol: Protocol,
        options: PrinterOptions,
        style_state: PrinterStyleState,
    ) -> Result<Vec<Command>> {
        let chars_per_line = options.get_characters_per_line();
        let mut commands = vec![];
        let mut text_size = style_state.text_size;
        let mut justify_mode = style_state.justify_mode;

        // Set global styles
        self.set_style(protocol.clone(), &mut text_size, &mut justify_mode, &mut commands)?;

        // Draw the line
        self.draw(
            protocol.clone(),
            chars_per_line,
            text_size.0,
            justify_mode,
            &mut commands,
        )?;

        // Restore initial style state
        self.restore_style(protocol, style_state, &mut commands)?;

        Ok(commands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_draw_one_char_pattern() {
        let mut commands = vec![];
        let line = LineBuilder::new().style(LineStyle::Simple).build();
        line.draw(Protocol::default(), 42, 1, JustifyMode::LEFT, &mut commands)
            .unwrap();
        let expected = "-".repeat(42).chars().map(|c| c as u8).collect::<Vec<u8>>();

        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0], expected);

        let mut commands = vec![];
        let line = LineBuilder::new().style(LineStyle::Double).offset(4).width(10).build();
        line.draw(Protocol::default(), 44, 1, JustifyMode::RIGHT, &mut commands)
            .unwrap();
        let expected = format!("{}{}", "=".repeat(10), " ".repeat(4))
            .chars()
            .map(|c| c as u8)
            .collect::<Vec<u8>>();

        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0], expected);
    }
    #[test]
    fn test_line_draw_two_chars_pattern() {
        let mut commands = vec![];
        let line = LineBuilder::new().style(LineStyle::Dashed).build();
        line.draw(Protocol::default(), 42, 1, JustifyMode::LEFT, &mut commands)
            .unwrap();
        let expected = "- ".repeat(21).chars().map(|c| c as u8).collect::<Vec<u8>>();

        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0], expected);

        let mut commands = vec![];
        let line = LineBuilder::new().style(LineStyle::Dashed).offset(3).build();
        line.draw(Protocol::default(), 44, 1, JustifyMode::LEFT, &mut commands)
            .unwrap();
        let expected = format!("{}{}-", " ".repeat(3), "- ".repeat(20))
            .chars()
            .map(|c| c as u8)
            .collect::<Vec<u8>>();

        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0], expected);
    }

    #[test]
    fn test_line_draw_with_size() {
        let mut commands = vec![];
        let line = LineBuilder::new().style(LineStyle::Dotted).size((3, 1)).build();
        line.draw(Protocol::default(), 42, 3, JustifyMode::LEFT, &mut commands)
            .unwrap();
        let expected = ".".repeat(14).chars().map(|c| c as u8).collect::<Vec<u8>>();

        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0], expected);
    }

    #[test]
    fn test_line_draw_with_empty_pattern() {
        let mut commands = vec![];
        let line = LineBuilder::new().style(LineStyle::Custom("")).size((3, 1)).build();
        line.draw(Protocol::default(), 42, 3, JustifyMode::LEFT, &mut commands)
            .unwrap();

        assert!(commands.is_empty());
    }

    #[test]
    fn test_set_style() {
        let mut commands = vec![];
        let mut size = (2, 2); // Style state value
        let mut justify_mode = JustifyMode::LEFT; // Style state value
        let line = LineBuilder::new()
            .style(LineStyle::Simple)
            .size((1, 1))
            .justify(JustifyMode::CENTER)
            .build();
        line.set_style(Protocol::default(), &mut size, &mut justify_mode, &mut commands)
            .unwrap();

        assert_eq!(size, (1, 1));
        assert_eq!(justify_mode, JustifyMode::CENTER);

        let mut commands = vec![];
        let mut size = (2, 2); // Style state value
        let mut justify_mode = JustifyMode::LEFT; // Style state value
        let line = LineBuilder::new().style(LineStyle::Simple).build();
        line.set_style(Protocol::default(), &mut size, &mut justify_mode, &mut commands)
            .unwrap();

        assert_eq!(size, (2, 2));
        assert_eq!(justify_mode, JustifyMode::LEFT);
    }
}

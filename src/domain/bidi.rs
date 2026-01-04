//! Bidirectional text support for RTL languages (Arabic, Hebrew)
//!
//! This module provides utilities for reordering bidirectional text
//! for correct visual display on ESC/POS thermal printers.
//!
//! ESC/POS printers display characters in the order they receive them,
//! so RTL (right-to-left) text must be reordered to visual order before sending.
//!
//! # Limitations
//!
//! - **No text shaping**: Arabic contextual letter forms are not handled.
//!   Users must provide pre-shaped text using Unicode presentation forms (U+FE70-U+FEFF)
//!   or use an external shaping library like `rustybuzz`.
//! - **Page code dependency**: The text must be encodable in the selected page code
//!   (e.g., PC864 for Arabic).

use unicode_bidi::BidiInfo;

/// Reorders text for visual display on thermal printers.
///
/// This function applies the Unicode Bidirectional Algorithm (UAX #9)
/// to reorder mixed RTL/LTR text into visual order suitable for
/// line-by-line printing on ESC/POS printers.
///
/// # Arguments
///
/// * `text` - The logical-order text to reorder
///
/// # Returns
///
/// The text reordered for visual display
///
/// # Example
///
/// ```rust
/// use escpos::bidi::reorder_for_display;
///
/// // Hebrew text gets reversed for display
/// let visual = reorder_for_display("אבג");
/// assert_eq!(visual, "גבא");
///
/// // LTR text stays unchanged
/// let visual = reorder_for_display("Hello");
/// assert_eq!(visual, "Hello");
/// ```
pub fn reorder_for_display(text: &str) -> String {
    let bidi_info = BidiInfo::new(text, None);
    let mut result = String::new();

    for para in &bidi_info.paragraphs {
        let line = para.range.clone();
        let reordered = bidi_info.reorder_line(para, line);
        result.push_str(&reordered);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reorder_rtl_text() {
        // Hebrew text "אבג" should be reversed for display
        let input = "אבג";
        let output = reorder_for_display(input);
        assert_eq!(output, "גבא");
    }

    #[test]
    fn test_reorder_mixed_text() {
        // Mixed RTL and LTR: "Hello אבג World"
        let input = "Hello אבג World";
        let output = reorder_for_display(input);
        // The RTL segment should be reversed
        assert!(output.contains("גבא"));
        assert!(output.contains("Hello"));
        assert!(output.contains("World"));
    }

    #[test]
    fn test_ltr_unchanged() {
        let input = "Hello World";
        let output = reorder_for_display(input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_arabic_text() {
        // Arabic "مرحبا" (Marhaba/Hello)
        let input = "مرحبا";
        let output = reorder_for_display(input);
        // Arabic is RTL, so characters should be reversed for visual display
        assert_eq!(output, "ابحرم");
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let output = reorder_for_display(input);
        assert_eq!(output, "");
    }

    #[test]
    fn test_numbers_in_rtl() {
        // Numbers in RTL context: "מחיר: 123"
        let input = "מחיר: 123";
        let output = reorder_for_display(input);
        // Numbers should stay in logical order within RTL context
        assert!(output.contains("123"));
    }
}

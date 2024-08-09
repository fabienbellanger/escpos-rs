//! Common functions

use crate::errors::{PrinterError, Result};

/// Get parameters pL and pH
pub(crate) fn get_parameters_number_2(data: &str, padding: u8) -> Result<(u8, u8)> {
    let bytes = data.as_bytes();
    let data_len = bytes.len() + (padding as usize);
    let ph = data_len / 256;
    let pl = data_len
        .checked_add_signed(-256 * isize::try_from(ph)?)
        .ok_or(PrinterError::Input(format!(
            "invalid parameter numbers (pL, pH) for data: {data}"
        )))?;

    Ok((u8::try_from(pl)?, u8::try_from(ph)?))
}

/// Get the number of characters
///
/// # Examples
/// ```
/// use escpos::utils::chars_number;
///
/// assert!(chars_number(42, 0).is_err());
/// assert_eq!(chars_number(42, 1).unwrap(), 42);
/// assert_eq!(chars_number(42, 2).unwrap(), 21);
/// assert_eq!(chars_number(42, 5).unwrap(), 8);
/// assert_eq!(chars_number(44, 3).unwrap(), 14);
/// ```
pub fn chars_number(width: u8, size: u8) -> Result<u8> {
    if !(1..=8).contains(&size) {
        return Err(PrinterError::Input(format!("invalid text_size width: {size}")));
    }

    Ok(width / size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_parameters_number_2() {
        assert_eq!(get_parameters_number_2("test123456", 3).unwrap(), (13, 0));
        assert_eq!(
            get_parameters_number_2("test123456".repeat(200).as_str(), 4).unwrap(),
            (212, 7)
        );
        assert_eq!(
            get_parameters_number_2("1".repeat(65_531).as_str(), 4).unwrap(),
            (255, 255)
        );
        assert!(get_parameters_number_2("1".repeat(65_600).as_str(), 4).is_err());
    }
}

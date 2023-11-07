//! Common functions

use crate::errors::{PrinterError, Result};

/// Get parameters pL and pH
///
/// # Example
/// ```rust
/// todo!()
/// ```
pub fn get_parameters_number_2(data: &str) -> Result<(u8, u8)> {
    let bytes = data.as_bytes();
    let data_len = bytes.len() + 3;
    let ph = data_len / 256;
    let pl = data_len
        .checked_add_signed(-256 * isize::try_from(ph)?)
        .ok_or(PrinterError::Input(format!(
            "invalid parameter numbers (pL, pH) for data: {data}"
        )))?;

    Ok((u8::try_from(pl)?, u8::try_from(ph)?))
}

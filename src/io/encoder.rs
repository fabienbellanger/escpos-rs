//! Encoder used to encode text

use crate::errors::Result;
use encoding_rs::{Encoding, UTF_8};

/// Encoder
#[derive(Clone)]
pub struct Encoder {
    codec: &'static Encoding,
}

impl Default for Encoder {
    fn default() -> Self {
        Encoder { codec: UTF_8 }
    }
}

impl Encoder {
    /// Create a new encoder
    pub fn new(codec: &'static Encoding) -> Self {
        Self { codec }
    }

    /// Encode string into the right codec
    pub(crate) fn encode(&self, data: &str) -> Result<Vec<u8>> {
        match self.codec.can_encode_everything() {
            true => Ok(self.codec.encode(data).0.into()),
            false => Err(crate::errors::PrinterError::Input(format!(
                "invalid {}",
                self.codec.name()
            ))),
        }
    }
}

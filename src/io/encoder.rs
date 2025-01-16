//! Encoder used to encode text

use crate::errors::Result;
use encoding_rs::{Encoding, UTF_8};

/// Encoder
#[derive(Clone)]
pub struct Encoder {
    codec: &'static Encoding,
    allow_unencodable: bool,
}

impl Default for Encoder {
    fn default() -> Self {
        Encoder {
            codec: UTF_8,
            allow_unencodable: false,
        }
    }
}

impl Encoder {
    /// Create a new encoder
    pub fn new(codec: &'static Encoding) -> Self {
        Self {
            codec,
            allow_unencodable: false,
        }
    }

    /// Allow this encoder to succeed if the Unicode text that it's encoding
    /// can't be fully mapped to the output encoding.
    ///
    /// Defaults to `false`.
    ///
    /// The unmappable characters will be replaced with HTML numeric character
    /// references, as per the implementation of [`encoding_rs::Encoding::encode()`].
    pub fn allow_unencodable(mut self, yes: bool) -> Self {
        self.allow_unencodable = yes;
        self
    }

    /// Encode string into the right codec
    pub(crate) fn encode(&self, data: &str) -> Result<Vec<u8>> {
        let (output, _, unmappable) = self.codec.encode(data);
        if unmappable && !self.allow_unencodable {
            return Err(crate::errors::PrinterError::Input(format!(
                "invalid {}",
                self.codec.name()
            )));
        }
        Ok(output.into())
    }
}

//! Encoder

use crate::errors::Result;
use encoding::{EncoderTrap, EncodingRef};

pub struct Encoder {
    codec: EncodingRef,
    trap: EncoderTrap,
}

impl Default for Encoder {
    fn default() -> Self {
        Encoder {
            codec: encoding::all::UTF_8,
            trap: EncoderTrap::Replace,
        }
    }
}

impl Encoder {
    pub fn new(codec: EncodingRef, trap: EncoderTrap) -> Self {
        Self { codec, trap }
    }

    pub fn encode(&self, data: &str) -> Result<Vec<u8>> {
        self.codec.encode(data, self.trap).map_err(Into::into)
    }
}

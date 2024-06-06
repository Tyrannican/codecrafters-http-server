use flate2::{write::GzEncoder, Compression};
use std::io::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Encoding {
    Gzip,
    None,
}

impl From<&str> for Encoding {
    fn from(value: &str) -> Self {
        match value {
            "gzip" => Self::Gzip,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Encoder {
    encoding: Encoding,
}

impl Encoder {
    pub(crate) fn new(encoding: impl AsRef<str>) -> Self {
        Self {
            encoding: Encoding::from(encoding.as_ref()),
        }
    }

    pub(crate) fn encode(&self, data: &[u8]) -> Vec<u8> {
        match self.encoding {
            Encoding::Gzip => self.gzip_encoding(data),
            Encoding::None => data.to_vec(),
        }
    }

    fn gzip_encoding(&self, data: &[u8]) -> Vec<u8> {
        println!("Encoding: {}", String::from_utf8(data.to_vec()).unwrap());
        let mut gz = GzEncoder::new(Vec::new(), Compression::default());
        gz.write_all(data).expect("failed to encode data");
        let compressed = gz.finish().expect("unable to compress data");

        compressed
    }
}

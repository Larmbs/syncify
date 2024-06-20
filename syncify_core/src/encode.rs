#![allow(unused)]
//! Module aimed at compression of bytes to shrink network payload

use flate2::Compression;
use flate2::bufread::ZlibEncoder;
use std::io::prelude::*;

/// Compresses a byte array
pub fn encode(bytes: &[u8]) -> Vec<u8> {
    let mut e = ZlibEncoder::new(&*bytes, Compression::default());
    let mut compressed_bytes = Vec::new();
    e.read_to_end(&mut compressed_bytes).unwrap();
    compressed_bytes
}

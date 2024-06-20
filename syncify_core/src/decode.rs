#![allow(unused)]
//! Module aimed at decompression of bytes

use flate2::bufread::ZlibDecoder;
use std::io::prelude::*;

/// Decompressed a byte array
pub fn decode(bytes: &[u8]) -> Vec<u8> {
    let mut e = ZlibDecoder::new(&*bytes);
    let mut decompressed_bytes = Vec::new();
    e.read_to_end(&mut decompressed_bytes).unwrap();
    decompressed_bytes
}

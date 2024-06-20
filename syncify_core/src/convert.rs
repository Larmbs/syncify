#![allow(unused)]
//! Module to help convert structs to bytes and back

use bincode::{
    serialize,
    deserialize,
};
use serde::{
    Deserialize,
    Serialize,
};

/// Converts any object that implements serde Serialize into a Vec<u8>
pub fn struct_to_bytes<T>(object: &T) -> Result<Vec<u8>, Box<bincode::ErrorKind>> where T: Serialize {
    serialize(object)
}

/// Converts bytes into an object that implements serde Deserialize
pub fn struct_from_bytes<T>(bytes: &[u8]) -> Result<T, Box<bincode::ErrorKind>> where T: for<'a> Deserialize<'a> {
    deserialize(bytes)
}

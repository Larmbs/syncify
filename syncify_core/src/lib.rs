#![allow(unused)]
use serde::{
    Serialize,
    Deserialize
};
use bincode::{
    self,
    serialize,
    deserialize,
};
use flate2::Compression;
use flate2::bufread::{ZlibEncoder, ZlibDecoder};
use std::io::prelude::*;

mod decode;
mod encode;

type Byte = u8;

/// Converts any object that implements serde Serialize into a Vec<u8>
fn struct_to_bytes<T>(object: &T) -> Result<Vec<Byte>, Box<bincode::ErrorKind>> where T: Serialize {
    serialize(object)
}

/// Converts bytes into an object that implements serde Deserialize
fn struct_from_bytes<T>(bytes: &[Byte]) -> Result<T, Box<bincode::ErrorKind>> where T: for<'a> Deserialize<'a> {
    deserialize(bytes)
}

/// Compresses a byte array
fn compress_bytes(bytes: &[Byte]) -> Vec<Byte> {
    let mut e = ZlibEncoder::new(&*bytes, Compression::default());
    let mut compressed_bytes = Vec::new();
    e.read_to_end(&mut compressed_bytes).unwrap();
    compressed_bytes
}

/// Decompressed a byte array
fn decompress_bytes(bytes: &[Byte]) -> Vec<Byte> {
    let mut e = ZlibDecoder::new(&*bytes);
    let mut decompressed_bytes = Vec::new();
    e.read_to_end(&mut decompressed_bytes).unwrap();
    decompressed_bytes
}

/// XORs two byte array together
fn xor(bytes1: &[Byte], bytes2: &[Byte]) -> Vec<Byte> {
    let [bytes1, bytes2] = if bytes1.len() > bytes2.len() {
        [bytes2, bytes1]
    } else {
        [bytes1, bytes2]
    };
    bytes1
        .iter()
        .chain(std::iter::repeat(&0))
        .zip(bytes2.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect()
}

/// Calculates the differences between the two objects byte wise
fn calc_diff<T>(previous: &T, new: &T) -> Result<Vec<Byte>, Box<bincode::ErrorKind>> where T: Serialize {
    Ok(xor(&struct_to_bytes(previous)?, &struct_to_bytes(new)?))
}

fn calc_new_val<T>(previous: &T, diff_bytes: &[Byte]) -> Result<Vec<Byte>, Box<bincode::ErrorKind>> where T: Serialize {
    let prev_bytes = struct_to_bytes(previous)?;
    let new = xor(&prev_bytes, &diff_bytes);
    Ok(new)
}

/// Represents different meanings to sent data
/// 
/// Data could be just small edit changes or you 
/// could specify a complete new object creation
/// 
enum OpCode {
    NewObject,
    DiffObject,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{
        Serialize,
        Deserialize
    };

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Object {
        p1: String,
        p2: u16,
        p3: Option<i128>,
    }

    #[test]
    fn test1() {
        let prev_obj = Object {
            p1: String::from("Hello World"),
            p2: 345,
            p3: None,
        };
        let new_obj = Object {
            p1: String::from("Hlo World"),
            p2: 345,
            p3: None,
        };

        let diff = calc_diff(&prev_obj, &new_obj).unwrap();

        let compressed = compress_bytes(&diff);

        let decompressed = decompress_bytes(&compressed);

        let new_bytes = calc_new_val(&prev_obj, &decompressed).unwrap();

        let new_obj1: Object = struct_from_bytes(&new_bytes).unwrap();

        assert_eq!(new_obj1, new_obj);
        println!("{:?}", new_obj1);
    }
}
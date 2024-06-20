#![allow(unused)]
//! Module that defines way of finding delta between objects

/// XORs two byte array together
pub fn xor(bytes1: &[u8], bytes2: &[u8]) -> Vec<u8> {
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

/// Calculates difference between two objects in bytes
pub fn calc_diff(prev: &[u8], new: &[u8]) -> Vec<u8> {
    xor(prev, new)
}

/// Calculates the next val given previous val and said change(diff)
pub fn calc_next_with_prev(prev: &[u8], diff: &[u8]) -> Vec<u8> {
    xor(prev, diff)
}

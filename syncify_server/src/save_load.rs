//! Module to load a save serde serializable objects

use serde::{
    Serialize,
    Deserialize,
};
use serde_json;

use syncify_core::{struct_to_bytes, struct_from_bytes};

use std::io;
use std::fs;
use std::path::Path;

/// Saves an object to some path
pub fn save<T, P>(object: &T, path: P) -> io::Result<()> where T: Serialize, P: AsRef<Path> {
    let bytes = struct_to_bytes(object).unwrap();
    fs::write(path, bytes)
}

/// Loads an object from a path
pub fn load<T, P>(path: P) -> io::Result<T> where T: for<'a> Deserialize<'a>, P: AsRef<Path> {
    Ok(struct_from_bytes(&fs::read(path).unwrap()).unwrap())
}

/// Saves an object as json to some path
pub fn save_to_json<T, P>(object: &T, path: P) {
    todo!()
}

/// Loads an object from json from some path
pub fn load_from_json<T, P>(path: P) {
    todo!()
}

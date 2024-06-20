#![allow(unused)]
use std::net::{
    SocketAddr,
    TcpListener
};

use std::io;

use std::sync::Mutex;

mod save_load;

use syncify_core::{struct_from_bytes, struct_to_bytes};

use lazy_static::lazy_static;
use serde::{
    Serialize,
    Deserialize,
};

// Creating a shared server listening object
lazy_static! {
    static ref LISTENER: Mutex<Option<TcpListener>> = Mutex::new(None);
    // Store the object as bytes, opens up room for error and attack as someone can send faulty object
    static ref SYNC_OBJECT: Mutex<Option<Vec<u8>>> = Mutex::new(None);
}

/// Binds the server to the addr you specify
pub fn bind_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> { 
    let listener = TcpListener::bind(addr)?;

    let mut guard = LISTENER.lock()?;
    *guard = Some(listener);

    Ok(())
}

/// Sets the object to be synced on the server
pub fn set_sync_object<T>(object: T) -> Result<(), Box<dyn std::error::Error>> where T: Serialize {
    let bytes = struct_to_bytes(&object).unwrap();

    let mut guard = SYNC_OBJECT.lock()?;
    *guard = Some(bytes);

    Ok(())
}

/// Sets the object to be synced on the server
pub fn get_sync_object<T>() -> Result<T, Box<dyn std::error::Error>> where T: Serialize + for<'a> Deserialize<'a> {
    let mut guard = SYNC_OBJECT.lock()?;
    Ok(struct_from_bytes(&guard.as_ref().unwrap()).unwrap())
}

/// Begin listening for incoming requests and handling them
pub fn listen() -> Result<(), Box<dyn std::error::Error>> {
    let mut guard = LISTENER.lock()?;

    match &mut *guard {
        Some(listener) => {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                
            }
        },
        None => panic_bind_server(),
    }

    Ok(())
}

/// Called when server has no socket addr to bind to
fn panic_bind_server() {
    panic!("You must bind the server to a SocketAddr, try bind_server()")
}

/// Called when user has not specified what object to sync over network
fn panic_no_sync_object_defined() {
    panic!("You must define an object to be synced, try set_sync_object()")
}

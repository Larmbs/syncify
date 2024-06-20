use std::io::Read;
use std::net::{
    SocketAddr,
    TcpListener
};

use std::sync::Mutex;

mod save_load;
pub use save_load::*;

use syncify_core::{struct_from_bytes, struct_to_bytes, Syncable};

use lazy_static::lazy_static;

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

/// Sets the object to be synced by the server
pub fn set_sync_object<T>(object: T) -> Result<(), Box<dyn std::error::Error>> where T: Syncable {
    let bytes = struct_to_bytes(&object).unwrap();

    let mut guard = SYNC_OBJECT.lock()?;
    *guard = Some(bytes);

    Ok(())
}

/// Gets the object currently being synced by the server
pub fn get_sync_object<T>() -> Result<T, Box<dyn std::error::Error>> where T: Syncable {
    let guard = SYNC_OBJECT.lock()?;

    match &guard.as_ref() {
        Some(obj_bytes) => {
            let obj = struct_from_bytes(obj_bytes)?;
            Ok(obj)
        },
        None => panic_no_sync_object_defined(),
    }
}

/// Begin listening for incoming requests and handling them
pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let mut guard = LISTENER.lock()?;

    let mut obj_bytes_guard = SYNC_OBJECT.lock()?;
    let obj_bytes = obj_bytes_guard.as_mut().unwrap();

    match &mut *guard {
        Some(listener) => {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut buf = Vec::new();
                stream.read(&mut buf).unwrap();

                *obj_bytes = buf;
            }
        },
        None => panic_bind_server(),
    }

    Ok(())
}

/// Stops the server from running
pub fn stop_server() {
    todo!()
}

#[inline]
/// Called when server has no socket addr to bind to
fn panic_bind_server() -> ! {
    panic!("You must bind the server to a SocketAddr, try bind_server()")
}

#[inline]
/// Called when user has not specified what object to sync over network
fn panic_no_sync_object_defined() -> ! {
    panic!("You must define an object to be synced, try set_sync_object()")
}

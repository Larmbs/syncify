#![allow(unused)]
use std::net::{
    SocketAddr,
    TcpListener
};

use std::io;

use std::sync::Mutex;


use lazy_static::lazy_static;

// Creating a shared server listening object
lazy_static! {
    static ref LISTENER: Mutex<Option<TcpListener>> = Mutex::new(None);
}

/// Binds the server to the addr you specify
pub fn bind_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> { 
    let listener = TcpListener::bind(addr)?;

    let mut guard = LISTENER.lock()?;
    *guard = Some(listener);

    Ok(())
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

fn panic_bind_server() {
    panic!("You must bind the server to a SocketAddr, try bind_server()")
}
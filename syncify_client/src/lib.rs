#![allow(unused)]
use lazy_static::lazy_static;
use std::net::{
    TcpStream,
    SocketAddr,
};
use std::sync::{
    Mutex, 
};
use std::io::{
    self, 
    Write,
    Read,
};

use serde::{Deserialize, Serialize};
use bincode::{
    deserialize, serialize, ErrorKind
};

lazy_static! {
    /// Creating a shared stream object to talk with server
    static ref CONN: Mutex<Option<TcpStream>> = Mutex::new(None);
}

/// Connects to external server for syncing
pub fn connect(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let mut guard = CONN.lock()?;

    let stream = TcpStream::connect(addr)?;
    *guard = Some(stream);

    Ok(())
}

/// Trait that allows struct to sync with other structs through server
pub trait Sync<'a>: Serialize + Deserialize<'a> {
    fn listen() {
        todo!()
    }
    /// Function that sends an entire functions state
    fn send(&mut self) {
        let bytes = serialize(self).unwrap();
        
        let mut guard = CONN.lock().unwrap();

        match &mut *guard {
            Some(stream) => {
                stream.write_all(&bytes).unwrap();
            },
            None => panic_no_connection(),
        }
    }
    fn init() {
        todo!()
    }
}

#[inline]
/// Functions that panics to warn of not properly initializing a connection with server prior to syncing
fn panic_no_connection() {
    panic!("You attempted to sync your objects without connecting to a server!");
}

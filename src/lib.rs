use lazy_static::lazy_static;
use std::net::{
    TcpStream,
    SocketAddr,
};
use std::sync::Mutex;
use std::io;

use serde::{Deserialize, Serialize};

lazy_static! {
    static ref CONN: Mutex<Option<TcpStream>> = Mutex::new(None);
}

/// Connects to external server for syncing
pub fn connect(addr: SocketAddr) -> io::Result<()> {
    let mut conn = CONN.lock().unwrap();

    let stream = TcpStream::connect(addr)?;
    *conn = Some(stream);

    Ok(())
}

/// Trait that allows struct to sync with other structs through server
pub trait Sync<'a>: Serialize + Deserialize<'a> {
    fn listen() {
        todo!()
    }
    fn send() {
        todo!()
    }
    fn init() {
        todo!()
    }
}

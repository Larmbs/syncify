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
pub fn bind_server(addr: SocketAddr) -> io::Result<()> { 
    let listener = TcpListener::bind(addr)?;

    // Updating listener
    let mut guard = LISTENER.lock().unwrap();
    *guard = Some(listener);

    Ok(())
}

/// Begin listening for incoming requests and handling them
pub fn listen() {
    let mut guard = LISTENER.lock().unwrap();

    match &mut *guard {
        Some(listener) => {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                
            }
        },
        None => panic!("You have not binded the server, please refer to the bind_server() method"),
    }
}

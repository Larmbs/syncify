# Syncify 
Allows for easy and simple syncing of objects over network.

## Description
Just a simple crate to help you sync objects over the internet. This can be useful on simple programs like file-sharing, multiplayer games, and communication.

## Features
- [ ] UDP Syncing
- [ ] TCP Syncing
- [x] Saving and loading of objects in files
- [x] Syncable trait

## Example
### Client 
This is the client code, its made so simple through this library. The server holds an object that is being synced so you can request a copy of it from the users side. If any changes get made to the value of it the server will notify that object to update its state. 
```Rust client.rs
use syncify_client::*;

// Best to put dynamic sized objects at the end
#[derive(Syncable)]
struct Doc {
    length: usize,
    name: String,
    editors: Vec<String>,
    text: String,
}
impl Doc {
    fn change_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}

fn main() {
    connect("127.0.0.1:7777".parse().unwrap());
    let mut synced_obj: Doc = get_instance().unwrap();
    // Will send request to server and change name on client side
    synced_obj.change_name(String::from("Different name"));
    // Connection automatically released with no use
}
```
### Server
This is the server end of the program this crate makes it really easy to start a server. Simply provide a beginning version of the object and then just start the server to begin syncing between connections. you could then just shut down the server and get back the final version of the object to save.
```Rust server.rs
use syncify_server::*;
use std::{thread, time};

struct Doc {
    length: usize,
    name: String,
    editors: Vec<String>,
    text: String,
}

fn main() {
    bind_server("127.0.0.1:7777".parse().unwrap());

    let object = load("saves/data.txt");
    set_sync_object(object);

    start_server();

    thread::sleep(time::Duration::from_secs(60));

    stop_server();

    let object = get_sync_object();
    save(object, "saves/data.txt");
}
```

## WARNINGS
This is still in development and there may be some unexpected behavior from race conditions that may mess up values within the server. It is best not to overload the server with requests. Users with bad internet connections may hurt the integrity of this system.

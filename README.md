# Syncify 
Allows for easy and simple syncing of objects over network.

## Example
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
}
```

```Rust server.rs
use syncify_server::*;

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

    sleep(10s)

    stop_server();

    let object = get_sync_object();
    save(object, "saves/data.txt");
}
```
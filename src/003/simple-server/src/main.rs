// fn main() {
//     println!("Hello, world!");
// }

use std::net::{TcpListener, TcpStream};

fn handle_client(_stream: TcpStream) {
    // ...
    println!("Received some contents...");
}

fn main() -> std::io::Result<()> {
    // let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    // It is more convenient and concise to use `?`
    let listener = TcpListener::bind("127.0.0.1:8080")?; // `?` means that the returned type is Result

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

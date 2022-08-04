#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::net::TcpListener;
#[allow(unused_imports)]
use std::io::{Read, Write};


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("localhost:6379").unwrap();
    match listener.accept() {
        Ok((mut socket, addr)) => {
            socket.write(b"PONG").unwrap();
        },
        Err(e) => println!("Error: {}", e),
    }
}

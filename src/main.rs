#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::net::TcpListener;
#[allow(unused_imports)]
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;


fn pong(mut stream: &TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    stream.write(b"+PONG\r\n").unwrap();
}


fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("localhost:6379").unwrap();
    let mut conn = listener.accept();
    loop {
        match conn {
            Ok((ref mut socket, _addr)) => {
                pong(&socket);
            },
            Err(ref e) => println!("Error: {}", e),
            }
        }
    }

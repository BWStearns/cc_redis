#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::net::TcpListener;
#[allow(unused_imports)]
use std::io::{Read, Write};
use std::net::TcpStream;
// use std::thread;
// use std::time::Duration;


fn pong(mut stream: &TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    stream.write(b"+PONG\r\n").unwrap();
}


fn main() {

    let aws_metadata = "251.254.169.254";
    let aws_metadata_port = 80;
    
    println!("Connecting to {}:{}", aws_metadata, aws_metadata_port);
    // Check for aws metadata
    let resp = reqwest::blocking::get(&format!("http://{}:{}/latest/meta-data/public-ipv4", aws_metadata, aws_metadata_port)).unwrap();
    // let resp = reqwest::blocking::get(&format!("http://google.com")).unwrap();
    println!("{}", resp.text().unwrap());

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

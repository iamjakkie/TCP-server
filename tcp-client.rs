use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8090").expect("Failed to connect to server");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        stream.write(input.as_bytes()).expect("Failed to write to server");

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).expect("Failed to read from server");
        let response = String::from_utf8_lossy(&buffer[..]);
        println!("Response: {}", response);
    }
}
// Importing modules from Rust libraries
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // buffer to read data from client
    let mut buffer = [0; 1024];
    // read data from stream to buffer
    stream.read(&mut buffer).expect("Failed to read from client");
    // converts data from buffer to string
    let request = String::from_utf8_lossy(&buffer[..]); 
    println!("Request: {}", request);
    let response = "Hello from server".as_bytes();
    stream.write(response).expect("Failed to write to client");
}

fn main() {
    // bind the server to the address
    let listener = TcpListener::bind("127.0.0.1:8090").expect("Failed to bind to address");
    println!("Server listening on port 8090");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn( || handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        }
    }
}
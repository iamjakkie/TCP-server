
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use std::net::{TcpListener, TcpStream};
use std::io::{stdin, BufRead, BufReader, Error, Write};
use std::{env, str, thread};

#[derive(Serialize, Deserialize, Debug)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

fn handle_client(stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut data = Vec::new();
    let mut stream = BufReader::new(stream);
    loop {
        data.clear();
        let bytes_read = stream.read_until(b'\n', &mut data)?;
        if bytes_read == 0 {
            return Ok(());
        }
    }
    let input: Point3D = serde_json::from_slice(&data)?;
    let value = input.x.pow(2) + input.y.pow(2) + input.z.pow(2);
}
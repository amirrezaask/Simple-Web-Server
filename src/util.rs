use std::fs;
use std::io::prelude::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn write_on_stream(stream: &mut TcpStream, body: &[u8]) -> Result<usize, std::io::Error> {
    stream.write(body)
}

pub fn read_from_stream(stream: &mut TcpStream) -> [u8; 512] {
    let mut buffer = [0; 512];

    match stream.read(&mut buffer) {
        Err(e) => println!("can't read stream : {}", e),
        _ => (),
    }
    buffer
}

use std::fs;
use std::io::prelude::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::util;
pub fn index(mut stream: TcpStream) {
    let request = String::from_utf8_lossy(&util::read_from_stream(&mut stream));

    let template =
        fs::read_to_string("index.html").unwrap_or(String::from("Could not read template"));

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", template);

    match util::write_on_stream(&mut stream, response.as_bytes()) {
        Ok(n) => println!("#{} bytes written", n),
        Err(e) => println!("could not write request : {}", e),
    }

    match stream.flush() {
        Ok(_) => return,
        Err(e) => println!("Error on flushing stream : {}", e),
    }
}

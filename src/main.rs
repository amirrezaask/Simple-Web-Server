extern crate clap;
use std::collections::HashMap;
use std::io::prelude::{ Read, Write };
use std::net::{ TcpListener, TcpStream };
use clap::{App, ArgMatches, Arg};

fn handle_stream (mut stream: TcpStream) {
    let mut buffer = [0; 512];
    if stream.read(&mut buffer).is_err() {
        println!("can't read stream" );
        return
    }
    println!("Request : {} ", String::from_utf8_lossy(&buffer[..]));
}



fn get_port<'a>() -> u32 {
    let matches = App::new("web-server-rs")
        .author("AmirrezaAsk")
        .about("Simple HTTP web server in rust")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("port to listen on")
                .takes_value(false),
        ).get_matches();
    let mut port: u32 = 0;
    if matches.occurrences_of("port") != 0 {
        port = matches.value_of("port").unwrap().parse().unwrap();
    } else {
        port = 8080;
    }
    port
    
}


fn main () {
    let port_number = get_port();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port_number)).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream);
    }
}
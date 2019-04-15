extern crate clap;

use std::net::TcpListener;

use clap::{App, ArgMatches, Arg};

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
    
}
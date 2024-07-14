extern crate db_core;

use db_core::Database;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream, server: &mut Database) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let query = String::from_utf8_lossy(&buffer[..]);
    let response = server.execute(query.to_string()).unwrap();
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut server = Database::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream, &mut server);
    }
}

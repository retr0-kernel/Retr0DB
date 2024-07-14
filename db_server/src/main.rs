use std::io::Read; // Remove `Write` if it's not used
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let query: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);
    // Rest of your logic...
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
}

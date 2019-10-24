extern crate native_tls;

use native_tls::TlsConnector;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let tls_connector = TlsConnector::new().unwrap();

    let stream = TcpStream::connect("localhost:5568").unwrap();
    let mut stream = tls_connector.connect("localhost", stream).unwrap();

    let stdin = io::stdin();
    let mut input_buffer = String::new();

    // Input loop.
    loop {
        println!("Awaiting input:");
        input_buffer.clear();
        stdin.read_line(&mut input_buffer).unwrap();

        stream.write(input_buffer.trim().as_bytes()).unwrap();
        stream.flush().unwrap();

        // Read answer. TODO: read answer as flatbuffer.
        let mut buffer = [0; 20];
        stream.read(&mut buffer).unwrap();
        let answer = String::from_utf8_lossy(&buffer[..]);
        let answer = answer.trim_end_matches(char::from(0));
        println!("Answer is {:?}", answer);
    }
}

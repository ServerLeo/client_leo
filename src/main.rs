extern crate native_tls;

use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let connector = TlsConnector::new().unwrap();

    let stream = TcpStream::connect("localhost:5568").unwrap();
    let mut stream = connector.connect("localhost", stream).unwrap();

    // Write req1
    stream.write("req1".as_bytes()).unwrap();
    stream.flush().unwrap();
    // Receive answer
    let mut buffer = [0; 20];
    stream.read(&mut buffer).unwrap();
    println!("{:?}", String::from_utf8_lossy(&buffer));
    // write close
    stream.write("close".as_bytes()).unwrap();
    stream.flush().unwrap();
}

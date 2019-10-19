extern crate native_tls;

use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let connector = TlsConnector::new().unwrap();

    let stream = TcpStream::connect("localhost:5568").unwrap();
    let mut stream = connector.connect("localhost", stream).unwrap();

    stream.write_all(b"123456789012345678901234567890").unwrap();
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));
}

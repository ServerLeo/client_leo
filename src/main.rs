use async_std::{net::TcpStream, prelude::*, task};
use async_tls::TlsConnector;
use std::{io, io::Cursor, path::Path, sync::Arc};

use rustls::ClientConfig;

fn main() {
    task::block_on(async { open_connection().await })
}

async fn open_connection() {
    let tcp_stream = TcpStream::connect("localhost:5568")
        .await
        .expect("TCP handshake failed.");

    let mut config = ClientConfig::new();

    let cafile = Path::new("end.chain");
    let file = async_std::fs::read(cafile)
        .await
        .expect("Failed to read file.");
    let mut pem = Cursor::new(file);
    config
        .root_store
        .add_pem_file(&mut pem)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
        .expect("Unable to create configuration object.");

    let tls_connector = TlsConnector::from(Arc::new(config));
    let mut tls_stream = tls_connector
        .connect("localhost", tcp_stream)
        .expect("TLS handshake failed.")
        .await
        .expect("Awaiting TLS failed");

    // IO initialization.
    let stdin = io::stdin();
    let mut input_buffer = String::new();

    // Input loop.
    loop {
        println!("Awaiting input:");
        input_buffer.clear();
        stdin
            .read_line(&mut input_buffer)
            .expect("Reading input failed.");

        tls_stream
            .write(input_buffer.trim().as_bytes())
            .await
            .expect("Writing into TLS stream failed.");
        tls_stream.flush().await.expect("Flushing stream failed.");

        // Read answer. TODO: read answer as flatbuffer.
        let mut buffer = [0; 20];
        tls_stream
            .read(&mut buffer)
            .await
            .expect("Reading from TLS stream failed.");
        let answer = String::from_utf8_lossy(&buffer[..]);
        let answer = answer.trim_end_matches(char::from(0));
        println!("Answer is {:?}", answer);
    }
}

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

fn handle_client(mut stream: TcpStream) -> Option<String> {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let received_data = str::from_utf8(&buffer[0..size]).unwrap_or("");
            println!("Received data: {}", received_data);
            stream
                .write_all(&buffer[0..size])
                .expect("Failed to write to stream");

            Some(received_data.to_string())
        }
        Err(e) => {
            println!(
                "An error occurred, terminating connection with {}: {}",
                stream.peer_addr().unwrap(),
                e
            );
            None
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9090").unwrap();
    println!("Server listening on port 9090");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let _ = handle_client(stream);
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

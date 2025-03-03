use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread;

pub fn port(port_num: String) {
    let addr = format!("127.0.0.1:{}", port_num);
    let listener = TcpListener::bind(addr.clone()).unwrap();
    println!("Listening on {}", addr);

    // Loop indefinitely over incoming connections.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Handle each connection in a new thread.
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, world!";
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

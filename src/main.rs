mod http;

// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::TcpListener,
};

use crate::http::status::HttpStatus;

const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "4221";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Logs from your program will appear here!");

    let ip_address = format!("{ADDRESS}:{PORT}");
    let listener = TcpListener::bind(ip_address).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = Vec::<u8>::new();

                let request = stream.read(&mut buf)?;
                let response_status = HttpStatus::Ok;

                stream.write(response_status.get_response().as_bytes())?;
                stream.flush()?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}

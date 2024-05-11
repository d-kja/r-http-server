// Uncomment this block to pass the first stage
use std::net::TcpListener;

const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "4221";

fn main() {
    println!("Logs from your program will appear here!");

    let ip_address = format!("{ADDRESS}:{PORT}");
    let listener = TcpListener::bind(ip_address).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

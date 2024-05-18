mod http;

use std::net::TcpListener;

use crate::http::handler::handle_incoming_request;

const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "4221";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip_address = format!("{ADDRESS}:{PORT}");
    let listener = TcpListener::bind(ip_address).unwrap();

    println!("HTTP Server started");

    handle_incoming_request(&listener)?;

    Ok(())
}

use std::{
    io::{Read, Write},
    net::TcpListener,
    ops::DerefMut,
    sync::{Arc, Mutex},
    thread::{self},
};

use super::routes::router;

pub fn handle_incoming_request(
    listener: &TcpListener,
) -> Result<(), Box<dyn std::error::Error>> {
    for stream in listener.incoming() {
        let stream = Arc::new(Mutex::new(stream?));
        let stream_clone = stream.clone();

        // can cause thread overflow, it's better to use a thread pool
        // and... it needs to do the actual thread clean up 
        // so please don't repeat this at home kids...
        thread::spawn(move || {
            let mut stream = stream_clone.lock().unwrap();
            let stream = stream.deref_mut();

            let mut buf = [0; 1024];
            stream.take(1024).read(&mut buf).unwrap();

            let buf = buf
                .iter()
                .filter(|&&b| !b.eq(&0))
                .map(|a| *a)
                .collect::<Vec<u8>>();

            let response = match router(&buf) {
                Ok(value) => value.get_response(),
                Err(err) => err.get_response(),
            };

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        });
    }

    Ok(())
}

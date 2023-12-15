use serde::Deserialize;
use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process;

#[derive(Deserialize)]
pub struct Config {
    pub algorithm: String,
    pub backend_pool: String,
}

pub fn proxy(mut stream: TcpStream, backend_addr: &str) -> Result<(), Box<dyn Error>> {
    let mut backend_stream = TcpStream::connect(backend_addr).unwrap_or_else(|err| {
        eprintln!("Cannot connect to the backend: {err}");
        process::exit(1);
    });
    let mut buffer = [0; 1024];

    println!("Forwarding request to: {:#?}", backend_addr);

    let mut bytes = stream.read(&mut buffer)?;
    backend_stream.write_all(&buffer[..bytes])?;
    backend_stream.flush()?;

    buffer = [0; 1024];
    bytes = backend_stream.read(&mut buffer)?;
    stream.write_all(&buffer[..bytes])?;
    stream.flush()?;

    Ok(())
}

pub fn round_robin(config: Config) {
    let mut backend_list = config.backend_pool.split(',').cycle();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else(|err| {
        eprintln!("Cannot bind listener to the address: {err}");
        process::exit(1);
    });

    println!("Chosen algorithm is {:?}", config.algorithm);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = proxy(stream, backend_list.next().unwrap()) {
                    // unwrap() is fine, since the iterator is cyclic, it will never return None
                    eprintln!("Proxy encountered an error: {e}");
                }
            }
            Err(e) => {
                eprintln!("Connection failed: {e}");
            }
        }
    }
}


pub fn weighted_round_robin(config: Config) {
    todo!();
}

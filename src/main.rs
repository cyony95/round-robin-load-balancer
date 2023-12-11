use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use serde::Deserialize;
use toml;

fn main() {
    let config: Config = toml::from_str(&fs::read_to_string("config.toml").unwrap()).unwrap();
    let mut backend_list = config.backend_pool.split(',').cycle();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Chosen algorithm is {:?}", config.algorithm);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        round_robin(stream, backend_list.next().unwrap()); // cycles through the given list of backends  backend_list.iter().cycle().next().unwrap()
    }
}

fn round_robin(mut stream: TcpStream, backend_addr: &str) {
    let mut backend_stream = TcpStream::connect(backend_addr).unwrap();
    println!("Forwarding request to: {:#?}", backend_addr);
    let mut buffer = [0; 1024];
    let bytes = stream.read(&mut buffer).unwrap();
    backend_stream.write_all(&buffer[..bytes]).unwrap();
    backend_stream.flush().unwrap();
    let mut buffer = [0; 1024];
    let bytes_read = backend_stream.read(&mut buffer).unwrap();
    stream.write_all(&buffer[..bytes_read]).unwrap();
    stream.flush().unwrap();
}

#[derive(Deserialize)]
struct Config {
    algorithm: String,
    backend_pool: String,
}

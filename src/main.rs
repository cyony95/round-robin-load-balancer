use std::{fs, process};
use toml;
use simple_lb::*;


fn main() {
    let config: Config = toml::from_str(&fs::read_to_string("config.toml").unwrap()).unwrap();
    match config.algorithm.as_str() {
        "round-robin" => {
            round_robin(config);
        }
        "weighted-round-robin" => {
            todo!();
        }
        _ => {
            eprintln!("Load balancing algorithm not known! Please choose round-robin or weighted-round-robin! Exiting...");
            process::exit(1);
        }
    }
}




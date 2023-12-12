use std::{fs, process};
use toml;
use simple_lb::*;


fn main() {
    let config: Config = toml::from_str(&fs::read_to_string("config.toml").unwrap_or_else(|err| {
        eprintln!("Problem reading from config.toml: {err}");
        process::exit(1);
    }))
    .unwrap_or_else(|err| {
        eprintln!("Make sure config.toml file is correct. {err}");
        process::exit(1);
    });
    match config.algorithm.as_str() {
        "round-robin" => {
            round_robin(config);
        }
        "weighted-round-robin" => {
            weighted_round_robin(config);
        }
        _ => {
            eprintln!("Load balancing algorithm not known! Please choose round-robin or weighted-round-robin! Exiting...");
            process::exit(1);
        }
    }
}




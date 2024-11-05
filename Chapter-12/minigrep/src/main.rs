use std::{env, process};

extern crate minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(args).unwrap_or_else(|error| {
        eprintln!("An error occurred while parsing the arguments: {error}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("An error occurred: {}", e);
        process::exit(1);
    }
}

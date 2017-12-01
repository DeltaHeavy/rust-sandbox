use std::env;
use std::process;

extern crate tutorial;
use tutorial::Config;

// paused on Tutorial Book 2nd Ed. 12.3

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("error parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = tutorial::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}


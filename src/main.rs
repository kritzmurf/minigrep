use std::{env, process};

use minigrep::{Config, run};

fn main() {
    //get CLI args
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing CLI args: {error}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}


mod lib;
use lib::Config;
use std::{env, process};

fn main() {
    let input_config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to use the program: {err}");
        process::exit(1);
    });
    if let Err(err) = lib::run(input_config) {
        eprintln!("Following error occurred: {err}");
        process::exit(1);
    };
}

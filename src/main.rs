mod lib;
use lib::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_config = Config::new(&args).unwrap_or_else(|err| {
        println!("Failed to use the program: {err}");
        process::exit(1);
    });
    if let Err(err) = lib::run(input_config) {
        println!("Following error occurred: {err}");
        process::exit(1);
    };
}

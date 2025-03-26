use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_text)?;

    println!("Text is {file_content}");
    Ok(())
}

pub struct Config {
    query: String,
    file_text: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        Ok(Config {
            query: args[1].clone(),
            file_text: args[2].clone(),
        })
    }
}

use std::error::Error;
use std::fs;

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
            return Err("not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            file_text: args[2].clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
        ";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut result = Vec::new();
        for line in contents.lines() {
            if line.contains(query) {
                result.push(line.trim());
            }
        };

        result
    }
}

fn main() {}

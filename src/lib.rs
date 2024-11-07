use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments")
        }

        let query = args[1].clone();

        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("\n{line}")
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_result() {
        let query = "duct";

        let contents = "\
Rust:
Safe, Fast, Productive.
Pick three.";

        assert_eq!(vec!["Safe, Fast, Productive."], search(query, contents));
            
    }

    #[test]
    fn test_no_result() {
        let query = "monormophization";

        let contents = "\
Rust:
Safe, Fast, Productive.
Pick three.";

    assert_eq!(vec!["Safe, Fast, Productive."], search(query, contents));
    }

    #[test]
    fn test_case_sensitivie() {
        let query = "duct";

        let contents = "\
Rust:
Safe, Fast, Productive.
Pick three.
Duct tape";
        assert_eq!(vec!["Safe, Fast, Productive."], search(query, contents));
    }

    #[test]
    fn test_case_insesitive() {
        let query = "rUsT";

        let contents = "\
Rust:
Safe, Fast, Productive.
Pick three.
Trust me";
        assert_eq!(vec!["Rust:", "Trust me"], search(query, contents));
    }
}
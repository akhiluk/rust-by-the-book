use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.search_term, &contents)
    } else {
        search(&config.search_term, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    
    return Ok(());
}

pub struct Config {
    pub search_term: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let search_term = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("MINIGREP_IGNORE_CASE").is_ok();

        return Ok(Config { search_term, file_path, ignore_case });
    }
}

pub fn search<'a>(search_term: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(search_term) {
            results.push(line);
        }
    }
    return results;
}

pub fn search_case_insensitive<'a>(search_term: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let search_term = search_term.to_lowercase();

    for line in contents.lines() {
        // Adding &search_term here because to_lowercase()
        // returns a String instead of &str.
        if line.to_lowercase().contains(&search_term) {
            results.push(line);
        }
    }
    
    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let search_term = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(search_term, contents));
    }

    #[test]
    fn case_insensitive() {
        let search_term = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(search_term, contents));
    }
}
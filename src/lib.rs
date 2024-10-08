use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments.");
        } else if args.len() > 3 {
            panic!("Extra arguments provided");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Config {
            query,
            file_path,
            ignore_case,
        }
    }

    pub fn build(mut args: impl Iterator<Item= String>) -> Result<Config, &'static str> {
        args.next();

        let query =  match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments. ")
        };

        let file_path =  match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments. ")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> { // dyn is short for dynamic
    let contents = fs::read_to_string(config.file_path)?;

    // println!("Contents: \n {:?}", contents);
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    for line in results{
        println!("{:?}", line)
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query:&str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|content| content.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query:&str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive
Pick three:.";
        assert_eq!(vec![""], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive
Pick three:.";
        assert_eq!(vec!["safe, fast, productive"], search_case_insensitive(query, contents));
    }
}
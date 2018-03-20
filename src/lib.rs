extern crate regex;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

static DEFAULT_OPTS: &'static str = "(?m)";

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a filename"),
        };

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = search(&config.query, &contents)?;

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Result<Vec<&'a str>, Box<Error>> {
    let mut pattern = String::from(::DEFAULT_OPTS);
    pattern.push_str(query);
    let regex = Regex::new(&pattern)?;

    Ok(contents
        .lines()
        .filter(|line| regex.is_match(line))
        .collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents).unwrap()
        );
    }

    #[test]
    fn regex_pattern() {
        let query = "^Rust"; // Match 'Rust' at the start of a line
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
You can do it all with Rust!";

        assert_eq!(vec!["Rust:"], search(query, contents).unwrap());
    }
}

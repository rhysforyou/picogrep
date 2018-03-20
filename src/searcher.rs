extern crate regex;

use std::error::Error;
use regex::Regex;

static DEFAULT_OPTS: &'static str = "(?m)";

pub fn search<'a>(query: &str, contents: &'a str) -> Result<Vec<&'a str>, Box<Error>> {
    let mut pattern = String::from(DEFAULT_OPTS);
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

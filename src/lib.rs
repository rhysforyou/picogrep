extern crate regex;

mod searcher;
pub mod config;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use config::Config;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = searcher::search(&config.query, &contents)?;

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

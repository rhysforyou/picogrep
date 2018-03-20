/*!
This crate provides a program called `minigrep`, a greatly simplified version of the standard UNIX
`grep` program. It supports searching for a plaintext phrase or regular expresion.

# Usage

```sh
$ minigrep [string-or-pattern] [path]
```

# Example: Find a Lines Beginning With a Date

```sh
$ minigrep '^{4}-\d{2}-\d{2}' error.log
2018-03-19: An old log entry
2018-03-20: A log entry
```
*/

extern crate regex;

mod searcher;
mod config;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
pub use config::Config;

/// Run `minigrep` with tthe provided arguments
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

use std::env;

/// A struct which store's picogrep's configuration
#[derive(Debug)]
pub struct Config {
    /// The plaintext query or regex pattern to search for
    pub query: String,
    /// The file to search in
    pub filename: String,
}

impl Config {
    /// Create a new `Config` from the provided command line arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use picogrep::Config;
    ///
    /// match Config::new(std::env::args()) {
    ///     Ok(config) => println!("{:?}", config),
    ///     Err(err) => eprintln!("An error occurred: {}", err),
    /// };
    /// ```
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

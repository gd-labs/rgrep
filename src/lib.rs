use std::{
    env,
    fs,
    error::Error
};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // The `read_to_string` function takes a file and converts its
    // content into a string, returning a result. To handle the Err
    // variant, `?` is used, returning an error of any kind.
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    // Variant with a unit representing that the process executed
    // as expected.
    Ok(())
}

/// Unit representing the operation of searching a determined
/// textual content in a file.
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    /// Extracts the query and filename from the arguments passed.
    pub fn new(
        mut args: impl Iterator<Item = String>, 
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query string not provided")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("File path not provided"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config {
            query,
            filename,
            case_sensitive
        })
    }
}

/// Whenever a reference is returned from a function, its lifetime
/// needs to be tied to one of the input parameters' lifetime. Given
/// that the return value is a reference to the file content's lines
/// that satisfy the query, the lifetime is based on it.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    // Shadows the `query` passed as a parameter.
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_insensitive(query, contents)
        );
    }
}
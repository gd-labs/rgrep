use std::{
    env,
    fs,
    process,
    error::Error
};

fn main() {
    // The `args` function provides an iterator over the arguments
    // passed to the program and `collect` turns that iterator
    // into a collection.
    let args: Vec<String> = env::args().collect();
    // Builds a new config object handling the Err value from the
    // `Result` type through a closure.
    let config = Config::new(&args).unwrap_or_else( |err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // The if let syntax functions as a matching statement that
    // handles the Err variant.
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // The `read_to_string` function takes a file and converts its
    // content into a string, returning a result. To handle the Err
    // variant, `?` is used, returning an error of any kind.
    let contents = fs::read_to_string(config.filename)?;

    println!("{}", contents);
    // Variant with a unit representing that the process executed
    // as expected.
    Ok(())
}

/// Unit representing the operation of searching a determined
/// textual content in a file.
struct Config {
    query: String,
    filename: String,
}

impl Config {
    /// Extracts the query and filename from the arguments passed.
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        // Here, `clone` is used so that the variables don't take
        // ownership over the string.
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config {query, filename})
    }

}

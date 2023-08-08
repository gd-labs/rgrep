use std::{
    env,
    process,
};
use rgrep::Config;

fn main() {
    // The `args` function provides an iterator over the arguments
    // passed to the program and `collect` turns that iterator
    // into a collection.
    let args: Vec<String> = env::args().collect();
    // Builds a new config object handling the Err value from the
    // `Result` type through a closure.
    let config = Config::new(&args).unwrap_or_else( |err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    // The if let syntax functions as a matching statement that
    // handles the Err variant.
    if let Err(e) = rgrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
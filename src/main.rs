use std::{
    env,
    process,
};
use rgrep::Config;

fn main() {
    // Builds a new config object handling the Err value from the
    // `Result` type through a closure.
    let config = Config::new(env::args()).unwrap_or_else( |err| {
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
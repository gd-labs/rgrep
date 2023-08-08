use std::env;
use std::fs;

fn main() {
    // The `args` function provides an iterator over the arguments
    // passed to the program and `collect` turns that iterator
    // into a collection.
    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let filename = &args[2];
    // The `read_to_string` function takes a file and converts its
    // content into a string, returning a result. To handle the Err
    // variant, `expect` is used, throwing an error message.
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong while reading the file.");

    println!("{}", contents);
}
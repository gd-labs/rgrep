# rgrep

Toy implementation of a command-line interface program that simulates the basic functionality of `grep` following the concepts presented through [The Rust Book](https://doc.rust-lang.org/book/) up until [Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html). The project focuses on combining knowledge such as code organization, vectors, strings, closures, iterators, handling errors properly, exploring the usage of traits and lifetimes and writing tests, putting those skills in practice and building a somewhat useful tool.

# Usage

The project can be compiled and built with the Cargo package manager through the `cargo build` command. In order to run the program, follow the template below:

```bash
cargo run [pattern] [file]
```

# Grep

Contextualizing the tool's functionality, given one or more patterns, `grep` searches input files for matches to the patterns. When it finds a match in a line, it copies the line to standard output or produces whatever other sort of output was requested through options.

Though `grep` expects to do the matching on text, it has no limits on input line length other than available memory, and it can match arbitrary characters within a line. If the final byte of an input file is not a newline, `grep` silently supplies one. Since newline is also a separator for the list of patterns, there is no way to match newline characters in a text.

The general look of the `grep` command line is:

```
grep [option...] [patterns] [file...]
```

There can be zero or more option arguments and file arguments. The patterns argument contains one or more patterns separated by newlines.

# Improvements

 - [ ] Add support for multiple patterns.
 - [ ] Allow pattern matching with regular expressions.
 - [ ] Accept multiple files as input.
 - [ ] Deal with case sensitivity through program arguments.

# License

This project is licensed under the [MIT License](LICENSE).
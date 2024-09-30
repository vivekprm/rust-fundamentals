//! This is a library that provides utilities for command-line tools.
//! So far it only provides a function to read a line from the standard input.
//! # Examples:
//! ```
//! use cli_utils::read_stdin;
//! let input = read_stdin();
//! ```
//! # Panics:
//! The `read_stdin` function will panic if it fails to read a line from the standard input.
use std::io::{BufRead, BufReader};
pub mod colors;
pub mod config;
/// this function reads a line from the standard input and returns it as a string
/// it will panic if it fails to read a line with message "Failed to read input line"
/// # Examples:
/// ```
/// let input = read_stdin();
/// ```
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}

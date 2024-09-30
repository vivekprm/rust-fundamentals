# Extending functionality using modules
Now to access our library into another project let's create another project called cli-tool,  where we will use our library which is not published yet.

In cli-tool Gopkg.toml add cli-utils dependency as below:
```toml
[package]
name = "cli-tool"
version = "0.1.0"
edition = "2021"

[dependencies]
cli-utils = {path="../cli-utils"}
```

Now use it ```main.rs``` as below:
```rust
use cli_utils::read_stdin;

fn main() {
    let buffer = read_stdin();

    println!("{}", buffer);
}
```

## Extending with modules
Now we will try to extend cli-utils library. cli-utils is  meant  to provide  utilities for command  line  tools. So  far we only have one  function. But instead of polluting  more functionality in lib.rs, we want to use separate file.

One common pattern for command line utilities is adding colors especially for terminal output. Let's add another file called ```colors.rs```

```rust
/// Returns a string wrapped in ANSI red color codes.
/// #Examples:
/// ```
/// use cli_utils::color::red;
/// let red_string = red("This is a red string.");
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}
```

color.rs file doesn't have any meaning for rust. Only  files that have meanings is ```lib.rs``` for libraries  and  ```main.rs``` for executables. So we will go to lib.rs  and will add  ```pub mod colors``` otherwise colors will not be available.

```rust
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
```

##  Verify code with doctest
How do we verify if everything is working in a library project as we don't have any ```main.rs``` file.

When we add documentations using  ```///``` we also make documentation executable. We can run it from vscode or using below command.
```sh
cargo test --doc --package cli-utils -- colors --nocapture
```

## Defining public and private modules
Using ```pub``` keyword makes a module function or datastructure publicly available. It can be used somewhere else besides the module where it's defined.

## Public & Private fields in Structs
Let's go and create a new module called ```config.rs``` in our cli-utils project.

```rust
//! This module contains the configuration options for the application.
//!
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains the configuration options for controlling logging.
/// # Examples:
/// ```
/// use cli_utils::config::Logging;
/// let config = Logging::new();
/// ```
///
/// Create a new `Logging` struct with custom values:
/// ```
/// use cli_utils::config::{Logging, LogLevel, LogOutput};
/// let config = Logging {
///    enabled: true,
///    level: LogLevel::Debug,
///    destination: LogOutput::File("log.txt".to_string()),
/// };
/// ```
pub struct Logging {
    enabled: bool,
    level: LogLevel,
    destination: LogOutput,
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
}
```

Now if we run the  doctest  as below:
```sh
cargo test --doc --package cli-utils -- config --nocapture
```

We get lots of errors and the reason is enabled, level, destination all are private fields. We can fix it using ```pub``` keyword or modify the documentation accordingly.
```rust
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,
}
```
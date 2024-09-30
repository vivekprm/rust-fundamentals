Often times, you might want to split things out of your project to support other projects needing similar functionality.

In this case we have a project called [resplit](https://github.com/alfredodeza/resplit). It's a CLI tool to split strings on characters or patterns, like ```cut``` CLI tool in linux.

main.rs
```rust
use resplit::Cli;
use clap::Parser;


fn main() {
    let cli = Cli::parse();
    let buffer = resplit::read_stdin();

    let result = resplit::split(buffer, &cli);
    println!("{}", result);
}
```

lib.rs
```rust
use std::io::{BufReader, BufRead};
use clap::Parser;

#[derive(Parser)]
#[command(name = "resplit")]
#[command(version = "0.0.1")]
#[command(author = "Alfredo Deza")]
#[command(about = "Split strings by one or more delimeters and return a field, like cut")]
pub struct Cli {
    #[arg(short('f'))]
    field: usize,
    #[arg(short('d'))]
    delimeter: String,
    #[arg(long)]
    debug: bool,
}

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read input line");
    line.trim().to_string()
}


pub fn split(s: String, cli: &Cli) -> String {
    let parts: Vec<&str> = s.split(&cli.delimeter).collect();
    if cli.debug {
        println!("Parts: {:?}", parts);
        println!("Indexes available starting at 0: {}", parts.len());
    }
    parts.get(cli.field).unwrap_or(&"").to_string()
}
```

So what we are going to assume here is in realworld scenario, we are goinng to extract ```read_stdin``` functionality out and make it a library.

So we are going to create a utility library so that this specific functionality will exist somewhere else in such a way that we don't need to write it here and this command line tool cann just rely on this library.

# Creating library using cargo
First let's create a folder called ```cli-utils``` and run below command from inside this folder.

```sh
cargo init --lib .
```

Let's get rid of all the code in ```src/lib.rs```  and paste below code:
```rs
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read input line");
    line.trim().to_string()
}
```

# Documenting your code
Documenting library code is very good way, not only so that you can understand what's going on here but it's easy for others to understand. We can run below commannd to see how documentation looks like.

```sh
cargo doc
```
It creates a new directory called ```target```. Go to the file ```target/doc/cli_utils/index.html``` and open in browser.

Now let's improve the documentation as below:
```rust
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
Documentation is  ```///``` however comments are ```//```.
Note we can use markdown as well while writing the documentation.

Now let's build the documentation again.

We can actually build the project, in this case, our library all the way to the top and we are going to use  ```//!``` and we can say:

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

# Using a Debugger
Let's look at below code:
```rust
fn main() {
    let mut input = String::new();
    while input.trim() != "stop" {
        println!("Please enter a word (type 'stop' to exit):");
        io::stdin::read_line(&mut input).expect("Failed to read input.");
        println!("You entered: {}", input)
    }
    println!("Goodbye!");
}
```

The problem that we see is all the inputs are being appened so probably input is being polluted. Let's try to clear it as below:
```rust
fn main() {
    let mut input = String::new();
    while input.trim() != "stop" {
        println!("Please enter a word (type 'stop' to exit):");
        io::stdin::read_line(&mut input).expect("Failed to read input.");
        println!("You entered: {}", input);
        input.clear();
    }
    println!("Goodbye!");
}
```

But now it doesn't exit on typing ```stop```. This is where we need debugger. So let's install ```CodeLLDB```  extension for vscode. Now after debugging we know that clear should happen before reading from standard input.

```rust
fn main() {
    let mut input = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Please enter a word (type 'stop' to exit):");
        io::stdin::read_line(&mut input).expect("Failed to read input.");
        println!("You entered: {}", input);
    }
    println!("Goodbye!");
}
```

# Using a Makefile
Makefile will allows us to normalize what we are doing with our project in such a way that regardless what projects are you using, you are always using a Makefile with the same commands. You'll have an abstraction that can help you build normalized processes around our tools. 

```makefile
SHELL := /bin/bash

clean:
	cargo clean
build:
	cargo build
```

We are going to default like .PHONY means that we are going to default to an action.

```makefile
SHELL := /bin/bash

.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

clean: ## Clean the project using cargo
	cargo clean

build: ## Build the project using cargo
	cargo build
```

We can add more targets  as:
```makefile
lint: ## Lint the project using cargo
	cargo clippy

format: ## Format the project using cargo
	cargo fmt
```

There is chance that rustfmt is not installed so we can use it as below:
```makefile
lint: ## Lint the project using cargo
	@rustup component add clippy 2 > /dev/null
	cargo clippy

format: ## Format the project using cargo
    @rustup component add rustfmt 2 > /dev/null
	cargo fmt
```

One thing that we really like is when we are releasing a new version of our project we  want to bump up the version. We have version defined in  ```Gopkg.toml``` so everytime instead of going to this file and changing the version, we can build an automation and put it in makefile.

```makefile
bump: ## Bump the version of the project
	@echo "Current version is: $(shell cargo pkgid | cut -d# -f2)"
	read -p "Enter new version number: " version; \
	updated_version=$$(cargo pkgid | cut -d# -f2 | sed -E "s/([0-9]+\.[0-9]+\.[0-9]+)$$/$$version/"); \
	sed -i -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml
	@echo "New version is $(shell cargo pkgid | cut -d# -f2)"%
```


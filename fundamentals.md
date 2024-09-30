# Creating new Project
Make sure cargo is installed. We  cann use cargo to created projects or we can use filesystem and create all files and folders manually.

```sh
cargo init .
```
This creates a binary project
Creates project in current directory and names the project as same as directory name. It creates:
- Cargo.toml
- src/main.rs

Inn case we want  to create library use:
```sh
cargo init . --lib
```

Now instead of ```main.rs``` we will have ```lib.rs```

What if  we want to create binary or library in it's own subdirectory?

```sh
cargo new foo
```

Cargo.lock pins the libraries and their exact versions being used.

# Basics
## Use Keyword
use keyword is used to import a library.  E.g.

```rust
use clap::Parser
```

Brings clap Parser in scope. :: tells that Parser is part of clip.

## Function
Function declaration starts with ```fn``` keyword. Below is a sample function.

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

## Variables
We use ```let``` keyword to define variables.

```rust
let cli = Cli::parse();
```

Consider below main function
```rust
fn main() {
    let message = "Name: Alfredo, Weight: ";
    let weight = 190.0;

    let kilos = weight / 2.2;
    println!("{}{}", message, kilos);
}
```

All these variables like message, weight & kilos  are immutable. E.g.  below gives error:
```rust
fn main() {
    let message = "Name: Alfredo, Weight: ";
    message.clear();
}
```

```message.clear()``` gives error because message is immutable variable.

If we want to make it mutable use ```mut``` keyword.
```rust
fn main() {
    let mut message = "Name: Alfredo, Weight: ";
    message.clear();
}
```

### Shadowing
```rust
fn main() {
    let mut height = 190;
    height = height - 20;

    let result = if height > 180 {
        println!("Tall")
    } else if height > 170 {
        println!("Average!")
    } else {
        println!("Short!")
    };
    println!("Result: {}", result)

    let health = if height < 180 {"good"} else {"unknown"}
    println!("Health: {}", health)

    // Shadowing to different  type
    let health = if height < 180 {true} else {false}
}
```

We can see all the print statements in the conditional doesn't have semicolon. In case of health why rust is allowing us to reassign. Because here we are redefining health with different type, which is called **shadowing**.

One of the benifits of using rust is that it's strongly typed language, changing the   types  on  the fly like this might not be good idea.

## Visibility
By default everything is private. In case we want a funnction to be public we add ```pub``` keyword.

```rust
pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

If we don't have a semicolon at the end of a line in a function , we are returing that.
```rust
pub fn greet(name: &str) {
    name = "vivek";
    name.trim().to_string()
}
```

# Control Flow
```rust
fn main() {
    let proceed = true;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not Proceeding");
    }

    let height = 190;
    if height > 180 {
        println!("Tall")
    } else if height > 170 {
        println!("Average!")
    } else {
        println!("Short!")
    }
}
```

## Other types of Conditionals
```rust
fn main() {
    let mut maybe_number = Some(42)
    lf let Some(number) == maybe_number {
        println!("The number is {}", number)
    } else {
        println!("There is no number")
    }
}
```

We can run below command to check if something is wrong with the code..
```sh
cargo check
```

We get warning that ```maybe_number``` doesn't need to be mutable.
```Some``` is a wrapper over an option. It gives either one of two possibilities. Either an actual value or you will get ```None```.

##  match control flow
```rust
fn main() {
    println!("Please enter a greeting: ");
    let mut name  = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    // use of match expression to pattern match against variable "name"
    match name.trim() {
        "Good Bye" => println!("Sorry to see you go."),
        "Hello" => println!("Hi, nice to meet you!"),
        _ => println!("I can't find a greeting, good bye.")
    }
}
```

# Loops 
## Using loops keyword
```rust
fn main() {
    let mut x = 1;
    // continue looping until x > 5
    loop {
        println!("x is {}", x);
        x += 1;
        if x > 5 {
            break;
        }
    }
}
```

It is similar to ```while(true)``` in other languages. In this case we have to add  break condition to prevent it from running indefinitely.

## Using while loop
```rust
fn main() {
    let mut i = 0;
    while i < 5 {
        println!("i = {}", i)
        i += 1
    }
}
```

Let's look at other example:
```rust
// This example is a useful application of `while` because it allows to continue
// asking for user input until the user types a specific word (in this case, stop)
fn main() {
    let mut input = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Please enter a word (type 'stop' to exit): ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        println!("You entered: {}", input);
    }
    println!("Goodbye!");
}
```

## For Loop
```rust
fn main() {
    for i in 1..10 {
        println!("i = {}", i)
    }

    for i in (1..=5).rev() {
        println!("{}", i)
    }

    let numbers = vec![1, 2, 3, 4, 5];
    for n in numbers {
        println!("{}", n);
    }
}
```

# Functions
Rust is a functional programming language.

##  Unit Functions
Functions that doesn't return anything is called unit function. E.g. in below function we are passing a slice.

```rust
fn process_numbers(numbers: &[i32]) {
    // Initialize the sum to 0.
    let mut sum = 0;

    // Iterate over the numbers, adding each to the sum.
    for number in numbers {
        sum += number;
    }

    // Print the sum.
    println!("The sum of the  nnumbers is: {}", sum);

    // If  the sum is even, print a message.
    if sum % 2 == 0 {
        println!("The sum is even.");
    } else {
        println!("The sum is odd.");
    }
}

fn main() {
    process_numbers(&[1,2,3])
}
```

In this case process_numbers is a unit function.

## Functions with return values
For example in below code funnction ```split_string``` is returing a string

```rust
fn  split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);
    // This will not  compile
    result.to_string()
}

fn main() {
    let chunk = split_string("Hello,world!".to_string(), ',', 1);
    println!("Split string: {}", chunk);
}
```

Last line in split_string doesn't comile because result in this case is Option enumerator, it can have more than one value. Instead changing it to below will work:
```rust
fn  split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);
    // This will not  compile
    result.expect("oops! something went wrong.").to_string()
}
```

## Using Arguments
```rust
fn  sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {
    // There are no variadic arguments in Rust
    let numbers  = [1, 2, 3, 4, 5];
    let result = sum(&numbers);
    println!("Sum of numbers: {}", result);
}
```

# Borrowing
It's hard thing to grasp if you are  coming from non-statically defined languages.

Borrowing is key concept in Rust because it allows you to write code that is both safe and efficient.
By lending ownership of a variable instead of transferring it, Rust ensures that only
one part of your program can modify the variable at a time, which helps prevent 
bugs and make it easier to reason about your code.

```rust
fn own_integer(x: i32) {
    x + 1;
}

fn own_string(s: String) {
    println!("{}", s);
}

fn own_vec(mut vector: Vec<i32>) {
    vector.push(10);
    println!("{:?}", vector);
}

// Borrowing is a mechanism by which Rust allows to lend ownership of a variable to a
// function or another part of your program without actually transferring the ownership 
// of the variable.
// When you borrow a variable, you are essentially saying "I want to use this variable for a little while, but I promise I won't modify it."
fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let my_int = 10;
    let my_string = String::from("Hello, World!");

    // This compiles no problem
    own_integer(my_int);
    println!("{}", my_int);

    own_string(my_string); // take ownership of my_string
    // this is using my_string which has also moved and is innvalid
    // println!("{:?}", my_string); // this will not compile

    own_vec(my_vec);
    // But this is using my_vec which was borrowed (moved) and yet is now invalid
    // println!("{:?}", my_vec); // this will not compile
}
```

Same thing that we are doing with int is not working for string. We are just passing a string value and printing it not modifying it.

With strings unlike boolean and integers this is not allowed. Behind the scenes for integers rust is going to copy the values because it's very cheap and we don't need to worry about. However, size of the string is not known at copilation time. So rust can't be confident about copying its value.

So to make it work we modify own_string to borrow ```my_string``` parameter not own it (as in this case). So to fix it, we change it as below:

```rust
fn own_string(s: &String) {
    println!("{}", s);
}
```

However this doesn't work with vector becuase we are modifying it. If we borrow we can use it but can't modify. To make it work we need to create a new vector.

```rust
fn own_vec(mut vector: &Vec<i32>) {
    // create new vector
    let mut new_vector = Vec::new();
    new_vector.push(10);
    println!("{:?}", new_vector);
}
```

**Note**:
- ***We can't borrow same thing as mutable several times, we can do that only once.***
- ***If we borrowed something as  mutable we cann't borrow  the same thing as immutable again***

# Panic
```rust
fn loop_and_panic(numbers: Vec<i32>) {
    for num in numbers {
        if num < 0 {
            panic("Negative number found!");
        }
        println!("Number: {}", num)
    }
}

fn main() {
    loop_and_panic(vec![1, 2, 3, 4, -5])
}
```

Panic halts execution when we get unexpected input and don't want to continue. However, panics are fine for example/demo code but frowned upon in production grade code, use it sparingly.

# Basic Error handling with match
```rust
use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("non_existent_file.txt");
    let file = match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        },
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("File not found: {}", error);
                }
                _ => {
                    println!("Error opening file: {}", error);
                }
            }
        },
    };
}
```

# Using structured data
It's a collection of related items

## Defining structs
```rust
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

fn main() {
    println!("?", Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 30,
    });
}
```

However, we can't print like that, we need debug attribute to do that. To make it work:
```rust
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

fn main() {
    println!("{:?}", Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 30,
    });
}
```

## Creating struct instances
```rust
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

fn main() {
    let alfredo = Person {
        first_name: "Alfredo".to_string(),
        last_name: "Martinez".to_string(),
        age: 25,
    };
    println!("The person's first name is: {}", alfredo.first_name);
    println!("The person's last name is: {}", alfredo.last_name);
    println!("The person's age is: {}", alfredo.age);
}
```

By the way all the fields in above struct are required. If we want a field to be optional we can say Option.

```rust
struct Person {
    first_name: String,
    last_name: String,
    age: Option<u8>,
}

let alfredo = Person {
        first_name: "Alfredo".to_string(),
        last_name: "Martinez".to_string(),
        age: None,
    };
```

In case we want to pass some value, we can define person as below:
```rust
let alfredo = Person {
    first_name: "Alfredo".to_string(),
    last_name: "Martinez".to_string(),
    age: Some(25),
};
println!("The person's age is: {:?}", alfredo.age);
```

## Associated Functions & Constructors
Constructor is a powerful way of automating tedious, repeatative things and provide an easy way to create an instance of struct.

We do that in rust with **associated funnction**. We define it using ```impl``` keyword.

```rust
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: &str, email: &str, uri: &str, active: bool) -> User {
        User {
            username: username.to_string(),
            email: email.to_string(),
            uri: uri.to_string(),
            active: active,
        }
    }
}

fn main() {
    let new_user = User::new(
        "alfredodeza",
        "alfredodeza@example.com",
        "https://alfredodeza.com",
        true,
    );
    println!("Hello, {}!", new_user.username);
}
```

The new function is convention create a new instance of struct ```User```.
Retruning a struct type is possible from the constructor as in this case but we can also return ```self```.

```rust
impl User {
    fn new(username: &str, email: &str, uri: &str, active: bool) -> Self {
        Self {
            username: username.to_string(),
            email: email.to_string(),
            uri: uri.to_string(),
            active: active,
        }
    }

    fn deactivate(&mut self) {
        self.active = false
    }
}
```

What does ```Self``` refers to?  It refers to User. Now we can use it to e.g. deactivate the user.

```rust
impl User {
    fn new(username: &str, email: &str, uri: &str, active: bool) -> Self {
        Self {
            username: username.to_string(),
            email: email.to_string(),
            uri: uri.to_string(),
            active: active,
        }
    }
    fn deactivate(&mut self) {
        self.active = false
    }
}
fn main() {
    let mut new_user = User::new(
        "alfredodeza",
        "alfredodeza@example.com",
        "https://alfredodeza.com",
        true,
    );
    println!("Username: {}", new_user.username);
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );
    new_user.deactivate();
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );
}
```

## Other struct uses
We can define instance in another way as below:
```rust
fn main() {
    let  username = String::from("alfredodeza")
    let  email = String::from("alfredodeza@example.com")
    let  uri = String::from("https://alfredodeza.com")
    let active = true;
    let user = User{username, email, uri, active};
}
```

Structs can also be created as touples.

```rust
struct Point(i32, i32, i32);
```

What does this mean?
It means that we can access these touple's fields using indexes.

```rust
fn main() {
    let my_point = Point(10, 20, 30);
    println!("Points: {}", my_point.0)
}
```

# Strings
There are two primary types of strings:
- One is string slices.
- Other one just String

What's the difference?
```rust
fn print_str(s: &str) {
    println!("{}", s);
}

fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s = "Hello, world!";
    print_str(s);

    let salutation = String::from("hello");
    print_string(salutation);
}
```

In the example above, first one ```&str``` is string slice. It can't be mutated/changed.
If we want to change it we can create a new string as below:

```rust
fn print_str(s: &str) {
    let new_string = s.to_string();
    new_string.push_str("some other string");
    println!("{}", new_string);
}
```

We can also  use formatting:
```rust
fn print_str(s: &str) {
    let new_string = format!("{}! other stuff here", s);
    println!("{}", new_string);
}
```

salutation string above is not mutable by default but we can make it mutable using ```mut``` keyword.

```rust
fn print_string(mut s: String) {
    println!("{}", s);
}
fn main() {
    let mut salutation = String::from("hello");
    print_string(salutation);
}
```

In general, we should use string over string slices when we need to create string that can grow or change over time.

So, if this main function was something like command line tool and you have argument parsing as input, you'll probably want to use a string, you wouldn't be able to use string slice because string slice is known at compilation time and regular string is not. 

## String Operations
There are many operations we can do with a string.

### Slicing
```rust
fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog".to_string();
    // use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..4]);
}
```

### Concatenating
```rust
fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog".to_string();
    let description = format!("Title: Quick Story\n{}", sentence);
    println!("{}", description);
}
```

### Iterating
```rust
fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog".to_string();
    let description = format!("Title: Quick Story\n{}", sentence);

    for c in description.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel: {}", c),
            _ => continue,
        }
    }
}
```

### Split into words
```rust
fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog".to_string();
    let description = format!("Title: Quick Story\n{}", sentence);

    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("Words: {:?}", words);
}
```

We can also do it as below:
```rust
let words = sentence.split(' ').collect()::<Vec<&str>>;
println!("Words: {:?}", words);
```

###  Reverse
```rust
fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog".to_string();

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);
}
```

#  Vectors
Vectors and slices in Rust have similar relationship as have String & String Slices. Let's look at some of the differences.

```rust
fn ownership() {
    let numbers = vec![1, 2, 3];
    let slice = &numbers[..];
    println!("Slice = {:?}", slice);
}
fn modifiable() {
    let mut numbers = vec![1, 2, 3];
    let slice = &mut numbers[..]; // Create a slice of all elements in the numbers
    slice[0] = 10;
    // This would fail
    // let other_slice = &numbers[..];
    println!("Slice = {:?}", slice);
}

fn main() {
    // slices & vectors are similar. But slices are immutable depending upon
    // how they are borrowed.
    ownership();
    modifiable();
}
```

```numbers[..]``` means all the numbers inn numbers vector are going to be copied. 
**We can't borrow same thing as mutable several times, we can do that only once.**
**If we borrowed something as mutable we can't borrow  the same thing as immutable again**

## Retrieving Values from Vectors
```rust
fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve value at a specific index
    let third_value = vec[2];
    println!("Third value: {}", third_value);

    // Retrieve the last value
    let last_value = vec.last().unwrap();
    println!("Last value: {}", last_value);

    // Retrieve the first value using pattern matching
    match vec.first() {
        Some(first_value) => println!("First value: {}", first_value),
        None => println!("Vector is empty"),
    }
}
```

**unwrap** is needed while getting the last value, because either you can get a value or ```None``` in case vector is empty. This is the safe way to getting the last value without getting into trouble.

Let's say we want to do something with the values we are getting, we can follow the last pattern.

```rust
fn get_item() {
    let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve value at a specific index
    let value = vec.get(index);
    println!("Value at index {} is: {:?}", index, value);
}

fn main() {
    get_item();
}
```

In the above example we get output as ```Some(4)```. If we do ```vec.get(index).unwrap()``` we will get the actual number.

But what is we wanted to pass the index from client.
```rust
fn get_item(index: u8) {
    let vec = vec![1, 2, 3, 4, 5];
    // Retrieve value at a specific index
    let value = vec.get(index);
    println!("Value at index {} is: {:?}", index, value);
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    get_item(3);
}
```
We can't do that because type of index of slice should be ```usize``` not ```u8```

## Adding elements to Vectors
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v);

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("{:?}", v);

    // append adds the given vector to the end of the vector, requires vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert item at given index
    v.insert(0, 0);
    println!("{:?}", v);
}
```

```extend``` is similar to ```append``` but append doesn't necessarily take an iterator. So ```append``` takes in another vector which is mutable. 

# Enums & Variants
```rust
enum DiskType {
    SSD,
    HDD,
}

#[derive(Debug)]
enum DiskSize {
    GB(u32),
    TB(u32),
}

fn main() {
    let disk_type = DiskType::SSD;
    //  Can't compare them like below
    if disk_type == DiskType::SSD {
        println!("Disk type is SSD");
    } else {
        println!("Disk type is HDD");
    }
}
```

We can't compare enums as above, instead use ```match```
```rust
fn main() {
    let disk_type = DiskType::SSD;
    match disk_type {
        DiskType::SSD => println!("Solid State Drive"),
        DiskType::HDD => println!("Hard Disk Drive"),
    }
}
```

We can also assign some data to it as:
```rust
fn main() {
    let disk_size = DiskSize::GB(128);
    println!("{:?}", disk_size);
}
```

```SSD``` & ```HDD```  are called variants of DiskType enum.

## Using Enums as type
Enums in Rust are a custom data type and we can use it as a type.

```rust
#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Tuscany,
    Champagne,
    Rioja,
    Rhone,
}

struct Wine {
    name: String,
    region: WineRegions,
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
}
```

We can make it a requirement for functions, we can say, hey, I am  going to call a function and what we are going to accept is a wine. So, instead of print statements above we can say:
```rust
fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} Not supported", w),
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };
    supported_regions(wine1.region);
    supported_regions(wine2.region);
    supported_regions(WineRegions::Rioja);
}
```

## The Option Enum
```rust
fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None    //  This is valid because it is the other variant of Option
    } else {
        Some(x / y) // Creates the Option<i32> value. Some() creates a new instance of Option
    }
}

fn main() {
    let x = 10;
    let y = 2;

    let result = divide(x, y);
    println!("The result is {:?}", result.unwrap()); // Will panic if result is None
    match result {
        Some(x) => println!("The result is {}", x),
        None => println!("Error: division by zero"),
    }
}
```

Option enum is not only useful in dealing with possible problems and errors, but also match & ```Some```, ```Some``` is used to create an instance of Option.  

```Some``` is also an enumerator that has a variant that wraps a value of some type T. That means it can take any value and return Option type. 

##  Applied Enums
Let's take a real world example.

```rust
use std::result;

enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

fn format_size(size: u64) -> String {
    let file_size = match size {
        0..=1024 => FileSize::Bytes(size),
        1025..=1048576 => FileSize::Kilobytes(size / 1024),
        1048577..=1073741824 => FileSize::Megabytes(size / 1024 / 1024),
        1073741825..=1099511627776 => FileSize::Gigabytes(size / 1024 / 1024 / 1024),
        _ => FileSize::Gigabytes(size / 1024 / 1024 / 1024),
    };
    match file_size {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64 / 1024.0),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64 / 1024.0),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64 / 1024.0),
    }
}

fn main() {
    let result = format_size(688883007399);
    println!("{}", result);
}
```

We can also create associated method to enum. Above example can be converted as below:

```rust
enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

impl FileSize {
    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{:.2} KB", *kb as f64 / 1024.0),
            FileSize::Megabytes(mb) => format!("{:.2} MB", *mb as f64 / 1024.0),
            FileSize::Gigabytes(gb) => format!("{:.2} GB", *gb as f64 / 1024.0),
        }
    }
}

fn main() {
    let size = 6388883007399;
    let file_size = match size {
        0..=1024 => FileSize::Bytes(size),
        1025..=1048576 => FileSize::Kilobytes(size / 1024),
        1048577..=1073741824 => FileSize::Megabytes(size / 1024 / 1024),
        1073741825..=1099511627776 => FileSize::Gigabytes(size / 1024 / 1024 / 1024),
        _ => FileSize::Gigabytes(size / 1024 / 1024 / 1024),
    };
    let result = file_size.format_size();
    println!("File size: {}", result);
}
```

## Using Vectors with Enums
Sometimes enum take some data..

```rust
enum Shape {
    Circle(f64),
    Square(f64),
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0)];

    let total_area: f64 = shapes
        .iter() // to iterate in place
        .map(|shape| match shape { // |shape| is used to extract values from iterator
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(side) => side * side,
        })
        .sum();
    println!("Total area: {} sq. units", total_area);
}
```

## Exhaustive Matches
```rust
enum WineGrapes {
    CabernetFranc,
    Tannat,
    Merlot,
}

fn taste_wine(grapes: WineGrapes) {
    match grapes {
        WineGrapes::CabernetFranc => println!("This is a Cabernet Franc wine."),
        // WineGrapes::Tannat => println!("This is a Tannat wine."),
        // WineGrapes::Merlot => println!("This is a Merlot wine."),
    }
}

fn main() {
    taste_wine(WineGrapes::CabernetFranc);
}
```

Why do we get compilation error on ```match grapes {```. If an enum provides 3 variants, rust wants us  to  deal with all three variants in match expression. So to fix that uncomment other match cases i.e. exhaustive matches. Or we can give a default case as below:

```rust
match grapes {
    WineGrapes::CabernetFranc => println!("This is a Cabernet Franc wine."),
    _ => println!("This is not a Cabernet Franc wine."),
}
```
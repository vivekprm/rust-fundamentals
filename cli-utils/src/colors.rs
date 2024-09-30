//! Colorized output utilities for the terminal using ANSI escape codes.
//! # Examples:
//! ```
//! use cli_utils::colors::*;
//! println!("{}{}{}", red("Red"), green("Green"), blue("Blue"));
//! ```

/// Returns a string wrapped in ANSI red color codes.
/// #Examples:
/// ```
/// use cli_utils::colors::*;
/// let red_string = red("Red");
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

pub enum Color {
    Red,
    Green,
    Blue,
    Bold,
}

pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized: String,
}

impl ColorString {
    // Create a method that will use the string and color fields to  create colorized field
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string),
        }
    }

    pub fn reset(&mut self) {
        self.colorized = reset(&self.string);
    }
}

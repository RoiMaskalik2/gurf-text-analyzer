//! User input handling module.
//!
//! includes functions for  user input from the command line.

use crate::{Error, Result};
use std::io;

/// Reads a string from the standard input, trims it, and checks that the input is not empty.
pub fn input_string(display_message: &str) -> Result<String> {
    println!("{}", display_message);

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    user_input.truncate(user_input.trim_end().len());

    (!user_input.is_empty())
        .then_some(())
        .ok_or(Error::EmptyString)?;

    Ok(user_input)
}

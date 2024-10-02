// This module contains functions for handling input/output operations.

// Import the io module from the standard library.
use std::io;

// Reads user input from standard input and returns it as a String.
pub fn read_user_input() -> String {
    // Create a mutable String to store the user input.
    let mut input_buffer = String::new();

    // Read user input from standard input into the buffer.
    // Returns a Result indicating success or an error.
    if let Err(error) = io::stdin().read_line(&mut input_buffer) {
        // If an error occurred, print the error message and return an empty String.
        eprintln!("Error reading input: {}", error);
        return String::new();
    }

    // Return the input buffer.
    input_buffer
}

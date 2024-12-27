//! Utility functions for the game

use std::io::{self, Write};

/// Clears the terminal screen
pub fn clear_terminal() {
    // Check the OS and clear the screen accordingly
    if cfg!(target_os = "windows") {
        // For Windows, we can use `cls`
        print!("{}[2J", 27 as char); // ANSI escape sequence to clear the screen
        io::stdout().flush().unwrap();
    } else {
        // For Unix-like OS (Linux, macOS, etc.), use the clear command
        print!("\x1B[2J\x1B[H"); // ANSI escape sequence to clear the screen and move cursor to top
        io::stdout().flush().unwrap();
    }
}

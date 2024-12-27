//! Utility functions for the game

use clap::Parser;
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

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The names of the players participating in the game.
    #[arg(short, long, value_parser, num_args = 2..=26, value_delimiter = ' ')]
    pub players: Vec<String>,

    /// The number of rows on the game board.
    #[arg(short, long, default_value = "6")]
    pub rows: usize,

    /// The number of columns on the game board.
    #[arg(short, long, default_value = "7")]
    pub cols: usize,

    /// The number of connected tokens required to win the game.
    #[arg(short, long, default_value = "4")]
    pub tokens_to_win: usize,
}

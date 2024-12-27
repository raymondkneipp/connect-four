//! This module contains the `Player` struct, which represents a player in the game.
//!
//! Each player has a unique name and token. The token is a single character that represents the player on the game board.
//!
//! ## Example
//!
//! ```
//! use connect_four::Player;
//!
//! let player = Player::new("Alice");
//! println!("Player name: {}", player.name);
//! println!("Player token: {}", player.token);
//! ```

use once_cell::sync::Lazy;
use std::sync::Mutex;

/// The next token to be assigned to a player.
///
/// This is a thread-safe static variable that is shared across all players.
static NEXT_TOKEN: Lazy<Mutex<char>> = Lazy::new(|| Mutex::new('a'));

#[derive(Clone, PartialEq, Debug)]
/// Represents a player in the game.
pub struct Player {
    /// This `name` field represents the name of the player.
    pub name: String,
    /// This `token` field represents the token of the player that is used on the game board.
    pub token: char,
}

impl Player {
    /// Creates a new player with the given name and generates a token.
    pub fn new<S: Into<String>>(name: S) -> Self {
        let name = name.into();
        if name.is_empty() {
            panic!("Player must have a name.")
        }

        let mut token_lock = NEXT_TOKEN.lock().unwrap();
        let token = *token_lock;

        // Increment token for the next player
        if *token_lock < 'z' {
            *token_lock = (token as u8 + 1) as char;
        }

        Self { name, token }
    }
}

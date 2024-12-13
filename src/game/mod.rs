// Declare the `game.rs` file as a module
pub mod game;
// Declare the `board.rs` file as a module
pub mod board;

pub mod player;

// Re-export key types for easier access
pub use game::Game;
pub use player::Player;

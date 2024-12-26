// Declare the `game.rs` file as a module
pub mod game;
// Declare the `board.rs` file as a module
pub mod board;
// Declare the `player.rs` file as a module
pub mod player;
// Declare the `util.rs` file as a module
pub mod util;

// Re-export key types for easier access
pub use game::Game;
pub use player::Player;

//! Game module
//!
//! Contains the `Game` struct and related implementations.
//!
//! The `Game` struct represents a game of Connect Four. It contains a `Board` and a list of `Player`s.
//!
//! The `Game` struct is responsible for managing the game state, player turns, and game status.
//!

use crate::game::util;
use crate::Player;
use std::{collections::HashSet, io};

use super::board::{Board, BoardCell};

/// Represents the status of a game.
pub enum GameStatus {
    /// The game is still in progress.
    Ongoing,
    /// The game ended in a tie.
    Draw,
    /// A player has won the game.
    Win(Player),
}

/// Represents a game of Connect Four.
pub struct Game {
    /// The game board state.
    pub board: Board,
    /// The number of tokens in a row required to win the game.
    pub tokens_to_win: usize,
    /// The list of players in the game.
    pub players: Vec<Player>,
    /// The index of the current player's turn.
    current_turn: usize,
}

impl Game {
    /// Creates a new game of Connect Four.
    ///
    /// ## Panics
    ///
    /// - Panics if the number of players is less than 2.
    /// - If there are duplicate tokens among players.
    /// - If the game configuration is invalid:
    /// - If the number of players is too many for the board size
    pub fn new(
        row_count: usize,
        col_count: usize,
        tokens_to_win: usize,
        players: Vec<Player>,
    ) -> Self {
        if players.len() < 2 {
            panic!("Must have at least 2 players.")
        }

        Self::validate_players(&players);

        if let Err(err) = Self::validate_game_config(row_count, col_count, tokens_to_win) {
            panic!("Invalid game configuration: {}", err);
        }

        if players.len() * tokens_to_win > row_count * col_count {
            panic!("Too many players for the board size.");
        }

        Self {
            board: Board::new(row_count, col_count),
            tokens_to_win,
            players,
            current_turn: 0,
        }
    }

    /// Advances the game to the next turn.
    fn next_turn(&mut self) {
        // Increment current turn and loop back to 0 if at the end
        self.current_turn = (self.current_turn + 1) % self.players.len();
    }

    /// Validates that there are no duplicate tokens among players.
    fn validate_players(players: &[Player]) {
        let mut seen_tokens = HashSet::new();

        for player in players {
            if !seen_tokens.insert(player.token) {
                panic!("Duplicate token found for player: {}", player.name);
            }
        }
    }

    /// Validates the game configuration.
    ///
    /// Returns an error message if the configuration is invalid.
    ///
    /// # Errors
    /// - If `rows` or `cols` is less than 1.
    /// - If `tokens_to_win` is less than 2.
    /// - If `tokens_to_win` is greater than `rows` or `cols`.
    fn validate_game_config(rows: usize, cols: usize, tokens_to_win: usize) -> Result<(), String> {
        if rows < 1 || cols < 1 {
            return Err("Rows and columns must be greater than 0.".to_string());
        }
        if tokens_to_win < 2 {
            return Err("Tokens to win must be at least 2.".to_string());
        }
        if tokens_to_win > rows || tokens_to_win > cols {
            return Err("Tokens to win cannot be greater than rows or columns.".to_string());
        }

        Ok(())
    }

    /// Prompts the current player for a valid column input.
    /// Returns the column number entered by the player.
    fn get_valid_input(&self) -> usize {
        loop {
            let mut input_line = String::new();
            println!("Please enter a column to play: ");

            if io::stdin().read_line(&mut input_line).is_err() {
                println!("Failed to read input. Please try again.");
                continue;
            }

            match input_line.trim().parse::<usize>() {
                Ok(value) if self.board.valid_move(value) => return value,
                Ok(_) => println!("Invalid move. Column is either full or out of range."),
                Err(_) => println!("Invalid input. Please enter a valid integer."),
            }
        }
    }

    /// Checks a line for a winner.
    fn check_line(line: &[BoardCell], tokens_to_win: usize) -> BoardCell {
        let mut count = 0;
        let mut last_player: BoardCell = None;

        for cell in line {
            if let Some(player) = cell {
                if Some(player) == last_player.as_ref() {
                    count += 1;
                    if count == tokens_to_win {
                        return Some(player.clone());
                    }
                } else {
                    count = 1;
                    last_player = Some(player.clone());
                }
            } else {
                count = 0;
                last_player = None;
            }
        }

        None
    }

    /// Finds the winner of the game.
    /// Returns the winning player if there is a winner, otherwise returns None.
    fn find_winner(&self) -> BoardCell {
        // Check rows for winner
        for row in &self.board.rows {
            if let Some(winner) = Self::check_line(row, self.tokens_to_win) {
                return Some(winner);
            }
        }

        // Check columns for winner
        for col in 0..self.board.rows[0].len() {
            let column: Vec<_> = self.board.rows.iter().map(|row| row[col].clone()).collect();
            if let Some(winner) = Self::check_line(&column, self.tokens_to_win) {
                return Some(winner);
            }
        }

        // Check top-left to bottom-right diagonals for winner
        for diagonal in self.board.get_diagonals_top_left_to_bottom_right() {
            if let Some(winner) = Self::check_line(&diagonal, self.tokens_to_win) {
                return Some(winner);
            }
        }

        // Check top-right to bottom-left diagonals for winner
        for diagonal in self.board.get_diagonals_top_right_to_bottom_left() {
            if let Some(winner) = Self::check_line(&diagonal, self.tokens_to_win) {
                return Some(winner);
            }
        }

        // No winner found
        None
    }

    /// Determines the status of the game.
    fn status(&self) -> GameStatus {
        if self.board.is_board_full() {
            return GameStatus::Draw;
        }

        match self.find_winner() {
            Some(winner) => GameStatus::Win(winner),
            None => GameStatus::Ongoing,
        }
    }

    /// Starts the game loop.
    /// The game will continue until a player wins or the game ends in a draw.
    pub fn start(&mut self) {
        loop {
            util::clear_terminal();

            println!("{}", self.board.display());

            match self.status() {
                GameStatus::Ongoing => (),
                GameStatus::Draw => {
                    println!("Draw!");
                    break;
                }
                GameStatus::Win(player) => {
                    println!("The winner is: {} ({})", player.name, player.token);
                    break;
                }
            }

            println!(
                "{}'s ({}) Turn",
                self.players[self.current_turn].name, self.players[self.current_turn].token
            );

            let input_col = self.get_valid_input();
            self.board
                .place_token(input_col, self.players[self.current_turn].clone());

            // chance turn
            self.next_turn();
        }
    }
}

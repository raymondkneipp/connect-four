use game::{Game, Player};

pub mod game {
    use std::{collections::HashSet, io};

    // use board::Board;
    pub type BoardRow = Vec<BoardCell>;
    pub type BoardCell = Option<Player>;
    pub type Board = Vec<BoardRow>;

    pub enum GameStatus {
        Ongoing,
        Draw,
        Win(Player),
    }

    pub struct Game {
        pub board: Board,
        pub tokens_to_win: usize,
        pub players: Vec<Player>,
        current_turn: usize,
    }

    #[derive(Clone, PartialEq)]
    pub struct Player {
        pub name: String,
        pub token: char,
    }

    impl Player {
        pub fn new<S: Into<String>>(name: S, token: char) -> Self {
            let name = name.into();
            if name.is_empty() {
                panic!("Player must have a name.")
            }

            Self { name, token }
        }
    }

    impl Game {
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

            Self {
                board: generate_board(row_count, col_count),
                tokens_to_win,
                players,
                current_turn: 0,
            }
        }

        fn next_turn(&mut self) {
            // Increment current turn and loop back to 0 if at the end
            self.current_turn = (self.current_turn + 1) % self.players.len();
        }

        fn validate_players(players: &[Player]) {
            let mut seen_tokens = HashSet::new();

            for player in players {
                if !seen_tokens.insert(player.token) {
                    panic!("Duplicate token found for player: {}", player.name);
                }
            }
        }

        fn display_board(&self) {
            for col in 0..self.board[0].len() {
                print!(" {} ", col);
            }

            println!();

            for row in &self.board {
                for col in row {
                    match col {
                        Some(player) => print!("[{}]", player.token),
                        None => print!("[ ]"),
                    }
                }

                println!();
            }
        }

        fn is_board_full(&self) -> bool {
            for row in &self.board {
                if row.iter().any(|cell| cell.is_none()) {
                    return false;
                }
            }

            true
        }

        fn valid_move(&self, col: usize) -> bool {
            if col >= self.board[0].len() {
                return false;
            }

            // check if column is full
            self.board.iter().any(|row| row[col].is_none())
        }

        fn get_valid_input(&self) -> usize {
            loop {
                let mut input_line = String::new();
                println!("Please enter a column to play: ");

                if io::stdin().read_line(&mut input_line).is_err() {
                    println!("Failed to read input. Please try again.");
                    continue;
                }

                match input_line.trim().parse::<usize>() {
                    Ok(value) if self.valid_move(value) => return value,
                    Ok(_) => println!("Invalid move. Column is either full or out of range."),
                    Err(_) => println!("Invalid input. Please enter a valid integer."),
                }
            }
        }

        fn get_diagonals_top_right_to_bottom_left(&self) -> Board {
            let mut diagonals = Vec::new();
            let rows = self.board.len();
            let cols = self.board[0].len();

            for d in 0..(rows + cols - 1) {
                let mut diagonal = Vec::new();
                for i in 0..rows {
                    let j = (cols as isize - 1) - (d as isize - i as isize);
                    if j >= 0 && (j as usize) < cols {
                        diagonal.push(self.board[i][j as usize].clone());
                    }
                }
                if !diagonal.is_empty() {
                    diagonals.push(diagonal);
                }
            }

            diagonals
        }

        fn get_diagonals_top_left_to_bottom_right(&self) -> Board {
            let mut diagonals = Vec::new();
            let rows = self.board.len();
            let cols = self.board[0].len();

            for d in 0..(rows + cols - 1) {
                let mut diagonal = Vec::new();
                for i in 0..rows {
                    let j = d as isize - i as isize;
                    if j >= 0 && (j as usize) < cols {
                        diagonal.push(self.board[i][j as usize].clone());
                    }
                }
                if !diagonal.is_empty() {
                    diagonals.push(diagonal);
                }
            }

            diagonals
        }

        fn check_line(line: &[BoardCell], tokens_to_win: usize) -> BoardCell {
            let mut count = 0;
            let mut last_player: Option<Player> = None;

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

        fn find_winner(&self) -> BoardCell {
            // Check rows for winner
            for row in &self.board {
                if let Some(winner) = Self::check_line(row, self.tokens_to_win) {
                    return Some(winner);
                }
            }

            // Check columns for winner
            for col in 0..self.board[0].len() {
                let column: Vec<_> = self.board.iter().map(|row| row[col].clone()).collect();
                if let Some(winner) = Self::check_line(&column, self.tokens_to_win) {
                    return Some(winner);
                }
            }

            // Check top-left to bottom-right diagonals for winner
            for diagonal in self.get_diagonals_top_left_to_bottom_right() {
                if let Some(winner) = Self::check_line(&diagonal, self.tokens_to_win) {
                    return Some(winner);
                }
            }

            // Check top-right to bottom-left diagonals for winner
            for diagonal in self.get_diagonals_top_right_to_bottom_left() {
                if let Some(winner) = Self::check_line(&diagonal, self.tokens_to_win) {
                    return Some(winner);
                }
            }

            // No winner found
            None
        }

        fn status(&self) -> GameStatus {
            if self.is_board_full() {
                return GameStatus::Draw;
            }

            match self.find_winner() {
                Some(winner) => GameStatus::Win(winner),
                None => GameStatus::Ongoing,
            }
        }

        fn place_token(&mut self, col: usize, player: Player) {
            let mut target_row = 0;

            for (i, row) in self.board.iter().enumerate() {
                if row[col].is_none() {
                    target_row = i;
                }
            }

            self.board[target_row][col] = Some(player);
        }

        pub fn start(&mut self) {
            loop {
                self.display_board();

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
                self.place_token(input_col, self.players[self.current_turn].clone());

                // chance turn
                self.next_turn();
            }
        }
    }

    fn generate_board(row_count: usize, col_count: usize) -> Board {
        vec![vec![None; col_count]; row_count]
    }

    // mod board {
    //     use super::Player;
    //
    //     pub type BoardRow = Vec<BoardCell>;
    //     pub type BoardCell = Option<Player>;
    //
    //     pub struct Board {
    //         pub rows: BoardRow,
    //     }
    //
    //     impl Board {
    //         pub fn new(row_count: usize, col_count: usize) -> Board {
    //             vec![vec![None; col_count]; row_count]
    //         }
    //     }
    // }
}

fn main() {
    let players = vec![Player::new("Bob", 'x'), Player::new("Sarah", 'o')];

    let mut game = Game::new(5, 7, 4, players);

    game.start();
}

use super::Player;

pub type BoardRow = Vec<BoardCell>;
pub type BoardCell = Option<Player>;

pub struct Board {
    pub rows: Vec<BoardRow>,
}

impl Board {
    pub fn new(row_count: usize, col_count: usize) -> Self {
        Self {
            rows: vec![vec![None; col_count]; row_count],
        }
    }

    pub fn display(&self) -> String {
        let mut output = String::new();

        // Determine the width needed to display the largest column index
        let max_col_width = self.rows[0].len().to_string().len() + 1;

        // Print the column headers (indices)
        for col in 0..self.rows[0].len() {
            // Format each column index to be right-aligned within the max width
            output.push_str(&format!("{:width$} ", col, width = max_col_width));
        }

        output.push('\n');

        // Print the board rows
        for row in &self.rows {
            for col in row {
                let symbol = match col {
                    Some(player) => player.token,
                    None => ' ',
                };
                output.push_str(&format!("[{:width$}]", symbol, width = max_col_width - 1));
            }

            output.push('\n');
        }

        output
    }

    pub fn place_token(&mut self, col: usize, player: Player) {
        let mut target_row = 0;

        for (i, row) in self.rows.iter().enumerate() {
            if row[col].is_none() {
                target_row = i;
            }
        }

        self.rows[target_row][col] = Some(player);
    }

    pub fn get_diagonals_top_right_to_bottom_left(&self) -> Vec<Vec<BoardCell>> {
        let mut diagonals = Vec::new();
        let rows = self.rows.len();
        let cols = self.rows[0].len();

        for d in 0..(rows + cols - 1) {
            let mut diagonal = Vec::new();
            for i in 0..rows {
                let j = (cols as isize - 1) - (d as isize - i as isize);
                if j >= 0 && (j as usize) < cols {
                    diagonal.push(self.rows[i][j as usize].clone());
                }
            }
            if !diagonal.is_empty() {
                diagonals.push(diagonal);
            }
        }

        diagonals
    }

    pub fn get_diagonals_top_left_to_bottom_right(&self) -> Vec<Vec<BoardCell>> {
        let mut diagonals = Vec::new();
        let rows = self.rows.len();
        let cols = self.rows[0].len();

        for d in 0..(rows + cols - 1) {
            let mut diagonal = Vec::new();
            for i in 0..rows {
                let j = d as isize - i as isize;
                if j >= 0 && (j as usize) < cols {
                    diagonal.push(self.rows[i][j as usize].clone());
                }
            }
            if !diagonal.is_empty() {
                diagonals.push(diagonal);
            }
        }

        diagonals
    }

    pub fn valid_move(&self, col: usize) -> bool {
        if col >= self.rows[0].len() {
            return false;
        }

        // check if column is full
        self.rows.iter().any(|row| row[col].is_none())
    }

    pub fn is_board_full(&self) -> bool {
        for row in &self.rows {
            if row.iter().any(|cell| cell.is_none()) {
                return false;
            }
        }

        true
    }
}

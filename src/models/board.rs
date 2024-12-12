use rand::{Rng, SeedableRng};
use serde::Deserialize;

// set the char
const WALL: char = '⬜';
const EMPTY: char = '⬛';
const COOKIE: char = '🍪';
const MILK: char = '🥛';

// State of a cell in the grid
#[derive(Debug, Default, Clone, Copy, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BoardValue {
    Cookie,
    Milk,
    #[default]
    Empty,
}

impl BoardValue {
    fn to_char(&self) -> char {
        match self {
            BoardValue::Cookie => COOKIE,
            BoardValue::Milk => MILK,
            BoardValue::Empty => EMPTY,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum GameStatus {
    // true if winner, false is no winner
    GameOver(bool),
    InPlay,
}

pub struct Board {
    pub grid: [[BoardValue; 4]; 4],
    pub rng: rand::rngs::StdRng,
    pub winner: Option<BoardValue>,
}

impl Default for Board {
    fn default() -> Self {
        let grid = [[BoardValue::Empty; 4]; 4];
        let rng = rand::rngs::StdRng::seed_from_u64(2024);

        // building string from chars
        Board {
            grid,
            winner: None,
            rng,
        }
    }
}

impl Board {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate_random_board(&mut self) {
        let grid = &mut self.grid;
        self.winner = Some(BoardValue::Empty);

        // populating the grid
        for row in grid.iter_mut() {
            for val in row.iter_mut() {
                let res = self.rng.gen::<bool>();
                if res {
                    *val = BoardValue::Cookie
                } else {
                    *val = BoardValue::Milk
                }
            }
        }

        // check for winners
        // horizontal
        for y in 0..4 {
            if grid[y].iter().all(|&t| t == BoardValue::Cookie) {
                self.winner = Some(BoardValue::Cookie);
                return;
            } else if grid[y].iter().all(|&t| t == BoardValue::Milk) {
                self.winner = Some(BoardValue::Milk);
                return;
            }
        }

        // vertical
        for x in 0..4 {
            if (0..grid[0].len()).all(|y| grid[y][x] == BoardValue::Cookie) {
                self.winner = Some(BoardValue::Cookie);
                return;
            } else if (0..grid[0].len()).all(|y| grid[y][x] == BoardValue::Milk) {
                self.winner = Some(BoardValue::Milk);
                return;
            }
        }

        // tl -> br
        if (0..grid.len()).all(|i| grid[i][i] == BoardValue::Cookie) {
            self.winner = Some(BoardValue::Cookie);
            return;
        } else if (0..grid.len()).all(|i| grid[i][i] == BoardValue::Milk) {
            self.winner = Some(BoardValue::Milk);
            return;
        }

        // br -> tl
        if (0..grid.len()).all(|i| grid[grid.len() - i - 1][i] == BoardValue::Cookie) {
            self.winner = Some(BoardValue::Cookie);
        } else if (0..grid.len()).all(|i| grid[grid.len() - i - 1][i] == BoardValue::Milk) {
            self.winner = Some(BoardValue::Milk);
        }
    }

    // A function for building the board state from the characters
    fn build_state_from_grid(&self) -> String {
        let mut state = [[WALL; 6]; 5];
        // Replacing the middle of the 6x5 grid, with the 4x4 gird
        for row in 0..self.grid.len() {
            for col in 0..self.grid.len() {
                state[row][col + 1] = self.grid[row][col].to_char();
            }
        }

        state
            .into_iter()
            .map(|row| {
                // concert chars to string
                let mut res: String = row.into_iter().collect();
                res.push('\n');
                res
            })
            .collect()
    }

    pub fn get_current_state(&self) -> String {
        let mut state = self.build_state_from_grid();
        println!("Winner {:?}", self.winner);

        if let Some(winner) = self.winner {
            if winner != BoardValue::Empty {
                state.push_str(&format!("{} wins!\n", winner.to_char()));
            } else {
                state.push_str("No winner.\n");
            }
        }

        state
    }

    pub fn get_column(&self, idx: usize) -> Vec<BoardValue> {
        self.grid.iter().map(|row| row[idx]).collect()
    }
}

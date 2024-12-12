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
    fn to_string(self) -> char {
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
    pub state: String,
    pub winner: BoardValue,
    pub game_status: GameStatus,
}

impl Default for Board {
    fn default() -> Self {
        let grid = [[BoardValue::Empty; 4]; 4];

        // building string from chars
        Board {
            grid,
            state: String::new(),
            winner: BoardValue::Empty,
            game_status: GameStatus::InPlay,
        }
    }
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self::default();
        let state = board.build_state_from_grid();

        board.state = state;

        board
    }

    // A function to check if there is a winner or not
    // return true if winner, else returns false
    pub fn detect_winner(&mut self) -> GameStatus {
        // if there is no empty grid, and game is still in play
        // then there is no winner, return false
        if !self.check_if_grid_is_still_in_play() && self.game_status == GameStatus::InPlay {
            self.game_status = GameStatus::GameOver(false); // No winner
            self.game_status
        } else {
            // Check rows
            for row in &self.grid {
                let res = check_if_all_lines_are_the_same(row);
                if res.0 {
                    self.winner = res.1;
                    println!("Winner: {:?}", self.winner);
                    self.game_status = GameStatus::GameOver(true);
                    return self.game_status;
                }
            }

            // Check columns
            for x in 0..self.grid.len() {
                let column = self.get_column(x); // constructing the columns
                let res = check_if_all_lines_are_the_same(&column);
                if res.0 {
                    self.winner = res.1;
                    println!("Winner: {:?}", self.winner);
                    self.game_status = GameStatus::GameOver(true);
                    return self.game_status;
                }
            }

            // Check diagonals
            // Main diagonal
            let main_diag: Vec<BoardValue> =
                (0..self.grid.len()).map(|i| self.grid[i][i]).collect();
            let anti_diag: Vec<BoardValue> = (0..self.grid.len())
                .map(|i| self.grid[self.grid.len() - 1 - i][i])
                .collect();
            let res_diag = check_if_all_lines_are_the_same(&main_diag);
            if res_diag.0 {
                self.winner = res_diag.1;
                println!("Winner: {:?}", self.winner);
                self.game_status = GameStatus::GameOver(true);
                return self.game_status;
            }
            let res_anti_diag = check_if_all_lines_are_the_same(&anti_diag);
            if res_anti_diag.0 {
                self.winner = res_anti_diag.1;
                println!("Winner: {:?}", self.winner);
                self.game_status = GameStatus::GameOver(true);
                self.game_status
            } else {
                GameStatus::InPlay
            }
        }
    }

    pub fn check_if_grid_is_still_in_play(&self) -> bool {
        // If an empty element is found in the grid, it returns true
        // this means that the game is still in play
        let res = self.grid.iter().all(|r| r.contains(&BoardValue::Empty));

        res
    }

    pub fn place_item_in_column(&mut self, item: BoardValue, column: usize) {
        // Place the value into the board by floating it down to the nearest empty column
        println!("Item to add: {:?}", item);
        for i in (0..self.grid.len()).rev() {
            if self.grid[i][column] == BoardValue::Empty {
                self.grid[i][column] = item;
                break;
            }
        }

        println!("{}", self.get_current_state());
    }

    // A function for building the board state from the characters
    fn build_state_from_grid(&self) -> String {
        let mut state = [[WALL; 6]; 5];
        // Replacing the middle of the 6x5 grid, with the 4x4 gird
        for row in 0..self.grid.len() {
            for col in 0..self.grid.len() {
                state[row][col + 1] = self.grid[row][col].to_string();
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
        println!("Getting state with game status: {:?}", self.game_status);

        if self.winner != BoardValue::Empty {
            // set winner
            state.push_str(&format!("{} wins!", self.winner.to_string()));
        } else {
            state.push_str("No winner.");
        }

        state
    }

    pub fn get_column(&self, idx: usize) -> Vec<BoardValue> {
        self.grid.iter().map(|row| row[idx]).collect()
    }
}

pub fn check_if_all_lines_are_the_same(line: &[BoardValue]) -> (bool, BoardValue) {
    let val = line[0]; // get the first element
    let res = line.iter().all(|&v| {
        if val != BoardValue::Empty {
            v.to_string() == val.to_string()
        } else {
            false
        }
    }); // checks if all elements are same as the first
    (res, val)
}

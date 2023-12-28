use serde::Serialize;

#[derive(Serialize, Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Hidden,
    Flagged,
    Questioned,
    ShowMineExploded,
    ShowValue,
    ShowInvalidMine
}

#[derive(Serialize, Clone, Copy, PartialEq, Debug)]
pub enum GameState {
    BeforeGame,
    Playing,
    GameOver,
    GameWon
}

// The value here is the number of surrounding mines,
// but value '9' IS the mine
#[derive(Serialize, Clone, Copy)]
pub struct CellDetails {
    pub state: CellState,
    pub value: u8
}

pub struct Board {
    width: usize,
    height: usize,
    hidden_cell_count: usize,
    flag_count: usize,
    mines: usize,
    state: GameState,
    cells: Vec<Vec<CellDetails>>
}

impl Board {
    pub const fn empty() -> Board {
        Board {
            width: 0,
            height: 0,
            hidden_cell_count: 0,
            mines: 0,
            flag_count: 0,
            state: GameState::BeforeGame,
            cells: Vec::new()
        }
    }

    pub fn new(width: u32, height: u32, mines: u32) -> Board {
        if width < 1 || height < 1 {
            return Board::empty();
        }
        // Generate a 2-D vector with 'height' rows and 'width' cells per row
        // All cells are hidden and have value 0 at the start
        Board {
            width: width as usize,
            height: height as usize,
            hidden_cell_count: (width * height) as usize,
            flag_count: 0,
            mines: mines as usize,
            state: GameState::Playing,
            cells: (0..height).map(|_| 
                        (0..width).map(|_| 
                            CellDetails {
                                state: CellState::Hidden, 
                                value: 0
                            } )
                .collect::<Vec<CellDetails>>())
                .collect::<Vec<Vec<CellDetails>>>()
        }
    }

    // pub fn get_cells(&self) -> &Vec<Vec<CellDetails>> {
    //     &self.cells
    // }

    pub fn clone_cells(&self) -> Vec<Vec<CellDetails>> {
        self.cells.to_vec()
    }

    pub fn board_empty(&self) -> bool {
        return self.width == 0 || self.height == 0;
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_value(&self, x: usize, y: usize) -> Result<u8, String> {
        if self.board_empty() {
            Err(String::from("Board is empty"))
        } else if x >= self.width || y >= self.height {
            Err(String::from("Cell coordinate out of bounds"))
        } else {
            Ok(self.cells.get(y).unwrap().get(x).unwrap().value)
        }
    }

    pub fn get_state(&self, x: usize, y: usize) -> Result<CellState, String> {
        if self.board_empty() {
            Err(String::from("Board is empty"))
        } else if x >= self.width || y >= self.height {
            Err(String::from("Cell coordinate out of bounds"))
        } else {
            Ok(self.cells.get(y).unwrap().get(x).unwrap().state)
        }
    }

    pub fn set_value(&mut self, x: usize, y: usize, value: u8) -> Result<(), String> {
        if self.board_empty() {
            return Err(String::from("Board is empty"));
        }
        if x >= self.width || y >= self.height {
            Err(String::from("Cell coordinate out of bounds"))
        } else if value > 9 {
            Err(String::from("Invalid cell value, must be between 0 and 9 inclusive"))
        } else {
            self.cells.get_mut(y).unwrap().get_mut(x).unwrap().value = value;
            Ok(())
        }
    }

    pub fn set_state(&mut self, x: usize, y: usize, state: CellState) -> Result<(), String> {
        if self.board_empty() {
            return Err(String::from("Board is empty"));
        }
        if x >= self.width || y >= self.height {
            Err(String::from("Cell coordinate out of bounds"))
        } else {
            self.cells.get_mut(y).unwrap().get_mut(x).unwrap().state = state;
            Ok(())
        }
    }

    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    pub fn set_game_state(&mut self, new_state: GameState) {
        self.state = new_state
    }

    pub fn get_flag_count(&self) -> usize {
        self.flag_count
    }

    pub fn add_flag(&mut self) {
        self.flag_count += 1;
    }

    pub fn subtract_flag(&mut self) {
        self.flag_count -= 1;
    }

    pub fn get_hidden_cell_count(&self) -> usize {
        self.hidden_cell_count
    }

    pub fn add_hidden_cell(&mut self) {
        self.hidden_cell_count += 1;
    }

    pub fn subtract_hidden_cell(&mut self) {
        self.hidden_cell_count -= 1;
    }

    // pub fn set_mine_count(&mut self, mines: usize) {
    //     self.mines = mines
    // }

    // pub fn get_mine_total(&self) -> usize {
    //     self.mines
    // }

    pub fn get_remaining_mines(&self) -> isize {
        self.mines as isize - self.flag_count as isize
    }

    pub fn all_mines_found(&self) -> bool {
        return self.hidden_cell_count as isize == (self.mines as isize - self.flag_count as isize) && self.flag_count <= self.mines
    }
}
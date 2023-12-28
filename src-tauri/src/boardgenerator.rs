use crate::board::{CellDetails, Board};
use crate::CURRENT_BOARD;

use rand::seq::SliceRandom;
use rand::thread_rng;

#[tauri::command]
/// Generates a new board with the given dimensions, where the cell at
/// `(first_x, first_y)` is guaranteed to be empty. (value = 0)
/// The maximum number of mines a board can have is `width * height - 9` for
/// start-cells in the centre, `... - 6` for start-cells on the edge and
/// `... - 4` for start-cells in a corner. The mine count is capped at that
/// value. 
/// 
/// This function will not poke the start cell; all cells are Hidden after generation
pub fn generate_board(width: u32, height: u32, mines: u32, first_x: u32, first_y: u32) -> Vec<Vec<CellDetails>> {
    if width == 0 || height == 0 || first_x >= width || first_y >= height {
        println!("Invalid parameters to generate board :(");
        return Vec::new();
    }

    let mut board = Board::new(width, height, mines);
    
    // Add mines to the generated board
    add_mines(&mut board, mines, first_x, first_y);

    // Add values
    calculate_numbers(&mut board);

    // Create a copy of the board
    let board_copy = board.clone_cells();

    // Set the generated board as the current board
    if let Ok(mut current_board) = CURRENT_BOARD.lock() {
        *current_board = board;
        
        // Return the copy to the front-end
        return board_copy;
    }
    println!("ERROR: Couldn't lock current board variable :(");
    return Vec::new();
}

/// Sets `mines` number of cells to have a mine in it.
/// Ensures that there are no mines around the provided
/// cell at (`start_x`, `start_y`)
fn add_mines(board: &mut Board, mines: u32, start_x: u32, start_y: u32) {
    let mut all_possible_cells: Vec<(usize, usize)> = Vec::with_capacity(board.get_height() * board.get_width());
    for x in 0..board.get_width() {
        for y in 0..board.get_height() {
            if !borders_cell(x as i32, y as i32, start_x as i32, start_y as i32) {
                all_possible_cells.push((x, y));
            }
        }
    }

    let sample = all_possible_cells.choose_multiple(&mut thread_rng(), mines as usize);

    for (x, y) in sample {
        if let Err(err) = board.set_value(*x, *y, 9) {
            println!("ERROR (add_mines): {}", err);
        }
    }
}

/// Calculate the numbers of empty cells when the mines have been set
fn calculate_numbers(board: &mut Board) {
    for x in 0..board.get_width() {
        for y in 0..board.get_height() {
            // Don't overwrite the mines we just set
            if board.get_value(x, y) == Ok(0) {
                if let Err(e) = board.set_value(x, y, count_surrounding_mines(x, y, &board)) {
                    println!("ERROR (bg.cn): Could not set value of cell ({}, {}), error: {}", x, y, e);
                }
            }
        }
    }
}

fn count_surrounding_mines(x: usize, y: usize, board: &Board) -> u8 {
    let mut mine_count = 0u8;
    let x_int = x as i32;
    let y_int = y as i32;
    for x_offset in [-1, 0, 1] {
        for y_offset in [-1, 0, 1] {
            // Don't check the cell itself
            if x_offset == 0 && y_offset == 0 { continue; }

            let x_check = x_int + x_offset;
            let y_check = y_int + y_offset;

            // We cannot turn -1 into a usize, so continue
            if x_check < 0 || y_check < 0 { continue; }
            // This is likely to return an Err for some cells, but we just ignore it as there will be no mine there
            if let Ok(v) = board.get_value(x_check as usize, y_check as usize) {
                if v == 9 {
                    mine_count += 1;
                }
            }
        }
    }
    return mine_count;
}

/// Returns `true` if cell `(x1, y1) == (x2, y2)`, or
/// is one of the 8 cells surrounding `(x2, y2)`
fn borders_cell(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    return (x2 - x1).abs() <= 1 && (y2 - y1).abs() <= 1;
}
use serde::Serialize;

use crate::{board::{CellState, Board, GameState}, CURRENT_BOARD};
use crate::settings;

#[derive(Serialize)]
pub struct BoardChange {
    x: usize,
    y: usize,
    state: Option<CellState>,
}

#[derive(Serialize)]
pub struct GameChange {
    flag_count: usize,
    hidden_cell_count: usize,
    mines_remaining: isize,
    game_state: GameState,
    changes: Vec<BoardChange>
}

#[tauri::command]
pub fn poke(x: usize, y: usize) -> GameChange {
    let mut changes: Vec<BoardChange> = Vec::new();
    if let Ok(mut current_board) = CURRENT_BOARD.lock() {
        match current_board.get_state(x, y) {
            Ok(CellState::Hidden) => poke_hidden_cell(x, y, &mut current_board, &mut changes),
            Ok(CellState::ShowValue) => poke_shown_cell(x, y, &mut current_board, &mut changes),
            // Don't allow the user to poke a flagged (or question-marked) cell
            Ok(_) => { },
            Err(e) => { println!("ERROR (bl.p): Couldn't poke cell ({}, {}), error: {}", x, y, e); }
        }

        // Don't allow the game to be won by marking too many cells as flags
        if current_board.get_game_state() == GameState::Playing && current_board.all_mines_found() {
            current_board.set_game_state(GameState::GameWon);
            flag_remaining_mines(&mut current_board, &mut changes);
        }

        GameChange {
            flag_count: current_board.get_flag_count(),
            hidden_cell_count: current_board.get_hidden_cell_count(),
            mines_remaining: current_board.get_remaining_mines(),
            game_state: current_board.get_game_state(),
            changes
        }

    } else {
        println!("ERROR (bl.poke): Couldn't lock current board variable. No changes to the board could be made");
        GameChange {
            flag_count: 0,
            hidden_cell_count: 0,
            mines_remaining: 0,
            game_state: GameState::GameOver,
            changes: Vec::new()
        }
    }
}

#[tauri::command]
pub fn mark(x: usize, y: usize) -> GameChange {
    if let Ok(mut current_board) = CURRENT_BOARD.lock() {
        let mut changes = Vec::new();

        match (*current_board).get_state(x, y) {
            // From Hidden, we always go to Flagged
            Ok(CellState::Hidden) => { add_state_change(x, y, CellState::Flagged, &mut changes, &mut current_board); },
            // From Flagged we go to Questioned if it's enabled, or back to Hidden if it's not
            Ok(CellState::Flagged) => {
                let next_state = if settings::is_questions_enabled() {
                    CellState::Questioned
                } else {
                    CellState::Hidden
                };
                add_state_change(x, y, next_state, &mut changes, &mut current_board);
            }
            // From Questioned we always go back to Hidden
            Ok(CellState::Questioned) => { add_state_change(x, y, CellState::Hidden, &mut changes, &mut current_board); },
            // If we're at any other CellState, don't do anything
            Ok(_) => {},
            Err(e) => { println!("ERROR (bl.mark): Could not get the state of cell ({}, {}); error: {}", x, y, e) },
        }

        GameChange {
            flag_count: current_board.get_flag_count(),
            hidden_cell_count: current_board.get_hidden_cell_count(),
            mines_remaining: current_board.get_remaining_mines(),
            game_state: current_board.get_game_state(),
            changes
        }
    } else {
        println!("ERROR (bl.mark): Couldn't lock current board variable. No changes to the board could be made");
        GameChange {
            flag_count: 0,
            hidden_cell_count: 0,
            mines_remaining: 0,
            game_state: GameState::GameOver,
            changes: Vec::new()
        }
    }
}

fn add_state_change(x: usize, y: usize, state: CellState, changes: &mut Vec<BoardChange>, board: &mut Board) {
    if let Ok(old_state) = board.get_state(x, y) {
        if let Ok(()) = board.set_state(x, y, state) {
            changes.push(BoardChange { x, y, state: Some(state) });
            if old_state == CellState::Hidden && state == CellState::ShowValue {
                board.subtract_hidden_cell();
            } else if old_state != CellState::Flagged && state == CellState::Flagged {
                board.add_flag();
                board.subtract_hidden_cell();
            } else if old_state == CellState::Flagged && state != CellState::Flagged {
                board.subtract_flag();
                board.add_hidden_cell();
            }
        } else {
            println!("ERROR (bl.asc): Couldn't set cell ({}, {}) to state {:?}", x, y, state);
        }
    }
}

fn poke_hidden_cell(x: usize, y: usize, board: &mut Board, changes: &mut Vec<BoardChange>) {
    poke_single_cell(x, y, board, changes);
}

fn poke_shown_cell(x: usize, y: usize, board: &mut Board, changes: &mut Vec<BoardChange>) {
    if settings::is_quick_uncover_enabled() {
        if settings::is_protected_uncover_enabled() {
            let flags = count_flags(x, y, &board);
            if let Ok(v) = board.get_value(x, y) {
                if v == flags {
                    poke_around_cell(x, y, board, changes);
                }
            }
        } else {
            poke_around_cell(x, y, board, changes);
        }
    }
}

fn show_all_mines(board: &mut Board, changes: &mut Vec<BoardChange>) {
    for x in 0..board.get_width() {
        for y in 0..board.get_height() {
            if let Ok(state) = board.get_state(x, y) {
                // Unveil all mines that are still hidden or question-marked
                if state == CellState::Hidden || state == CellState::Questioned {
                    if let Ok(v) = board.get_value(x, y) {
                        if v == 9 {
                            add_state_change(x, y, CellState::ShowValue, changes, board);
                        }
                    }
                } else if state == CellState::Flagged {
                    if let Ok(v) = board.get_value(x, y) {
                        if v != 9 {
                            add_state_change(x, y, CellState::ShowInvalidMine, changes, board);
                        }
                    }
                } 
            }
        }
    }
}

fn poke_around_cell(x: usize, y: usize, board: &mut Board, changes: &mut Vec<BoardChange>) {
    let x_int = x as i32;
    let y_int = y as i32;
    for x_offset in [-1, 0, 1] {
        for y_offset in [-1, 0, 1] {
            // Don't poke the cell itself
            if x_offset == 0 && y_offset == 0 { continue; }

            let x_poke = x_int + x_offset;
            let y_poke = y_int + y_offset;

            // We cannot turn -1 into a usize, so continue
            if x_poke < 0 || y_poke < 0 { continue; }
            // This is likely to return an Err for some cells, but we just ignore it as we don't need to poke there
            poke_single_cell(x_poke as usize, y_poke as usize, board, changes)
        }
    }
}

fn poke_single_cell(x: usize, y: usize, board: &mut Board, changes: &mut Vec<BoardChange>) {
    if let Ok(state) = board.get_state(x, y) {
        if state == CellState::Hidden {
            match board.get_value(x, y) {
                Ok(9) => {
                    add_state_change(x, y, CellState::ShowMineExploded, changes, board);
                    board.set_game_state(GameState::GameOver);
                    show_all_mines(board, changes);
                },
                Ok(0) => {
                    add_state_change(x, y, CellState::ShowValue, changes, board);
                    poke_around_cell(x, y, board, changes);
                },
                Ok(_) => add_state_change(x, y, CellState::ShowValue, changes, board),
                Err(e) => println!("ERROR (bl.psc): Couldn't poke cell ({}, {}), error: {}", x, y, e)
            }
        }
    }
}

fn count_flags(x: usize, y: usize, board: &Board) -> u8 {
    let mut flag_count = 0u8;
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
            if let Ok(state) = board.get_state(x_check as usize, y_check as usize) {
                if state == CellState::Flagged {
                    flag_count += 1;
                }
            }
        }
    }
    return flag_count;
}

fn flag_remaining_mines(board: &mut Board, changes: &mut Vec<BoardChange>) {
    for x in 0..board.get_width() {
        for y in 0..board.get_height() {
            if let Ok(state) = board.get_state(x, y) {
                if state == CellState::Hidden {
                    if let Ok(v) = board.get_value(x, y) {
                        if v == 9 {
                            add_state_change(x, y, CellState::Flagged, changes, board);
                        }
                    }
                }
            }
        }
    }
}


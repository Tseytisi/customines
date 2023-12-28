// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board;
mod boardgenerator;
mod boardlogic;
mod settings;

use std::sync::Mutex;
use board::Board;
use settings::Settings;

static CURRENT_BOARD: Mutex<Board> = Mutex::new(Board::empty());
static GAME_SETTINGS: Mutex<Settings> = Mutex::new(Settings::new());

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![boardgenerator::generate_board, 
            boardlogic::poke, boardlogic::mark, settings::set_game_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

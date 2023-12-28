use serde::{Serialize, Deserialize};

use crate::GAME_SETTINGS;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    /// Enable marking cells with a question mark. If disabled,
    /// cells will go back to 'unmarked' after clicking an already-flagged
    /// cell again
    questions_enabled: bool,

    /// If true, the user can click an already uncovered cell with a number
    /// to uncover all surrounding cells
    quick_uncover: bool,
    
    /// If true, won't uncover surrounding cells if the click isn't safe
    protected_quick_uncover: bool
}

impl Settings {
    pub const fn new() -> Settings {
        Settings {
            questions_enabled: true,
            quick_uncover: true,
            protected_quick_uncover: true,
        }
    }
}

#[tauri::command]
pub fn set_game_settings(settings: Settings) {
    if let Ok(mut old_settings) = GAME_SETTINGS.lock() {
        *old_settings = settings;
    }
}

pub fn is_questions_enabled() -> bool {
    if let Ok(settings) = GAME_SETTINGS.lock() {
        return settings.questions_enabled;
    } else {
        println!("ERROR (s.iqe): Couldn't lock the global game settings object");
        return true;
    }
}

pub fn is_quick_uncover_enabled() -> bool {
    if let Ok(settings) = GAME_SETTINGS.lock() {
        return settings.quick_uncover;
    } else {
        println!("ERROR (s.ique): Couldn't lock the global game settings object");
        return true;
    }
}

pub fn is_protected_uncover_enabled() -> bool {
    if let Ok(settings) = GAME_SETTINGS.lock() {
        return settings.protected_quick_uncover;
    } else {
        println!("ERROR (s.ipue): Couldn't lock the global game settings object");
        return true;
    }
}
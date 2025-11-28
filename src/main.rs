mod game_objects;
mod game_logic;
mod rendering;
mod controller;
mod ui;

use game_logic::GameManager;
use std::io;

fn main() -> Result<(), io::Error> {
    let terminal = ratatui::init();
    let game_manager = GameManager::new();
    let app_result = game_manager.run(terminal);
    ratatui::restore();

    app_result
}

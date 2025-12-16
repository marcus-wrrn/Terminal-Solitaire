mod game_objects;
mod game_logic;
mod rendering;
mod controller;
mod resources;
mod ui;

use game_logic::GameManager;
use ratatui::crossterm::{
    execute,
    event::{EnableMouseCapture, DisableMouseCapture},
};
use std::io;

fn main() -> Result<(), io::Error> {
    let terminal = ratatui::init();
    execute!(std::io::stdout(), EnableMouseCapture)?;

    let game_manager = GameManager::new();
    let app_result = game_manager.run(terminal);

    execute!(std::io::stdout(), DisableMouseCapture)?;
    ratatui::restore();

    app_result
}

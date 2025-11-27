use crate::game_objects::{Board, Deck};
use crate::rendering::BoardRenderer;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    DefaultTerminal, Frame,
};
use std::io;
use std::time::Duration;

pub struct GameState {
    board: Board,
}

impl GameState {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();

        let mut board = Board::new();
        board.setup(&mut deck);

        Self {
            board,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    // pub fn board_mut(&mut self) -> &mut Board {
    //     &mut self.board
    // }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GameManager {
    game_state: GameState,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            game_state: GameState::new(),
        }
    }

    // pub fn game_state(&self) -> &GameState {
    //     &self.game_state
    // }

    // pub fn game_state_mut(&mut self) -> &mut GameState {
    //     &mut self.game_state
    // }

    pub fn run(self, mut terminal: DefaultTerminal) -> Result<(), io::Error> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        let board_renderer = BoardRenderer::new(self.game_state.board());
        frame.render_widget(board_renderer, frame.area());
    }
}

impl Default for GameManager {
    fn default() -> Self {
        Self::new()
    }
}

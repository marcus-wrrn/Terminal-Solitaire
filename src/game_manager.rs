use crate::game_objects::{Board, Deck};
use crate::rendering::BoardRenderer;
use crate::controller::{Controller, GameAction};
use ratatui::{DefaultTerminal, Frame};
use std::io;

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
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GameManager {
    game_state: GameState,
    controller: Controller,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            game_state: GameState::new(),
            controller: Controller::new(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), io::Error> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Some(action) = self.controller.poll_action()? {
                match action {
                    GameAction::Quit => break,
                    GameAction::DrawStock => {
                        // TODO: Implement stock draw logic
                    }
                    GameAction::Undo => {
                        // TODO: Implement undo logic
                    }
                    GameAction::Restart => {
                        // TODO: Implement restart logic
                    }
                    GameAction::Help => {
                        // TODO: Implement help display
                    }
                    _ => {
                        // Handle other actions
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

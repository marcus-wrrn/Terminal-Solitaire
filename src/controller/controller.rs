use super::KeyBindings;
use ratatui::crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode};
use std::io;

/// Represents the different areas of focus on the game board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusArea {
    Tableau,
    Foundation,
    Stock,
    Waste,
}

/// Represents actions the player can take in the game
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameAction {
    /// Quit the game
    Quit,

    /// Select the current card/pile
    Select,

    /// Confirm/execute the current action
    Enter,

    /// Cancel the current selection
    Cancel,

    /// Move selection left
    MoveLeft,

    /// Move selection right
    MoveRight,

    /// Move selection up
    MoveUp,

    /// Move selection down
    MoveDown,

    /// Change focus to tableau piles
    FocusTableau,

    /// Change focus to foundation piles
    FocusFoundation,

    /// Change focus to stock pile
    FocusStock,

    /// Change focus to waste pile
    FocusWaste,

    /// Draw a card from the stock
    DrawStock,

    /// Undo the last move
    Undo,

    /// Restart the game
    Restart,

    /// Show help menu
    Help,

    /// No action (for unbound keys)
    None,
}

/// Controller for handling user input and key bindings
pub struct Controller {
    key_bindings: KeyBindings,
    current_focus: FocusArea,
}

impl Controller {
    /// Create a new Controller with default key bindings
    pub fn new() -> Self {
        Self {
            key_bindings: KeyBindings::default(),
            current_focus: FocusArea::Tableau,
        }
    }

    /// Create a new Controller with custom key bindings
    pub fn with_key_bindings(key_bindings: KeyBindings) -> Self {
        Self {
            key_bindings,
            current_focus: FocusArea::Tableau,
        }
    }

    pub fn key_bindings(&self) -> &KeyBindings {
        &self.key_bindings
    }

    pub fn key_bindings_mut(&mut self) -> &mut KeyBindings {
        &mut self.key_bindings
    }

    pub fn current_focus(&self) -> FocusArea {
        self.current_focus
    }

    /// Set the current focus area
    pub fn set_focus(&mut self, focus: FocusArea) {
        self.current_focus = focus;
    }

    pub fn poll_action(&mut self) -> io::Result<Option<GameAction>> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let action = self.map_key_to_action(key);
                    match action {
                        GameAction::FocusTableau => self.current_focus = FocusArea::Tableau,
                        GameAction::FocusFoundation => self.current_focus = FocusArea::Foundation,
                        GameAction::FocusStock => self.current_focus = FocusArea::Stock,
                        GameAction::FocusWaste => self.current_focus = FocusArea::Waste,
                        _ => {}
                    }

                    return Ok(Some(action));
                }
            }
        }
        Ok(None)
    }

    /// This method will block until a key event is received.
    pub fn read_action(&mut self) -> io::Result<GameAction> {
        loop {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let action = self.map_key_to_action(key);
                    match action {
                        GameAction::FocusTableau => self.current_focus = FocusArea::Tableau,
                        GameAction::FocusFoundation => self.current_focus = FocusArea::Foundation,
                        GameAction::FocusStock => self.current_focus = FocusArea::Stock,
                        GameAction::FocusWaste => self.current_focus = FocusArea::Waste,
                        _ => {}
                    }

                    return Ok(action);
                }
            }
        }
    }

    fn map_key_to_action(&self, key: KeyEvent) -> GameAction {
        let code = key.code;
        let bindings = &self.key_bindings;

        if code == bindings.quit {
            GameAction::Quit
        } else if code == bindings.select {
            GameAction::Select
        } else if code == bindings.enter {
            GameAction::Enter
        } else if code == bindings.cancel {
            GameAction::Cancel
        } else if code == bindings.move_left {
            GameAction::MoveLeft
        } else if code == bindings.move_right {
            GameAction::MoveRight
        } else if code == bindings.move_up {
            GameAction::MoveUp
        } else if code == bindings.move_down {
            GameAction::MoveDown
        } else if code == bindings.focus_tableau {
            GameAction::FocusTableau
        } else if code == bindings.focus_foundation {
            GameAction::FocusFoundation
        } else if code == bindings.focus_stock {
            GameAction::FocusStock
        } else if code == bindings.focus_waste {
            GameAction::FocusWaste
        } else if code == bindings.draw_stock {
            GameAction::DrawStock
        } else if code == bindings.undo {
            GameAction::Undo
        } else if code == bindings.restart {
            GameAction::Restart
        } else if code == bindings.help {
            GameAction::Help
        } else {
            GameAction::None
        }
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controller_creation() {
        let controller = Controller::new();
        assert_eq!(controller.current_focus(), FocusArea::Tableau);
    }

    #[test]
    fn test_focus_management() {
        let mut controller = Controller::new();
        controller.set_focus(FocusArea::Foundation);
        assert_eq!(controller.current_focus(), FocusArea::Foundation);
    }

    // #[test]
    // fn test_custom_key_bindings() {
    //     let custom_bindings = KeyBindings::default()
    //         .with_quit(KeyCode::Char('x'))
    //         .with_select(KeyCode::Char('s'));

    //     let controller = Controller::with_key_bindings(custom_bindings);
    //     assert_eq!(controller.key_bindings().quit, KeyCode::Char('x'));
    //     assert_eq!(controller.key_bindings().select, KeyCode::Char('s'));
    // }
}

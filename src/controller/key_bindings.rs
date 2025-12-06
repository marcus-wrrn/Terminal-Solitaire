pub use ratatui::crossterm::event::KeyCode;

/// Configuration for rebindable key bindings in the game
#[derive(Debug, Clone)]
pub struct KeyBindings {
    /// Key to quit the game
    pub quit: KeyCode,

    /// Key to select a card or pile
    pub select: KeyCode,

    /// Key to confirm an action (e.g., move a card)
    pub enter: KeyCode,

    /// Key to cancel/deselect
    pub cancel: KeyCode,

    /// Key to move selection left
    pub move_left: KeyCode,

    /// Key to move selection right
    pub move_right: KeyCode,

    /// Key to move selection up
    pub move_up: KeyCode,

    /// Key to move selection down
    pub move_down: KeyCode,

    /// Key to focus on tableau piles
    pub focus_tableau: KeyCode,

    /// Key to focus on foundation piles
    pub focus_foundation: KeyCode,

    /// Key to focus on stock pile
    pub focus_stock: KeyCode,

    /// Key to focus on waste pile
    pub focus_waste: KeyCode,

    /// Key to draw from stock
    pub draw_stock: KeyCode,

    /// Key to undo last move
    pub undo: KeyCode,

    /// Key to restart game
    pub restart: KeyCode,

    /// Key to show help
    pub options_menu: KeyCode,

    pub open_menu: KeyCode
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: KeyCode::Char('q'),
            select: KeyCode::Char(' '),
            enter: KeyCode::Enter,
            cancel: KeyCode::Char('x'),
            move_left: KeyCode::Left,
            move_right: KeyCode::Right,
            move_up: KeyCode::Up,
            move_down: KeyCode::Down,
            focus_tableau: KeyCode::Char('t'),
            focus_foundation: KeyCode::Char('f'),
            focus_stock: KeyCode::Char('s'),
            focus_waste: KeyCode::Char('w'),
            draw_stock: KeyCode::Char('d'),
            undo: KeyCode::Char('u'),
            restart: KeyCode::Char('r'),
            options_menu: KeyCode::Esc,
            open_menu: KeyCode::Char('m'),
        }
    }
}

// impl KeyBindings {
//     // /// Builder method to set the quit key
//     // pub fn with_quit(mut self, key: KeyCode) -> Self {
//     //     self.quit = key;
//     //     self
//     // }

//     // /// Builder method to set the select key
//     // pub fn with_select(mut self, key: KeyCode) -> Self {
//     //     self.select = key;
//     //     self
//     // }

//     // /// Builder method to set the enter key
//     // pub fn with_enter(mut self, key: KeyCode) -> Self {
//     //     self.enter = key;
//     //     self
//     // }

//     // /// Builder method to set the cancel key
//     // pub fn with_cancel(mut self, key: KeyCode) -> Self {
//     //     self.cancel = key;
//     //     self
//     // }
// }

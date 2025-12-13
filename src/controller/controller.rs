use super::KeyBindings;
use ratatui::crossterm::event::{self, Event, KeyEvent, KeyEventKind, MouseButton, MouseEventKind};
use std::io;
use std::rc::Rc;
use std::cell::RefCell;

/// Represents the input mode for the controller
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlMode {
    Keyboard,
    Mouse,
}

/// Tracks the current drag operation state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DragState {
    /// Starting position of the drag
    pub start_x: u16,
    pub start_y: u16,
    /// Current mouse position
    pub current_x: u16,
    pub current_y: u16,
    pub is_active: bool,
}

const DRAG_THRESHOLD: u16 = 2;

/// Represents actions the player can take in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameAction {
    /// Quit the game
    Quit,

    /// Select the current card/pile
    Select,

    /// Confirm/execute the current action
    Enter,

    OpenMenu,

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
    OptionsMenu,

    /// Start dragging cards from a position
    StartDrag(u16, u16),

    /// Update drag position (mouse moved while dragging)
    UpdateDrag(u16, u16),

    /// Complete drag operation (mouse released)
    CompleteDrag(u16, u16),

    /// Cancel drag operation
    CancelDrag,

    LeftMousePress(u16, u16),

    Click,
    /// No action (for unbound keys)
    None,
}

/// Controller for handling user input and key bindings
pub struct Controller {
    key_bindings: Rc<RefCell<KeyBindings>>,
    drag_state: Option<DragState>,
    control_mode: ControlMode,
}

impl Controller {
    pub fn new() -> Self {

        Self {
            key_bindings: Rc::new(RefCell::new(KeyBindings::default())),
            drag_state: None,
            control_mode: ControlMode::Mouse,
        }
    }

    pub fn keybindings(&self) -> Rc<RefCell<KeyBindings>> {
        self.key_bindings.clone()
    }

    fn is_drag_movement(&self, x: u16, y: u16) -> bool {
        if let Some(drag) = &self.drag_state {
            let dx = (x as i32 - drag.start_x as i32).abs();
            let dy = (y as i32 - drag.start_y as i32).abs();
            (dx as u16) > DRAG_THRESHOLD || (dy as u16) > DRAG_THRESHOLD
        } else {
            false
        }
    }

    pub fn poll_action(&mut self) -> io::Result<Option<GameAction>> {
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        let action = self.map_key_to_action(key);
                        self.handle_key_action(action.clone());
                        return Ok(Some(action));
                    }
                }
                Event::Mouse(mouse) => {
                    let action = self.handle_mouse_action(mouse);
                    return Ok(action);
                }
                _ => {}
            }
        }
        Ok(None)
    }

    fn handle_key_action(&mut self, action: GameAction) {
        match action {
            GameAction::MoveLeft | GameAction::MoveRight |
            GameAction::MoveUp | GameAction::MoveDown => {
                self.control_mode = ControlMode::Keyboard;
            }
            GameAction::Cancel => {
                if self.drag_state.is_some() {
                    self.drag_state = None;
                }
            }
            _ => {}
        }
    }

    pub fn is_keyboard_mode(&self) -> bool {
        self.control_mode == ControlMode::Keyboard
    }

    fn handle_mouse_action(&mut self, mouse: event::MouseEvent) -> Option<GameAction> {
        self.control_mode = ControlMode::Mouse;

        match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                self.drag_state = Some(DragState {
                    start_x: mouse.column,
                    start_y: mouse.row,
                    current_x: mouse.column,
                    current_y: mouse.row,
                    is_active: false,
                });
                Some(GameAction::LeftMousePress(mouse.column, mouse.row))
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if let Some(drag) = self.drag_state {
                    let is_drag = self.is_drag_movement(mouse.column, mouse.row);

                    self.drag_state = Some(DragState {
                        start_x: drag.start_x,
                        start_y: drag.start_y,
                        current_x: mouse.column,
                        current_y: mouse.row,
                        is_active: drag.is_active || is_drag,
                    });

                    if !drag.is_active && is_drag {
                        Some(GameAction::StartDrag(drag.start_x, drag.start_y))
                    } else if drag.is_active {
                        Some(GameAction::UpdateDrag(mouse.column, mouse.row))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            MouseEventKind::Up(MouseButton::Left) => {
                if let Some(drag) = self.drag_state {
                    self.drag_state = None;
                    if drag.is_active {
                        Some(GameAction::CompleteDrag(mouse.column, mouse.row))
                    } else {
                        Some(GameAction::Click)
                    }
                } else {
                    None
                }
            }
            MouseEventKind::Down(MouseButton::Right) => {
                if self.drag_state.is_some() {
                    self.drag_state = None;
                    Some(GameAction::CancelDrag)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn map_key_to_action(&self, key: KeyEvent) -> GameAction {
        let code = key.code;
        let bindings = self.key_bindings.borrow();

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
        } else if code == bindings.options_menu {
            GameAction::OptionsMenu
        } else if code == bindings.open_menu {
            GameAction::OpenMenu
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

use crate::controller::GameAction;
use crate::game_logic::balaterm::BalatermState;
use crate::game_logic::game_handler::{AppTransition, GameHandler};
use crate::game_logic::MenuManager;
use crate::rendering::GameRenderer;
use crate::ui::DebugLog;
use ratatui::{buffer::Buffer, layout::Rect};

pub struct BalatermGame {
    game_state: BalatermState,
}

impl BalatermGame {
    pub fn new() -> Self {
        Self {
            game_state: BalatermState::new(),
        }
    }
}

impl GameHandler for BalatermGame {
    fn handle_action(&mut self, action: GameAction, _debug_log: &mut DebugLog) -> Option<AppTransition> {
        match action {
            GameAction::Quit => Some(AppTransition::Quit),
            GameAction::OptionsMenu => Some(AppTransition::OpenSettings),
            GameAction::HelpMenu => Some(AppTransition::ShowHelp),
            _ => None,
        }
    }

    fn handle_mouse_action(&mut self, _action: GameAction, _renderer: &GameRenderer, _debug_log: &mut DebugLog) {}

    fn update(&mut self, _menu_manager: &mut MenuManager, _debug_log: &mut DebugLog) {}

    fn draw(&self, _area: Rect, _buf: &mut Buffer, _renderer: &mut GameRenderer, _debug_log: &DebugLog) {}

    fn restart(&self) -> Box<dyn GameHandler> {
        Box::new(BalatermGame::new())
    }

    fn set_keyboard_mode(&mut self, _enabled: bool) {}
}

use crate::controller::GameAction;
use crate::game_logic::MenuManager;
use crate::rendering::GameRenderer;
use crate::ui::DebugLog;
use ratatui::{buffer::Buffer, layout::Rect};

pub enum AppTransition {
    Quit,
    Restart,
    OpenSettings,
    ShowHelp,
}

pub trait GameHandler {
    fn handle_action(
        &mut self,
        action: GameAction,
        debug_log: &mut DebugLog,
    ) -> Option<AppTransition>;
    fn handle_mouse_action(
        &mut self,
        action: GameAction,
        renderer: &GameRenderer,
        debug_log: &mut DebugLog,
    );
    fn update(&mut self, menu_manager: &mut MenuManager, debug_log: &mut DebugLog);
    fn draw(&self, area: Rect, buf: &mut Buffer, renderer: &mut GameRenderer, debug_log: &DebugLog);
    fn restart(&self) -> Box<dyn GameHandler>;
    fn set_keyboard_mode(&mut self, enabled: bool);
}

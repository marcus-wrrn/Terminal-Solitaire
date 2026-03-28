use crate::controller::GameAction;
use crate::game_logic::balaterm::BalatermState;
use crate::game_logic::game_handler::{AppTransition, GameHandler};
use crate::game_logic::MenuManager;
use crate::rendering::balaterm::{BalatermGameRenderer, BalatermRenderingInstructions};
use crate::rendering::GameRenderer;
use crate::ui::DebugLog;
use ratatui::{buffer::Buffer, layout::Rect};

pub struct BalatermGame {
    game_state: BalatermState,
    game_renderer: BalatermGameRenderer,
}

impl BalatermGame {
    pub fn new() -> Self {
        Self {
            game_state: BalatermState::new(),
            game_renderer: BalatermGameRenderer::new(),
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

    fn draw(&self, area: Rect, buf: &mut Buffer, _renderer: &mut GameRenderer, debug_log: &DebugLog) {
        let instr = BalatermRenderingInstructions {
            title: "Balaterm",
            hands: &self.game_state.hands,
            selected_hand: None,
            selected_slot: None,
            debug_log,
        };
        self.game_renderer.render(area, buf, &instr);
    }

    fn restart(&self) -> Box<dyn GameHandler> {
        Box::new(BalatermGame::new())
    }

    fn set_keyboard_mode(&mut self, _enabled: bool) {}
}

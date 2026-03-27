use crate::controller::GameAction;
use crate::game_logic::balaterm::BalatermState;
use crate::game_logic::game_handler::{AppTransition, GameHandler};
use crate::game_logic::MenuManager;
use crate::rendering::balaterm::HandRenderer;
use crate::rendering::GameRenderer;
use crate::ui::DebugLog;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
};

pub struct BalatermGame {
    game_state: BalatermState,
    hand_renderer: HandRenderer,
}

impl BalatermGame {
    pub fn new() -> Self {
        Self {
            game_state: BalatermState::new(),
            hand_renderer: HandRenderer::new(),
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

    fn draw(&self, area: Rect, buf: &mut Buffer, _renderer: &mut GameRenderer, _debug_log: &DebugLog) {
        let sections = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(crate::rendering::card_renderer::CardRenderer::HEIGHT),
        ])
        .flex(Flex::Center)
        .split(area);

        let hand_area = sections[1];

        for (i, hand) in self.game_state.hands.iter().enumerate() {
            let y_offset = i as u16 * (crate::rendering::card_renderer::CardRenderer::HEIGHT + 1);
            let row_area = Rect {
                y: hand_area.y + y_offset,
                ..hand_area
            };
            self.hand_renderer.render(hand, None, buf, row_area);
        }
    }

    fn restart(&self) -> Box<dyn GameHandler> {
        Box::new(BalatermGame::new())
    }

    fn set_keyboard_mode(&mut self, _enabled: bool) {}
}

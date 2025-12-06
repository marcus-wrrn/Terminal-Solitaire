use crate::ui::{OptionsMenu, MenuOption, popups::VictoryScreen};
use crate::game_logic::GameState;
use crate::controller::GameAction;
use ratatui::{buffer::Buffer, layout::Rect};

pub struct MenuManager {
    options_menu: OptionsMenu,
    victory_screen: VictoryScreen,
}

impl MenuManager {
    pub fn new() -> Self {
        Self {
            options_menu: OptionsMenu::new(),
            victory_screen: VictoryScreen::new()
        }
    }

    pub fn is_menu_active(&self) -> bool {
        self.options_menu.is_visible()
    }

    pub fn toggle_options_menu(&mut self) {
        self.options_menu.toggle();
    }

    pub fn handle_menu_action(&mut self, action: GameAction) -> Option<MenuAction> {
        if !self.options_menu.is_visible() {
            return None;
        }

        let menu_action = match action {
            GameAction::Quit | GameAction::Cancel | GameAction::OptionsMenu => {
                self.options_menu.hide();
                Some(MenuAction::CloseMenu)
            }
            GameAction::MoveUp => {
                self.options_menu.move_up();
                Some(MenuAction::Navigate)
            }
            GameAction::MoveDown => {
                self.options_menu.move_down();
                Some(MenuAction::Navigate)
            }
            GameAction::Select | GameAction::Enter => {
                let selected = self.options_menu.selected_option();
                self.options_menu.hide();
                Some(MenuAction::OptionSelected(selected))
            }
            _ => None,
        };

        return menu_action;
    }

    pub fn handle_game_state(&mut self, game_state: &GameState) {
        if game_state.has_won() {
            self.victory_screen.show();
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if self.victory_screen.is_visible() {
            self.victory_screen.render(area, buf);
        }

        // Options menu always displayed last
        if self.options_menu.is_visible() {
            self.options_menu.render(area, buf);
        }

    }
}

impl Default for MenuManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuAction {
    CloseMenu,
    Navigate,
    OptionSelected(MenuOption),
}

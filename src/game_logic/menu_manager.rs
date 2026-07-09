use crate::ui::{OptionsMenu, MenuOption, popups::{VictoryPop, StartupPop}};
use crate::controller::{GameAction, KeyBindings};
use ratatui::{buffer::Buffer, layout::Rect};
use std::rc::Rc;
use std::cell::RefCell;

pub struct MenuManager {
    options_menu: OptionsMenu,
    victory_screen: VictoryPop,
    startup_screen: StartupPop,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuAction {
    CloseMenu,
    Navigate,
    OptionSelected(MenuOption),
}


impl MenuManager {
    pub fn new(key_bindings: Rc<RefCell<KeyBindings>>) -> Self {
        let startup_screen = StartupPop::new(key_bindings);
        Self {
            options_menu: OptionsMenu::new(),
            victory_screen: VictoryPop::new(),
            startup_screen,
        }
    }

    pub fn is_menu_active(&self) -> bool {
        self.options_menu.is_visible() || self.startup_screen.is_visible()
    }

    pub fn show_startup_screen(&mut self) {
        self.startup_screen.show();
    }

    pub fn show_options_menu(&mut self) {
        self.options_menu.show();
    }

    pub fn hide_options_menu(&mut self) {
        self.options_menu.hide();
    }

    pub fn handle_menu(&mut self, action: GameAction) -> Option<MenuAction> {
        // hide startup help menu on any key press
        if self.startup_screen.is_visible() {
            self.startup_screen.hide();
        }

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

    pub fn show_victory_screen(&mut self) {
        self.victory_screen.show();
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if self.startup_screen.is_visible() {
            self.startup_screen.render(area, buf);
        }

        if self.victory_screen.is_visible() {
            self.victory_screen.render(area, buf);
        }

        // Options menu always displayed last
        if self.options_menu.is_visible() {
            self.options_menu.render(area, buf);
        }
    }
}

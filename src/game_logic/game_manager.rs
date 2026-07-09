use crate::game_logic::{MenuManager, MenuAction, GameHandler, AppTransition, KlondikeGame};
use crate::controller::{Controller, GameAction};
use crate::ui::{DebugLog, MenuOption};
use crate::rendering::GameRenderer;
use ratatui::{DefaultTerminal, Frame};
use std::io;

enum AppState {
    InGame,
    Settings,
}

pub struct GameManager {
    app_state: AppState,
    game: Box<dyn GameHandler>,
    controller: Controller,
    debug_log: DebugLog,
    game_renderer: GameRenderer,
    menu_manager: MenuManager,
    quit_game: bool,
}

impl GameManager {
    pub fn new() -> Self {
        let controller = Controller::new();
        let key_bindings = controller.keybindings();
        Self {
            app_state: AppState::InGame,
            game: Box::new(KlondikeGame::new()),
            controller,
            debug_log: DebugLog::default(),
            game_renderer: GameRenderer::new(),
            menu_manager: MenuManager::new(key_bindings),
            quit_game: false,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), io::Error> {
        loop {
            if self.quit_game {
                break;
            }
            terminal.draw(|frame| self.draw(frame))?;

            if let AppState::InGame = self.app_state {
                self.game.update(&mut self.menu_manager, &mut self.debug_log);

                if self.controller.is_keyboard_mode() {
                    self.game.set_keyboard_mode(true);
                }
            }

            if let Some(action) = self.controller.poll_action()? {
                if let AppState::Settings = self.app_state {
                    if let Some(menu_action) = self.menu_manager.handle_menu(action) {
                        self.handle_settings_action(menu_action);
                    }
                } else if self.menu_manager.is_menu_active() {
                    if let Some(menu_action) = self.menu_manager.handle_menu(action) {
                        self.handle_menu_action(menu_action);
                    }
                } else {
                    self.handle_game_action(action);
                }
            }
        }

        Ok(())
    }

    fn handle_game_action(&mut self, action: GameAction) {
        let is_mouse_action = matches!(
            action,
            GameAction::LeftMousePress(_, _)
                | GameAction::StartDrag(_, _)
                | GameAction::UpdateDrag(_, _)
                | GameAction::CompleteDrag(_, _)
        );

        if is_mouse_action {
            self.game.handle_mouse_action(action, &self.game_renderer, &mut self.debug_log);
            return;
        }

        if let Some(transition) = self.game.handle_action(action, &mut self.debug_log) {
            self.handle_transition(transition);
        }
    }

    fn handle_transition(&mut self, transition: AppTransition) {
        match transition {
            AppTransition::Quit => self.quit_game = true,
            AppTransition::Restart => self.restart_game(),
            AppTransition::OpenSettings => {
                self.app_state = AppState::Settings;
                self.menu_manager.show_options_menu();
            }
            AppTransition::ShowHelp => self.menu_manager.show_startup_screen(),
        }
    }

    fn handle_settings_action(&mut self, menu_action: MenuAction) {
        match menu_action {
            MenuAction::CloseMenu => {
                self.app_state = AppState::InGame;
            }
            MenuAction::OptionSelected(option) => {
                match option {
                    MenuOption::Back => {
                        self.app_state = AppState::InGame;
                    }
                    MenuOption::Quit => {
                        self.quit_game = true;
                    }
                    MenuOption::Restart => {
                        self.restart_game();
                        self.app_state = AppState::InGame;
                    }
                    MenuOption::DeveloperMode => {
                        self.debug_log.visible = !self.debug_log.visible;
                        self.app_state = AppState::InGame;
                    }
                    MenuOption::Help => {
                        self.menu_manager.show_startup_screen();
                        self.app_state = AppState::InGame;
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_menu_action(&mut self, menu_action: MenuAction) {
        match menu_action {
            MenuAction::OptionSelected(option) => {
                match option {
                    MenuOption::Back => self.menu_manager.hide_options_menu(),
                    MenuOption::Quit => self.quit_game = true,
                    MenuOption::Restart => self.restart_game(),
                    MenuOption::DeveloperMode => self.debug_log.visible = !self.debug_log.visible,
                    MenuOption::Help => self.menu_manager.show_startup_screen(),
                }
            }
            _ => {}
        }
    }

    fn restart_game(&mut self) {
        self.game = self.game.restart();
        self.debug_log.clear();
        self.menu_manager = MenuManager::new(self.controller.keybindings());
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        self.game.draw(frame.area(), frame.buffer_mut(), &mut self.game_renderer, &self.debug_log);
        self.menu_manager.render(frame.area(), frame.buffer_mut());
    }
}

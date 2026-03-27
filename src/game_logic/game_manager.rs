use crate::game_logic::{GameState, SelectionManager, AnimationManager, MenuManager, MenuAction, BalatermState};
use crate::game_objects::{PileType, Selection, HoverState};
use crate::rendering::{GameRenderer, RenderingInstructions};
use crate::controller::{Controller, GameAction};
use crate::ui::{DebugLog, MenuOption, MainMenuOption};
use ratatui::{DefaultTerminal, Frame};
use std::io;

enum Game {
    Klondike {
        game_state: GameState,
        selection_manager: SelectionManager,
        animation_manager: AnimationManager,
        hover_state: HoverState,
    },
    Balaterm {
        game_state: BalatermState,
    },
}

enum AppState {
    MainMenu,
    InGame,
    Settings { came_from_game: bool },
}

pub struct GameManager {
    app_state: AppState,
    game: Game,
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
            app_state: AppState::MainMenu,
            game: Game::Klondike {
                game_state: GameState::new(),
                selection_manager: SelectionManager::new(),
                animation_manager: AnimationManager::new(),
                hover_state: HoverState::None,
            },
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
                if let Game::Klondike { game_state, animation_manager, selection_manager, .. } = &mut self.game {
                    animation_manager.process_game_state(game_state, &mut self.debug_log);
                    self.menu_manager.handle_game_state(game_state);

                    if self.controller.is_keyboard_mode() {
                        selection_manager.set_visible(true);
                    }
                }
            }

            if let Some(action) = self.controller.poll_action()? {
                if let AppState::MainMenu = self.app_state {
                    if let Some(menu_action) = self.menu_manager.handle_main_menu(action) {
                        self.handle_main_menu_action(menu_action);
                    }
                } else if let AppState::Settings { came_from_game } = self.app_state {
                    if let Some(menu_action) = self.menu_manager.handle_menu(action) {
                        self.handle_settings_action(menu_action, came_from_game);
                    }
                } else if self.menu_manager.is_menu_active() {
                    if let Some(menu_action) = self.menu_manager.handle_menu(action) {
                        self.handle_menu_action(menu_action);
                    }
                } else {
                    match &self.game {
                        Game::Klondike { .. } => self.handle_klondike_action(action),
                        Game::Balaterm { .. } => self.handle_balaterm_action(action),
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_klondike_action(&mut self, action: GameAction) {
        match action {
            GameAction::Quit => self.quit_game = true,
            GameAction::MoveLeft => {
                let Game::Klondike { game_state, selection_manager, .. } = &mut self.game else { return };
                let valid_moves = selection_manager.picked_up().map(|pu| Self::compute_valid_moves(game_state, &pu));
                selection_manager.move_left(game_state.board(), valid_moves.as_ref());
            }
            GameAction::MoveRight => {
                let Game::Klondike { game_state, selection_manager, .. } = &mut self.game else { return };
                let valid_moves = selection_manager.picked_up().map(|pu| Self::compute_valid_moves(game_state, &pu));
                selection_manager.move_right(game_state.board(), valid_moves.as_ref());
            }
            GameAction::MoveUp => {
                let Game::Klondike { game_state, selection_manager, .. } = &mut self.game else { return };
                let valid_moves = selection_manager.picked_up().map(|pu| Self::compute_valid_moves(game_state, &pu));
                selection_manager.move_up(game_state.board(), valid_moves.as_ref());
            }
            GameAction::MoveDown => {
                let Game::Klondike { game_state, selection_manager, .. } = &mut self.game else { return };
                let valid_moves = selection_manager.picked_up().map(|pu| Self::compute_valid_moves(game_state, &pu));
                selection_manager.move_down(game_state.board(), valid_moves.as_ref());
            }
            GameAction::Select | GameAction::Enter => {
                self.handle_select_action();
            }
            GameAction::Cancel => {
                let Game::Klondike { selection_manager, .. } = &mut self.game else { return };
                selection_manager.cancel_pickup();
            }
            GameAction::DrawStock => {
                let Game::Klondike { game_state, .. } = &mut self.game else { return };
                let _ = game_state.draw_from_stock();
            }
            GameAction::Undo => {}
            GameAction::Restart => {
                self.restart_game();
            }
            GameAction::OptionsMenu => {
                self.app_state = AppState::Settings { came_from_game: true };
                self.menu_manager.show_options_menu();
            }
            GameAction::LeftMousePress(x, y) => {
                self.handle_left_mouse_press(x, y);
            }
            GameAction::Click => {
                self.handle_click();
            }
            GameAction::StartDrag(x, y) => {
                self.handle_start_drag(x, y);
            }
            GameAction::UpdateDrag(x, y) => {
                self.handle_update_drag(x, y);
            }
            GameAction::CompleteDrag(x, y) => {
                self.handle_complete_drag(x, y);
            }
            GameAction::CancelDrag => {
                let Game::Klondike { selection_manager, hover_state, .. } = &mut self.game else { return };
                selection_manager.cancel_pickup();
                *hover_state = HoverState::None;
            }
            GameAction::HelpMenu => {
                self.menu_manager.show_startup_screen();
            }
            _ => {}
        }
    }

    fn handle_balaterm_action(&mut self, action: GameAction) {
        match action {
            GameAction::Quit => self.quit_game = true,
            GameAction::OptionsMenu => {
                self.app_state = AppState::Settings { came_from_game: true };
                self.menu_manager.show_options_menu();
            }
            GameAction::HelpMenu => {
                self.menu_manager.show_startup_screen();
            }
            _ => {}
        }
    }

    fn handle_select_action(&mut self) {
        let Game::Klondike { game_state, selection_manager, .. } = &mut self.game else { return };
        if selection_manager.has_picked_up() {
            let source = selection_manager.picked_up().unwrap();
            let target = selection_manager.selection();
            if let Err(msg) = game_state.place_cards(source, target) {
                self.debug_log.log(format!("{}", msg));
                selection_manager.cancel_pickup();
            } else {
                selection_manager.place();
            }
        } else {
            let selection = selection_manager.selection();
            if let Err(msg) = game_state.pick_up_cards(selection) {
                self.debug_log.log(format!("{}", msg));
            } else {
                selection_manager.pick_up();
                let valid_moves = Self::compute_valid_moves(game_state, &selection);
                if !valid_moves.is_empty() {
                    selection_manager.set_selection(valid_moves[0]);
                }
            }
        }
    }

    fn handle_left_mouse_press(&mut self, x: u16, y: u16) {
        let Game::Klondike { game_state, selection_manager, .. } = &mut self.game else { return };
        if let Some(selection) = self.game_renderer.coordinate_to_selection(game_state.board(), x, y) {
            selection_manager.set_selection(selection);
            selection_manager.set_visible(true);
        }
    }

    fn handle_click(&mut self) {
        let Game::Klondike { game_state, selection_manager, .. } = &mut self.game else { return };
        let selection = selection_manager.selection();
        if selection.pile == PileType::Stock && !selection_manager.has_picked_up() {
            if let Err(msg) = game_state.draw_from_stock() {
                self.debug_log.log(msg);
            }
            return;
        }

        let moves = Self::compute_valid_moves(game_state, &selection);

        if let Some(first_move) = moves.first() {
            if let Err(msg) = game_state.pick_up_cards(selection) {
                self.debug_log.log(format!("{}", msg));
            } else {
                selection_manager.pick_up();
                selection_manager.set_selection(*first_move);
                let source = selection_manager.picked_up().unwrap();
                let target = selection_manager.selection();
                if let Err(msg) = game_state.place_cards(source, target) {
                    self.debug_log.log(format!("{}", msg));
                    selection_manager.cancel_pickup();
                } else {
                    selection_manager.place();
                }
            }
        }
        selection_manager.set_visible(false);
    }

    fn handle_start_drag(&mut self, x: u16, y: u16) {
        let Game::Klondike { game_state, selection_manager, .. } = &mut self.game else { return };
        if let Some(selection) = self.game_renderer.coordinate_to_selection(game_state.board(), x, y) {
            selection_manager.set_selection(selection);
            if let Err(msg) = game_state.pick_up_cards(selection) {
                self.debug_log.log(format!("{}", msg));
            } else {
                selection_manager.pick_up();
            }
        }
    }

    fn handle_update_drag(&mut self, x: u16, y: u16) {
        let Game::Klondike { game_state, selection_manager, hover_state, .. } = &mut self.game else { return };
        if selection_manager.has_picked_up() {
            if let Some(target) = self.game_renderer.coordinate_to_selection(game_state.board(), x, y) {
                let Some(source) = selection_manager.picked_up() else {
                    self.debug_log.log("Error: selection not found for dragging");
                    return;
                };
                let is_valid = game_state.is_valid_placement(source, target);
                *hover_state = if is_valid {
                    HoverState::Valid(target)
                } else {
                    HoverState::Invalid(target)
                };
            } else {
                *hover_state = HoverState::None;
            }
        }
    }

    fn handle_complete_drag(&mut self, x: u16, y: u16) {
        let Game::Klondike { game_state, selection_manager, hover_state: _, .. } = &mut self.game else { return };
        if let Some(target) = self.game_renderer.coordinate_to_selection(game_state.board(), x, y) {
            if selection_manager.has_picked_up() {
                selection_manager.set_selection(target);
                let Some(source) = selection_manager.picked_up() else {
                    self.debug_log.log("Could not find picked up card for drag");
                    return;
                };
                if let Err(val) = game_state.place_cards(source, target) {
                    self.debug_log.log(format!("{}", val));
                    selection_manager.cancel_pickup();
                } else {
                    selection_manager.place();
                }
                selection_manager.set_visible(false);
            }
        }

        let Game::Klondike { selection_manager, hover_state, .. } = &mut self.game else { return };
        if selection_manager.has_picked_up() {
            selection_manager.cancel_pickup();
        }
        *hover_state = HoverState::None;
    }

    fn handle_main_menu_action(&mut self, menu_action: MenuAction) {
        match menu_action {
            MenuAction::MainMenuSelected(option) => match option {
                MainMenuOption::Klondike => {
                    self.game = Game::Klondike {
                        game_state: GameState::new(),
                        selection_manager: SelectionManager::new(),
                        animation_manager: AnimationManager::new(),
                        hover_state: HoverState::None,
                    };
                    self.app_state = AppState::InGame;
                }
                MainMenuOption::Balaterm => {
                    self.game = Game::Balaterm {
                        game_state: BalatermState::new(),
                    };
                    self.app_state = AppState::InGame;
                }
                MainMenuOption::Settings => {
                    self.app_state = AppState::Settings { came_from_game: false };
                    self.menu_manager.show_options_menu();
                }
                MainMenuOption::Quit => {
                    self.quit_game = true;
                }
            },
            _ => {}
        }
    }

    fn handle_settings_action(&mut self, menu_action: MenuAction, came_from_game: bool) {
        match menu_action {
            MenuAction::CloseMenu => {
                if came_from_game {
                    self.app_state = AppState::InGame;
                } else {
                    self.app_state = AppState::MainMenu;
                    self.menu_manager.show_main_menu();
                }
            }
            MenuAction::OptionSelected(option) => {
                match option {
                    MenuOption::Back => {
                        if came_from_game {
                            self.app_state = AppState::InGame;
                        } else {
                            self.app_state = AppState::MainMenu;
                            self.menu_manager.show_main_menu();
                        }
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
                        self.app_state = if came_from_game { AppState::InGame } else { AppState::MainMenu };
                        if !came_from_game {
                            self.menu_manager.show_main_menu();
                        }
                    }
                    MenuOption::Help => {
                        self.menu_manager.show_startup_screen();
                        self.app_state = if came_from_game { AppState::InGame } else { AppState::MainMenu };
                        if !came_from_game {
                            self.menu_manager.show_main_menu();
                        }
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
                    MenuOption::Back => {
                        self.menu_manager.hide_options_menu();
                    }
                    MenuOption::Quit => {
                        self.quit_game = true;
                    }
                    MenuOption::Restart => {
                        self.restart_game();
                    }
                    MenuOption::DeveloperMode => {
                        self.debug_log.visible = !self.debug_log.visible;
                    }
                    MenuOption::Help => {
                        self.menu_manager.show_startup_screen();
                    }
                }
            }
            _ => {}
        }
    }

    fn restart_game(&mut self) {
        self.game = match self.game {
            Game::Klondike { .. } => Game::Klondike {
                game_state: GameState::new(),
                selection_manager: SelectionManager::new(),
                animation_manager: AnimationManager::new(),
                hover_state: HoverState::None,
            },
            Game::Balaterm { .. } => Game::Balaterm {
                game_state: BalatermState::new(),
            },
        };
        self.debug_log.clear();
        self.menu_manager = MenuManager::new(self.controller.keybindings());
        self.menu_manager.hide_main_menu();
    }

    fn compute_valid_moves(game_state: &GameState, selection: &Selection) -> Vec<Selection> {
        let mut valid_moves = Vec::new();
        let board = game_state.board();

        let Some(card) = board.get_card_at_selection(selection) else {
            return valid_moves;
        };

        if let Some(pile) = board.get_pile(selection.pile, selection.pile_index) && selection.card_index == pile.len() - 1 {
            for i in 0..4 {
                if let Some(pile) = board.get_foundation_pile(i) {
                    if pile.can_place_card(card) {
                        valid_moves.push(Selection::new(PileType::Foundation, i, pile.len()));
                    }
                }
            }
        }

        for i in 0..7 {
            if let Some(pile) = board.get_tableau_pile(i) {
                if pile.can_place_card(card) {
                    valid_moves.push(Selection::new(PileType::Tableau, i, pile.len()));
                }
            }
        }

        valid_moves
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        match self.app_state {
            AppState::MainMenu => {
                self.menu_manager.render_main_menu(frame.area(), frame.buffer_mut());
            }
            AppState::Settings { came_from_game } => {
                if came_from_game {
                    if let Game::Klondike { game_state, selection_manager, hover_state, .. } = &self.game {
                        let selection = selection_manager.selection_if_visible();
                        let picked_up = selection_manager.picked_up();
                        let rendering_instr = RenderingInstructions::new(
                            "=== Klondike ===",
                            game_state.board(),
                            selection,
                            picked_up.as_ref(),
                            hover_state,
                            &self.debug_log,
                        );
                        self.game_renderer.render(frame.area(), frame.buffer_mut(), &rendering_instr);
                    }
                } else {
                    self.menu_manager.render_main_menu(frame.area(), frame.buffer_mut());
                }
                self.menu_manager.render(frame.area(), frame.buffer_mut());
            }
            AppState::InGame => {
                match &self.game {
                    Game::Klondike { game_state, selection_manager, hover_state, .. } => {
                        let selection = selection_manager.selection_if_visible();
                        let picked_up = selection_manager.picked_up();
                        let rendering_instr = RenderingInstructions::new(
                            "=== Klondike ===",
                            game_state.board(),
                            selection,
                            picked_up.as_ref(),
                            hover_state,
                            &self.debug_log,
                        );
                        self.game_renderer.render(frame.area(), frame.buffer_mut(), &rendering_instr);
                    }
                    Game::Balaterm { .. } => {}
                }
                self.menu_manager.render(frame.area(), frame.buffer_mut());
            }
        }
    }
}

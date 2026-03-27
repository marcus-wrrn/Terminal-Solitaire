use crate::game_logic::{GameState, SelectionManager, AnimationManager, HoverState, MenuManager, MenuAction};
use crate::game_objects::{PileType, Selection};
use crate::rendering::{GameRenderer, RenderingInstructions};
use crate::controller::{Controller, GameAction};
use crate::ui::{DebugLog, MenuOption, MainMenuOption};
use ratatui::{DefaultTerminal, Frame};
use std::io;

enum AppState {
    MainMenu,
    InGame,
}

pub struct GameManager {
    app_state: AppState,
    game_state: GameState,
    selection_manager: SelectionManager,
    controller: Controller,
    debug_log: DebugLog,
    game_renderer: GameRenderer,
    hover_state: HoverState,
    menu_manager: MenuManager,
    animation_manager: AnimationManager,
    quit_game: bool,
}

impl GameManager {
    pub fn new() -> Self {
        let controller = Controller::new();
        let key_bindings = controller.keybindings();
        Self {
            app_state: AppState::MainMenu,
            game_state: GameState::new(),
            selection_manager: SelectionManager::new(),
            controller: controller,
            debug_log: DebugLog::default(),
            game_renderer: GameRenderer::new(),
            hover_state: HoverState::None,
            menu_manager: MenuManager::new(key_bindings),
            animation_manager: AnimationManager::new(),
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
                self.animation_manager.process_game_state(&mut self.game_state, &mut self.debug_log);
                self.menu_manager.handle_game_state(&self.game_state);

                if self.controller.is_keyboard_mode() {
                    self.selection_manager.set_visible(true);
                }
            }

            if let Some(action) = self.controller.poll_action()? {
                if let AppState::MainMenu = self.app_state {
                    if let Some(menu_action) = self.menu_manager.handle_main_menu(action) {
                        self.handle_main_menu_action(menu_action);
                    }
                } else if self.menu_manager.is_menu_active() {
                    if let Some(menu_action) = self.menu_manager.handle_menu(action) {
                        self.handle_menu_action(menu_action);
                    }
                } else {
                    match action {
                        GameAction::Quit => self.quit_game = true,
                        GameAction::MoveLeft => {
                            let valid_moves = if let Some(picked_up) = self.selection_manager.picked_up() {
                                Some(self.find_valid_moves(&picked_up))
                            } else {
                                None
                            };
                            self.selection_manager.move_left(self.game_state.board(), valid_moves.as_ref());
                        }
                        GameAction::MoveRight => {
                            let valid_moves = if let Some(picked_up) = self.selection_manager.picked_up() {
                                Some(self.find_valid_moves(&picked_up))
                            } else {
                                None
                            };
                            self.selection_manager.move_right(self.game_state.board(), valid_moves.as_ref());
                        }
                        GameAction::MoveUp => {
                            let valid_moves = if let Some(picked_up) = self.selection_manager.picked_up() {
                                Some(self.find_valid_moves(&picked_up))
                            } else {
                                None
                            };
                            self.selection_manager.move_up(self.game_state.board(), valid_moves.as_ref());
                        }
                        GameAction::MoveDown => {
                            let valid_moves = if let Some(picked_up) = self.selection_manager.picked_up() {
                                Some(self.find_valid_moves(&picked_up))
                            } else {
                                None
                            };
                            self.selection_manager.move_down(self.game_state.board(), valid_moves.as_ref());
                        }
                        GameAction::Select | GameAction::Enter => {
                            self.handle_select_action();
                        }
                        GameAction::Cancel => {
                            self.selection_manager.cancel_pickup();
                        }
                        GameAction::DrawStock => {
                            let _ = self.game_state.draw_from_stock();
                        }
                        GameAction::Undo => {
                        }
                        GameAction::Restart => {
                            self.restart_game();
                        }
                        GameAction::OptionsMenu => {
                            self.menu_manager.toggle_options_menu();
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
                            self.selection_manager.cancel_pickup();
                            self.hover_state = HoverState::None;
                        }
                        GameAction::HelpMenu => {
                            self.menu_manager.show_startup_screen();
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_select_action(&mut self) {
        if self.selection_manager.has_picked_up() {
            let source = self.selection_manager.picked_up().unwrap();
            let target = self.selection_manager.selection();
            if let Err(msg) = self.game_state.place_cards(source, target) {
                self.debug_log.log(format!("{}", msg));
                self.selection_manager.cancel_pickup();
            } else {
                self.selection_manager.place();
            }
        } else {
            let selection = self.selection_manager.selection();
            if let Err(msg) = self.game_state.pick_up_cards(selection) {
                self.debug_log.log(format!("{}", msg));
            } else {
                self.selection_manager.pick_up();
                let valid_moves = self.find_valid_moves(&selection);
                if !valid_moves.is_empty() {
                    self.selection_manager.set_selection(valid_moves[0]);
                }
            }
        }
    }

    fn handle_left_mouse_press(&mut self, x: u16, y: u16) {
        if let Some(selection) = self.game_renderer.coordinate_to_selection(self.game_state.board(), x, y) {
            self.selection_manager.set_selection(selection);
            self.selection_manager.set_visible(true);
        }
    }

    fn handle_click(&mut self) {
        let selection = self.selection_manager.selection();
        if selection.pile == PileType::Stock && !self.selection_manager.has_picked_up() {
            self.handle_stock_click();
            return;
        }

        let moves = self.find_valid_moves(&selection);

        if let Some(first_move) = moves.first() {
            if let Err(msg) = self.game_state.pick_up_cards(selection) {
                self.debug_log.log(format!("{}", msg));
            } else {
                self.selection_manager.pick_up();
                self.selection_manager.set_selection(*first_move);
                let source = self.selection_manager.picked_up().unwrap();
                let target = self.selection_manager.selection();
                if let Err(msg) = self.game_state.place_cards(source, target) {
                    self.debug_log.log(format!("{}", msg));
                    self.selection_manager.cancel_pickup();
                } else {
                    self.selection_manager.place();
                }
            }
        }
        self.selection_manager.set_visible(false);
    }

    fn handle_start_drag(&mut self, x: u16, y: u16) {
        if let Some(selection) = self.game_renderer.coordinate_to_selection(self.game_state.board(), x, y) {
            self.selection_manager.set_selection(selection);
            if let Err(msg) = self.game_state.pick_up_cards(selection) {
                self.debug_log.log(format!("{}", msg));
            } else {
                self.selection_manager.pick_up();
            }
        }
    }

    fn handle_stock_click(&mut self) {
        if let Err(msg) = self.game_state.draw_from_stock() {
            self.debug_log.log(msg);
        }
    }

    fn handle_update_drag(&mut self, x: u16, y: u16) {
        if self.selection_manager.has_picked_up() {
            if let Some(target) = self.game_renderer.coordinate_to_selection(self.game_state.board(), x, y) {
                let Some(source) = self.selection_manager.picked_up() else {
                    self.debug_log.log("Error: selection not found for dragging");
                    return;
                };
                let is_valid = self.game_state.is_valid_placement(source, target);
                self.hover_state = if is_valid {
                    HoverState::Valid(target)
                } else {
                    HoverState::Invalid(target)
                }
            } else {
                self.hover_state = HoverState::None;
            }
        }
    }

    fn handle_complete_drag(&mut self, x: u16, y: u16) {
        if let Some(target) = self.game_renderer.coordinate_to_selection(self.game_state.board(), x, y) {
            if self.selection_manager.has_picked_up() {
                self.selection_manager.set_selection(target);
                let Some(source) = self.selection_manager.picked_up() else {
                    self.debug_log.log("Could not find picked up card for drag");
                    return;
                };
                if let Err(val) = self.game_state.place_cards(source, target) {
                    self.debug_log.log(format!("{}", val));
                    self.selection_manager.cancel_pickup();
                } else {
                    self.selection_manager.place();
                }
                self.selection_manager.set_visible(false);
            }
        }

        if self.selection_manager.has_picked_up() {
            self.selection_manager.cancel_pickup();
        }
        self.hover_state = HoverState::None;
    }

    fn handle_main_menu_action(&mut self, menu_action: MenuAction) {
        match menu_action {
            MenuAction::MainMenuSelected(option) => match option {
                MainMenuOption::Play => {
                    self.app_state = AppState::InGame;
                }
                MainMenuOption::Settings => {
                    self.app_state = AppState::InGame;
                    self.menu_manager.toggle_options_menu();
                }
                MainMenuOption::Quit => {
                    self.quit_game = true;
                }
            },
            _ => {}
        }
    }

    fn handle_menu_action(&mut self, menu_action: MenuAction) {
        match menu_action {
            MenuAction::OptionSelected(option) => {
                match option {
                    MenuOption::Quit => {
                        self.quit_game = true;
                    }
                    MenuOption::Restart => {
                        self.restart_game();
                    }
                    // MenuOption::RebindKeys => {
                    // }
                    MenuOption::DeveloperMode => {
                        self.debug_log.visible = !self.debug_log.visible; // toggle debug log
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
        self.game_state = GameState::new();
        self.selection_manager = SelectionManager::new();
        self.debug_log.clear();
        self.hover_state = HoverState::None;
        self.animation_manager.stop_animation();
        self.menu_manager = MenuManager::new(self.controller.keybindings());
        self.menu_manager.hide_main_menu();
    }

    pub fn find_valid_moves(&self, selection: &Selection) -> Vec<crate::game_objects::Selection> {
        let mut valid_moves = Vec::new();
        let board = self.game_state.board();

        let Some(card) = board.get_card_at_selection(selection) else {
            return valid_moves;
        };

        // If card is the last in the pile then check foundation
        if let Some(pile) = board.get_pile(selection.pile, selection.pile_index) && selection.card_index == pile.len() - 1 {
            for i in 0..4 {
                if let Some(pile) = board.get_foundation_pile(i) {
                    if pile.can_place_card(card) {
                        valid_moves.push(crate::game_objects::Selection::new(
                            crate::game_objects::PileType::Foundation,
                            i,
                            pile.len()
                        ));
                    }
                }
            }
        } 

        // check tableau
        for i in 0..7 {
            if let Some(pile) = board.get_tableau_pile(i) {
                if pile.can_place_card(card) {
                    valid_moves.push(crate::game_objects::Selection::new(
                        crate::game_objects::PileType::Tableau,
                        i,
                        pile.len()
                    ));
                }
            }
        }

        valid_moves
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        if let AppState::MainMenu = self.app_state {
            self.menu_manager.render_main_menu(frame.area(), frame.buffer_mut());
            return;
        }

        let selection = self.selection_manager.selection_if_visible();
        let picked_up = self.selection_manager.picked_up();
        let rendering_instr = RenderingInstructions::new(
            self.game_state.board(),
            selection,
            picked_up.as_ref(),
            &self.hover_state,
            &self.debug_log
        );

        self.game_renderer.render(frame.area(), frame.buffer_mut(), &rendering_instr);
        self.menu_manager.render(frame.area(), frame.buffer_mut());
    }
}

use crate::game_logic::{GameState, SelectionNavigator, AnimationManager, HoverState, MenuManager, MenuAction};
use crate::rendering::{GameRenderer, BoardRenderer};
use crate::controller::{Controller, GameAction};
use crate::ui::{DebugLog, WinPopup, MenuOption};
use ratatui::{DefaultTerminal, Frame};
use std::io;

pub struct GameManager {
    game_state: GameState,
    controller: Controller,
    debug_log: DebugLog,
    board_renderer: BoardRenderer,
    hover_state: HoverState,
    menu_manager: MenuManager,
    win_popup: WinPopup,
    animation_manager: AnimationManager,
}

impl GameManager {
    pub fn new() -> Self {
        let mut win_popup = WinPopup::new_with_title("Congratulations!!".to_string());
        win_popup.add_line(ratatui::text::Line::from(vec![
            ratatui::text::Span::styled(
                "YOU WON!",
                ratatui::style::Style::default()
                    .fg(ratatui::style::Color::Green)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            ),
        ]));

        Self {
            game_state: GameState::new(),
            controller: Controller::new(),
            debug_log: DebugLog::default(),
            board_renderer: BoardRenderer::new(),
            hover_state: HoverState::None,
            menu_manager: MenuManager::new(),
            win_popup,
            animation_manager: AnimationManager::new(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), io::Error> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            self.animation_manager.process_game_state(&mut self.game_state, &mut self.debug_log);

            if self.game_state.has_won() && !self.win_popup.is_visible() {
                self.win_popup.show();
            }

            if let Some(action) = self.controller.poll_action()? {
                if self.win_popup.is_visible() {
                    match action {
                        GameAction::Quit => break,
                        GameAction::Restart => {
                            self.restart_game();
                        }
                        _ => {}
                    }
                } else if self.menu_manager.is_menu_active() {
                    if let Some(menu_action) = self.menu_manager.handle_menu_action(action) {
                        self.handle_menu_action(menu_action);
                    }
                } else {
                    match action {
                        GameAction::Quit => break,
                        GameAction::MoveLeft => {
                            let new_selection = SelectionNavigator::move_left(self.game_state.board(), self.game_state.selection());
                            self.game_state.set_selection(new_selection);
                        }
                        GameAction::MoveRight => {
                            let new_selection = SelectionNavigator::move_right(self.game_state.board(), self.game_state.selection());
                            self.game_state.set_selection(new_selection);
                        }
                        GameAction::MoveUp => {
                            let new_selection = SelectionNavigator::move_up(self.game_state.board(), self.game_state.selection());
                            self.game_state.set_selection(new_selection);
                        }
                        GameAction::MoveDown => {
                            let new_selection = SelectionNavigator::move_down(self.game_state.board(), self.game_state.selection());
                            self.game_state.set_selection(new_selection);
                        }
                        GameAction::Select | GameAction::Enter => {
                            self.handle_select_action();
                        }
                        GameAction::Cancel => {
                            self.game_state.cancel_pickup();
                        }
                        GameAction::DrawStock => {
                            let _ = self.game_state.draw_from_stock();
                        }
                        GameAction::Undo => {
                        }
                        GameAction::Restart => {
                            self.game_state = GameState::new();
                            self.debug_log.clear();
                            self.hover_state = HoverState::None;
                            self.animation_manager.stop_animation();
                        }
                        GameAction::Help => {
                            self.menu_manager.toggle_options_menu();
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
                            self.game_state.cancel_pickup();
                            self.hover_state = HoverState::None;
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_select_action(&mut self) {
        if self.game_state.has_picked_up_cards() {
            if let Err(msg) = self.game_state.place_cards() {
                self.debug_log.log(format!("{}", msg));
            }
        } else {
            if let Err(msg) = self.game_state.pick_up_cards() {
                self.debug_log.log(format!("{}", msg));
            }
        }
    }

    fn handle_start_drag(&mut self, x: u16, y: u16) {
        if let Some(selection) = self.board_renderer.coordinate_to_selection(self.game_state.board(), x, y) {
            self.game_state.set_selection(selection);
            if let Err(msg) = self.game_state.pick_up_cards() {
                self.debug_log.log(format!("{}", msg));
            }
        }
    }

    fn handle_stock_click(&mut self) {
        if let Err(msg) = self.game_state.draw_from_stock() {
            self.debug_log.log(msg);
        }
    }

    fn handle_update_drag(&mut self, x: u16, y: u16) {
        if self.game_state.has_picked_up_cards() {
            if let Some(target) = self.board_renderer.coordinate_to_selection(self.game_state.board(), x, y) {
                let is_valid = self.game_state.is_valid_placement(&target);
                self.hover_state = if is_valid {
                    HoverState::Valid(target)
                } else {
                    HoverState::Invalid(target)
                };
            } else {
                self.hover_state = HoverState::None;
            }
        }
    }

    fn handle_complete_drag(&mut self, x: u16, y: u16) {
        if let Some(target) = self.board_renderer.coordinate_to_selection(self.game_state.board(), x, y) {
            if target.pile == crate::game_objects::PileType::Stock && !self.game_state.has_picked_up_cards() {
                self.handle_stock_click();
            } else if self.game_state.has_picked_up_cards() {
                self.game_state.set_selection(target);
                if let Err(val) = self.game_state.place_cards() {
                    self.debug_log.log(format!("{}", val));
                }
            }
        }

        if self.game_state.has_picked_up_cards() {
            self.game_state.cancel_pickup();
        }
        self.hover_state = HoverState::None;
    }

    fn handle_menu_action(&mut self, menu_action: MenuAction) {
        match menu_action {
            MenuAction::OptionSelected(option) => {
                match option {
                    MenuOption::Restart => {
                        self.restart_game();
                    }
                    MenuOption::RebindKeys => {
                    }
                    MenuOption::DeveloperMode => {
                    }
                    MenuOption::Help => {
                    }
                }
            }
            _ => {}
        }
    }

    fn restart_game(&mut self) {
        self.game_state = GameState::new();
        self.debug_log.clear();
        self.hover_state = HoverState::None;
        self.animation_manager.stop_animation();
        self.win_popup.hide();
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let selection = self.game_state.selection();
        let game_renderer = GameRenderer::new(
            self.game_state.board(),
            &selection,
            &self.hover_state,
            &self.debug_log,
            &mut self.board_renderer
        );

        frame.render_widget(game_renderer, frame.area());

        self.menu_manager.render(frame.area(), frame.buffer_mut());

        if self.win_popup.is_visible() {
            self.win_popup.render(frame.area(), frame.buffer_mut());
        }
    }
}

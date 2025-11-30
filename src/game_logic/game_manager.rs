use crate::game_objects::Selection;
use crate::game_logic::{GameState, SelectionNavigator};
use crate::rendering::{GameRenderer, BoardRenderer};
use crate::controller::{Controller, GameAction};
use crate::ui::{DebugLog, OptionsMenu};
use ratatui::{DefaultTerminal, Frame};
use std::io;
// use std::rc::Rc;
// use std::cell::RefCell;

pub struct GameManager {
    game_state: GameState,
    controller: Controller,
    debug_log: DebugLog,
    board_renderer: BoardRenderer,
    hover_selection: Option<Selection>,
    options_menu: OptionsMenu,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            game_state: GameState::new(),
            controller: Controller::new(),
            debug_log: DebugLog::default(),
            board_renderer: BoardRenderer::new(),
            hover_selection: None,
            options_menu: OptionsMenu::new(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), io::Error> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Some(action) = self.controller.poll_action()? {
                if self.options_menu.is_visible() {
                    match action {
                        GameAction::Quit | GameAction::Cancel | GameAction::Help => {
                            self.options_menu.hide();
                        }
                        GameAction::MoveUp => {
                            self.options_menu.move_up();
                        }
                        GameAction::MoveDown => {
                            self.options_menu.move_down();
                        }
                        GameAction::Select | GameAction::Enter => {
                        }
                        _ => {}
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
                            self.hover_selection = None;
                        }
                        GameAction::Help => {
                            self.options_menu.toggle();
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
                            self.hover_selection = None;
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
            let _ = self.game_state.place_cards();
        } else {
            let _ = self.game_state.pick_up_cards();
        }
    }

        fn handle_start_drag(&mut self, x: u16, y: u16) {
        if let Some(selection) = self.board_renderer.coordinate_to_selection(self.game_state.board(), x, y) {
            self.game_state.set_selection(selection);
            let _ = self.game_state.pick_up_cards();
        }
    }

    fn handle_update_drag(&mut self, x: u16, y: u16) {
        if self.game_state.has_picked_up_cards() {
            self.hover_selection = self.board_renderer.coordinate_to_selection(self.game_state.board(), x, y);
        }
    }

    fn handle_complete_drag(&mut self, x: u16, y: u16) {
        if self.game_state.has_picked_up_cards() {
            if let Some(target) = self.board_renderer.coordinate_to_selection(self.game_state.board(), x, y) {
                self.game_state.set_selection(target);
                if let Err(val) = self.game_state.place_cards() {
                    self.debug_log.log(format!("{}", val));
                }
            } else {
                self.game_state.cancel_pickup();
            }
        }
        self.hover_selection = None;
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let selection = self.game_state.selection();
        let game_renderer = GameRenderer::new(
            self.game_state.board(),
            &selection,
            self.hover_selection.as_ref(),
            &self.debug_log,
            &mut self.board_renderer
        );

        frame.render_widget(game_renderer, frame.area());

        if self.options_menu.is_visible() {
            self.options_menu.render(frame.area(), frame.buffer_mut());
        }
    }
}

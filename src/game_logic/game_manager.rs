use crate::game_objects::{PileType, Selection};
use crate::game_logic::GameState;
use crate::rendering::{GameRenderer, BoardRenderer};
use crate::controller::{Controller, GameAction};
use crate::ui::DebugLog;
use ratatui::{DefaultTerminal, Frame};
use std::io;
use std::rc::Rc;
use std::cell::RefCell;

pub struct GameManager {
    game_state: GameState,
    controller: Controller,
    debug_log: Rc<RefCell<DebugLog>>,
    board_renderer: BoardRenderer,
    hover_selection: Option<Selection>,
}

impl GameManager {
    pub fn new() -> Self {
        let debug_log = Rc::new(RefCell::new(DebugLog::default()));
        Self {
            game_state: GameState::new(Rc::clone(&debug_log)),
            controller: Controller::new(),
            debug_log,
            board_renderer: BoardRenderer::new(),
            hover_selection: None,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), io::Error> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Some(action) = self.controller.poll_action()? {
                match action {
                    GameAction::Quit => break,
                    GameAction::MoveLeft => self.move_selection_left(),
                    GameAction::MoveRight => self.move_selection_right(),
                    GameAction::MoveUp => self.move_selection_up(),
                    GameAction::MoveDown => self.move_selection_down(),
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
                        // TODO: Implement undo logic
                    }
                    GameAction::Restart => {
                        // TODO: Implement restart logic
                    }
                    GameAction::Help => {
                        // TODO: Implement help display
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

        Ok(())
    }

    fn get_topmost_card_index(&self, pile_type: PileType, pile_index: usize) -> usize {
        match pile_type {
            PileType::Tableau => {
                self.game_state.board()
                    .get_tableau_pile(pile_index)
                    .map(|p| if p.len() > 0 { p.len() - 1 } else { 0 })
                    .unwrap_or(0)
            }
            PileType::Foundation => {
                self.game_state.board()
                    .get_foundation_pile(pile_index)
                    .map(|p| if p.len() > 0 { p.len() - 1 } else { 0 })
                    .unwrap_or(0)
            }
            PileType::Waste => {
                let len = self.game_state.board().waste.len();
                if len > 0 { len - 1 } else { 0 }
            }
            PileType::Stock => {
                let len = self.game_state.board().stock.len();
                if len > 0 { len - 1 } else { 0 }
            }
        }
    }

    fn move_selection_left(&mut self) {
        let current = self.game_state.selection();

        match current.pile {
            PileType::Tableau => {
                if current.pile_index > 0 {
                    let next_pile_index = current.pile_index - 1;
                    let card_index = self.get_topmost_card_index(PileType::Tableau, next_pile_index);
                    self.game_state.set_selection(Selection::new(
                        PileType::Tableau,
                        next_pile_index,
                        card_index
                    ));
                }
            }
            PileType::Foundation => {
                if current.pile_index > 0 {
                    let next_pile_index = current.pile_index - 1;
                    let card_index = self.get_topmost_card_index(PileType::Foundation, next_pile_index);
                    self.game_state.set_selection(Selection::new(
                        PileType::Foundation,
                        next_pile_index,
                        card_index
                    ));
                } else {
                    let card_index = self.get_topmost_card_index(PileType::Waste, 0);
                    self.game_state.set_selection(Selection::new(
                        PileType::Waste,
                        0,
                        card_index
                    ));
                }
            }
            PileType::Waste => {
                let card_index = self.get_topmost_card_index(PileType::Stock, 0);
                self.game_state.set_selection(Selection::new(
                    PileType::Stock,
                    0,
                    card_index
                ));
            }
            PileType::Stock => {}
        }
    }

    fn move_selection_right(&mut self) {
        let current = self.game_state.selection();

        match current.pile {
            PileType::Tableau => {
                if current.pile_index < 6 {
                    let next_pile_index = current.pile_index + 1;
                    let card_index = self.get_topmost_card_index(PileType::Tableau, next_pile_index);

                    self.game_state.set_selection(Selection::new(
                        PileType::Tableau,
                        next_pile_index,
                        card_index
                    ));
                }
            }
            PileType::Foundation => {
                if current.pile_index < 3 {
                    let card_index = self.get_topmost_card_index(PileType::Foundation, current.pile_index + 1);
                    self.game_state.set_selection(Selection::new(
                        PileType::Foundation,
                        current.pile_index + 1,
                        card_index
                    ));
                }
            }
            PileType::Stock => {
                let card_index = self.get_topmost_card_index(PileType::Waste, 0);
                self.game_state.set_selection(Selection::new(
                    PileType::Waste,
                    0,
                    card_index
                ));
            }
            PileType::Waste => {
                let card_index = self.get_topmost_card_index(PileType::Foundation, 0);
                self.game_state.set_selection(Selection::new(
                    PileType::Foundation,
                    0,
                    card_index
                ));
            }
        }
    }

    fn move_selection_up(&mut self) {
        let current = self.game_state.selection();

        match current.pile {
            PileType::Tableau => {
                if current.card_index > 0 {
                    self.game_state.set_selection(Selection::new(
                        PileType::Tableau,
                        current.pile_index,
                        current.card_index - 1
                    ));
                } else {
                    if current.pile_index < 4 {
                        let card_index = self.get_topmost_card_index(PileType::Stock, 0);
                        self.game_state.set_selection(Selection::new(
                            PileType::Stock,
                            0,
                            card_index
                        ));
                    } else {
                        let card_index = self.get_topmost_card_index(PileType::Foundation, 0);
                        self.game_state.set_selection(Selection::new(
                            PileType::Foundation,
                            0,
                            card_index
                        ));
                    }
                }
            }
            PileType::Foundation => {}
            PileType::Stock => {}
            PileType::Waste => {}
        }
    }

    fn move_selection_down(&mut self) {
        let current = self.game_state.selection();

        match current.pile {
            PileType::Tableau => {
                if let Some(pile) = self.game_state.board().get_tableau_pile(current.pile_index) {
                    let pile_len = pile.len();
                    if pile_len > 0 && current.card_index < pile_len - 1 {
                        self.game_state.set_selection(Selection::new(
                            PileType::Tableau,
                            current.pile_index,
                            current.card_index + 1
                        ));
                    }
                }
            }
            PileType::Foundation => {
                if current.pile_index < 3 {
                    self.game_state.set_selection(Selection::new(
                        PileType::Tableau,
                        current.pile_index + 4,
                        0
                    ));
                } else {
                    self.game_state.set_selection(Selection::new(
                        PileType::Tableau,
                        6,
                        0
                    ));
                }
            }
            PileType::Stock => {
                self.game_state.set_selection(Selection::new(
                    PileType::Tableau,
                    0,
                    0
                ));
            }
            PileType::Waste => {
                self.game_state.set_selection(Selection::new(
                    PileType::Tableau,
                    0,
                    0
                ));
            }
        }
    }

    fn handle_select_action(&mut self) {
        if self.game_state.has_picked_up_cards() {
            let _ = self.game_state.place_cards();
        } else {
            let _ = self.game_state.pick_up_cards();
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let selection = self.game_state.selection();
        let debug_log_ref = self.debug_log.borrow();
        let game_renderer = GameRenderer::with_hover(
            self.game_state.board(),
            &selection,
            self.hover_selection.as_ref(),
            &*debug_log_ref,
            &mut self.board_renderer
        );

        frame.render_widget(game_renderer, frame.area());
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
                let _ = self.game_state.place_cards();
            } else {
                self.game_state.cancel_pickup();
            }
        }
        self.hover_selection = None;
    }
}

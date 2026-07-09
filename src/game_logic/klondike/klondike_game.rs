use crate::controller::GameAction;
use crate::game_logic::MenuManager;
use crate::game_logic::game_handler::{AppTransition, GameHandler};
use crate::game_logic::klondike::{AnimationManager, GameState, SelectionManager};
use crate::game_objects::{HoverState, PileType, Selection};
use crate::rendering::{GameRenderer, RenderingInstructions};
use crate::ui::DebugLog;
use ratatui::{buffer::Buffer, layout::Rect};

pub struct KlondikeGame {
    game_state: GameState,
    selection_manager: SelectionManager,
    animation_manager: AnimationManager,
    hover_state: HoverState,
}

impl KlondikeGame {
    pub fn new() -> Self {
        Self {
            game_state: GameState::new(),
            selection_manager: SelectionManager::new(),
            animation_manager: AnimationManager::new(),
            hover_state: HoverState::None,
        }
    }

    fn compute_valid_moves(game_state: &GameState, selection: &Selection) -> Vec<Selection> {
        let mut valid_moves = Vec::new();
        let board = game_state.board();

        let Some(card) = board.get_card_at_selection(selection) else {
            return valid_moves;
        };

        if let Some(pile) = board.get_pile(selection.pile, selection.pile_index)
            && selection.card_index == pile.len() - 1
        {
            for i in 0..4 {
                if let Some(pile) = board.get_foundation_pile(i)
                    && pile.can_place_card(card)
                {
                    valid_moves.push(Selection::new(PileType::Foundation, i, pile.len()));
                }
            }
        }

        for i in 0..7 {
            if let Some(pile) = board.get_tableau_pile(i)
                && pile.can_place_card(card)
            {
                valid_moves.push(Selection::new(PileType::Tableau, i, pile.len()));
            }
        }

        valid_moves
    }

    fn handle_select(&mut self, debug_log: &mut DebugLog) {
        if self.selection_manager.has_picked_up() {
            let source = self.selection_manager.picked_up().unwrap();
            let target = self.selection_manager.selection();
            if let Err(msg) = self.game_state.place_cards(source, target) {
                debug_log.log(msg.to_string());
                self.selection_manager.cancel_pickup();
            } else {
                self.selection_manager.place();
            }
        } else {
            let selection = self.selection_manager.selection();
            if let Err(msg) = self.game_state.pick_up_cards(selection) {
                debug_log.log(msg.to_string());
            } else {
                self.selection_manager.pick_up();
                let valid_moves = Self::compute_valid_moves(&self.game_state, &selection);
                if !valid_moves.is_empty() {
                    self.selection_manager.set_selection(valid_moves[0]);
                }
            }
        }
    }

    fn handle_click(&mut self, debug_log: &mut DebugLog) {
        let selection = self.selection_manager.selection();
        if selection.pile == PileType::Stock && !self.selection_manager.has_picked_up() {
            if let Err(msg) = self.game_state.draw_from_stock() {
                debug_log.log(msg);
            }
            return;
        }

        let moves = Self::compute_valid_moves(&self.game_state, &selection);

        if let Some(first_move) = moves.first() {
            if let Err(msg) = self.game_state.pick_up_cards(selection) {
                debug_log.log(msg.to_string());
            } else {
                self.selection_manager.pick_up();
                self.selection_manager.set_selection(*first_move);
                let source = self.selection_manager.picked_up().unwrap();
                let target = self.selection_manager.selection();
                if let Err(msg) = self.game_state.place_cards(source, target) {
                    debug_log.log(msg.to_string());
                    self.selection_manager.cancel_pickup();
                } else {
                    self.selection_manager.place();
                }
            }
        }
        self.selection_manager.set_visible(false);
    }
}

impl GameHandler for KlondikeGame {
    fn handle_action(
        &mut self,
        action: GameAction,
        debug_log: &mut DebugLog,
    ) -> Option<AppTransition> {
        match action {
            GameAction::Quit => return Some(AppTransition::Quit),
            GameAction::MoveLeft => {
                let valid_moves = self
                    .selection_manager
                    .picked_up()
                    .map(|pu| Self::compute_valid_moves(&self.game_state, &pu));
                self.selection_manager
                    .move_left(self.game_state.board(), valid_moves.as_ref());
            }
            GameAction::MoveRight => {
                let valid_moves = self
                    .selection_manager
                    .picked_up()
                    .map(|pu| Self::compute_valid_moves(&self.game_state, &pu));
                self.selection_manager
                    .move_right(self.game_state.board(), valid_moves.as_ref());
            }
            GameAction::MoveUp => {
                let valid_moves = self
                    .selection_manager
                    .picked_up()
                    .map(|pu| Self::compute_valid_moves(&self.game_state, &pu));
                self.selection_manager
                    .move_up(self.game_state.board(), valid_moves.as_ref());
            }
            GameAction::MoveDown => {
                let valid_moves = self
                    .selection_manager
                    .picked_up()
                    .map(|pu| Self::compute_valid_moves(&self.game_state, &pu));
                self.selection_manager
                    .move_down(self.game_state.board(), valid_moves.as_ref());
            }
            GameAction::Select | GameAction::Enter => self.handle_select(debug_log),
            GameAction::Cancel => self.selection_manager.cancel_pickup(),
            GameAction::DrawStock => {
                let _ = self.game_state.draw_from_stock();
            }
            GameAction::Undo => {}
            GameAction::Restart => return Some(AppTransition::Restart),
            GameAction::OptionsMenu => return Some(AppTransition::OpenSettings),
            GameAction::Click => self.handle_click(debug_log),
            GameAction::CancelDrag => {
                self.selection_manager.cancel_pickup();
                self.hover_state = HoverState::None;
            }
            GameAction::HelpMenu => return Some(AppTransition::ShowHelp),
            _ => {}
        }
        None
    }

    fn handle_mouse_action(
        &mut self,
        action: GameAction,
        renderer: &GameRenderer,
        debug_log: &mut DebugLog,
    ) {
        match action {
            GameAction::LeftMousePress(x, y) => {
                if let Some(selection) =
                    renderer.coordinate_to_selection(self.game_state.board(), x, y)
                {
                    self.selection_manager.set_selection(selection);
                    self.selection_manager.set_visible(true);
                }
            }
            GameAction::StartDrag(x, y) => {
                if let Some(selection) =
                    renderer.coordinate_to_selection(self.game_state.board(), x, y)
                {
                    self.selection_manager.set_selection(selection);
                    if let Err(msg) = self.game_state.pick_up_cards(selection) {
                        debug_log.log(msg.to_string());
                    } else {
                        self.selection_manager.pick_up();
                    }
                }
            }
            GameAction::UpdateDrag(x, y) => {
                if self.selection_manager.has_picked_up() {
                    if let Some(target) =
                        renderer.coordinate_to_selection(self.game_state.board(), x, y)
                    {
                        let Some(source) = self.selection_manager.picked_up() else {
                            debug_log.log("Error: selection not found for dragging");
                            return;
                        };
                        let is_valid = self.game_state.is_valid_placement(source, target);
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
            GameAction::CompleteDrag(x, y) => {
                if let Some(target) =
                    renderer.coordinate_to_selection(self.game_state.board(), x, y)
                    && self.selection_manager.has_picked_up()
                {
                    self.selection_manager.set_selection(target);
                    let Some(source) = self.selection_manager.picked_up() else {
                        debug_log.log("Could not find picked up card for drag");
                        self.hover_state = HoverState::None;
                        return;
                    };
                    if let Err(val) = self.game_state.place_cards(source, target) {
                        debug_log.log(val.to_string());
                        self.selection_manager.cancel_pickup();
                    } else {
                        self.selection_manager.place();
                    }
                    self.selection_manager.set_visible(false);
                }

                if self.selection_manager.has_picked_up() {
                    self.selection_manager.cancel_pickup();
                }
                self.hover_state = HoverState::None;
            }
            _ => {}
        }
    }

    fn update(&mut self, menu_manager: &mut MenuManager, debug_log: &mut DebugLog) {
        self.animation_manager
            .process_game_state(&mut self.game_state, debug_log);
        if self.game_state.has_won() {
            menu_manager.show_victory_screen();
        }
    }

    fn draw(
        &self,
        area: Rect,
        buf: &mut Buffer,
        renderer: &mut GameRenderer,
        debug_log: &DebugLog,
    ) {
        let selection = self.selection_manager.selection_if_visible();
        let picked_up = self.selection_manager.picked_up();
        let rendering_instr = RenderingInstructions::new(
            "=== Klondike ===",
            self.game_state.board(),
            selection,
            picked_up.as_ref(),
            &self.hover_state,
            debug_log,
        );
        renderer.render(area, buf, &rendering_instr);
    }

    fn restart(&self) -> Box<dyn GameHandler> {
        Box::new(KlondikeGame::new())
    }

    fn set_keyboard_mode(&mut self, enabled: bool) {
        self.selection_manager.set_visible(enabled);
    }
}

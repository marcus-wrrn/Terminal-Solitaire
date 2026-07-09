use crate::game_objects::{Board, PileType, Selection};

pub struct SelectionManager {
    selection: Selection,
    picked_up: Option<Selection>,
    visible: bool,
}

impl SelectionManager {
    pub fn new() -> Self {
        Self {
            selection: Selection::new(PileType::Tableau, 0, 0),
            picked_up: None,
            visible: false,
        }
    }

    pub fn selection(&self) -> Selection {
        self.selection
    }

    pub fn selection_if_visible(&self) -> Option<&Selection> {
        if self.visible {
            return Some(&self.selection);
        }
        None
    }

    pub fn set_selection(&mut self, selection: Selection) {
        self.selection = selection;
    }

    pub fn picked_up(&self) -> Option<Selection> {
        self.picked_up
    }

    pub fn has_picked_up(&self) -> bool {
        self.picked_up.is_some()
    }

    pub fn pick_up(&mut self) {
        self.picked_up = Some(self.selection);
    }

    pub fn place(&mut self) {
        self.picked_up = None;
    }

    pub fn cancel_pickup(&mut self) {
        self.picked_up = None;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    // Navigation methods for keyboard control

    pub fn move_left(&mut self, board: &Board, valid_moves: Option<&Vec<Selection>>) {
        if let Some(moves) = valid_moves
            && !moves.is_empty()
        {
            self.cycle_to_previous_valid_move(moves);
            return;
        }

        self.selection = match self.selection.pile {
            PileType::Tableau => {
                if self.selection.pile_index > 0 {
                    let next_pile_index = self.selection.pile_index - 1;
                    let card_index = Self::get_face_up_card_at_index(
                        board,
                        PileType::Tableau,
                        next_pile_index,
                        self.selection.card_index,
                    );
                    Selection::new(PileType::Tableau, next_pile_index, card_index)
                } else {
                    self.selection
                }
            }
            PileType::Foundation => {
                if self.selection.pile_index > 0 {
                    let next_pile_index = self.selection.pile_index - 1;
                    let card_index = Self::get_topmost_face_up_card_index(
                        board,
                        PileType::Foundation,
                        next_pile_index,
                    );
                    Selection::new(PileType::Foundation, next_pile_index, card_index)
                } else {
                    let card_index =
                        Self::get_topmost_face_up_card_index(board, PileType::Waste, 0);
                    Selection::new(PileType::Waste, 0, card_index)
                }
            }
            PileType::Waste => {
                let card_index = Self::get_topmost_face_up_card_index(board, PileType::Stock, 0);
                Selection::new(PileType::Stock, 0, card_index)
            }
            PileType::Stock => self.selection,
        };
    }

    pub fn move_right(&mut self, board: &Board, valid_moves: Option<&Vec<Selection>>) {
        if let Some(moves) = valid_moves
            && !moves.is_empty()
        {
            self.cycle_to_next_valid_move(moves);
            return;
        }

        self.selection = match self.selection.pile {
            PileType::Tableau => {
                if self.selection.pile_index < 6 {
                    let next_pile_index = self.selection.pile_index + 1;
                    let card_index = Self::get_face_up_card_at_index(
                        board,
                        PileType::Tableau,
                        next_pile_index,
                        self.selection.card_index,
                    );
                    Selection::new(PileType::Tableau, next_pile_index, card_index)
                } else {
                    self.selection
                }
            }
            PileType::Foundation => {
                if self.selection.pile_index < 3 {
                    let card_index = Self::get_topmost_face_up_card_index(
                        board,
                        PileType::Foundation,
                        self.selection.pile_index + 1,
                    );
                    Selection::new(
                        PileType::Foundation,
                        self.selection.pile_index + 1,
                        card_index,
                    )
                } else {
                    self.selection
                }
            }
            PileType::Stock => {
                let card_index = Self::get_topmost_face_up_card_index(board, PileType::Waste, 0);
                Selection::new(PileType::Waste, 0, card_index)
            }
            PileType::Waste => {
                let card_index =
                    Self::get_topmost_face_up_card_index(board, PileType::Foundation, 0);
                Selection::new(PileType::Foundation, 0, card_index)
            }
        };
    }

    /// Helper function to move selection to topmost piles
    fn move_to_upward_piles(board: &Board, pile_index: usize) -> Selection {
        if pile_index < 4 {
            let card_index = Self::get_topmost_face_up_card_index(board, PileType::Stock, 0);
            Selection::new(PileType::Stock, 0, card_index)
        } else {
            let card_index = Self::get_topmost_face_up_card_index(board, PileType::Foundation, 0);
            Selection::new(PileType::Foundation, 0, card_index)
        }
    }

    pub fn move_up(&mut self, board: &Board, valid_moves: Option<&Vec<Selection>>) {
        if let Some(moves) = valid_moves
            && !moves.is_empty()
        {
            self.cycle_to_previous_valid_move(moves);
            return;
        }

        self.selection = match self.selection.pile {
            PileType::Tableau => {
                if self.selection.card_index > 0
                    && let Some(pile) = board.get_tableau_pile(self.selection.pile_index)
                {
                    let target_index = self.selection.card_index - 1;
                    if pile.cards.get(target_index).is_some_and(|c| c.face_up) {
                        Selection::new(PileType::Tableau, self.selection.pile_index, target_index)
                    } else {
                        Self::move_to_upward_piles(board, self.selection.pile_index)
                    }
                } else {
                    Self::move_to_upward_piles(board, self.selection.pile_index)
                }
            }
            PileType::Foundation => self.selection,
            PileType::Stock => self.selection,
            PileType::Waste => self.selection,
        };
    }

    pub fn move_down(&mut self, board: &Board, valid_moves: Option<&Vec<Selection>>) {
        if let Some(moves) = valid_moves
            && !moves.is_empty()
        {
            self.cycle_to_next_valid_move(moves);
            return;
        }

        self.selection = match self.selection.pile {
            PileType::Tableau => {
                if let Some(pile) = board.get_tableau_pile(self.selection.pile_index) {
                    let pile_len = pile.len();
                    if pile_len > 0 && self.selection.card_index < pile_len - 1 {
                        let target_index = self.selection.card_index + 1;
                        if pile.cards.get(target_index).is_some_and(|c| c.face_up) {
                            Selection::new(
                                PileType::Tableau,
                                self.selection.pile_index,
                                target_index,
                            )
                        } else {
                            self.selection
                        }
                    } else {
                        self.selection
                    }
                } else {
                    self.selection
                }
            }
            PileType::Foundation => {
                let tableau_index = if self.selection.pile_index < 3 {
                    self.selection.pile_index + 4
                } else {
                    6
                };
                let card_index =
                    Self::get_topmost_face_up_card_index(board, PileType::Tableau, tableau_index);
                Selection::new(PileType::Tableau, tableau_index, card_index)
            }
            PileType::Stock => {
                let card_index = Self::get_topmost_face_up_card_index(board, PileType::Tableau, 0);
                Selection::new(PileType::Tableau, 0, card_index)
            }
            PileType::Waste => {
                let card_index = Self::get_topmost_face_up_card_index(board, PileType::Tableau, 0);
                Selection::new(PileType::Tableau, 0, card_index)
            }
        };
    }

    fn cycle_to_next_valid_move(&mut self, valid_moves: &[Selection]) {
        if let Some(current_index) = valid_moves.iter().position(|&s| s == self.selection) {
            let next_index = (current_index + 1) % valid_moves.len();
            self.selection = valid_moves[next_index];
        } else {
            self.selection = valid_moves[0];
        }
    }

    fn cycle_to_previous_valid_move(&mut self, valid_moves: &[Selection]) {
        if let Some(current_index) = valid_moves.iter().position(|&s| s == self.selection) {
            let prev_index = if current_index == 0 {
                valid_moves.len() - 1
            } else {
                current_index - 1
            };
            self.selection = valid_moves[prev_index];
        } else {
            self.selection = valid_moves[valid_moves.len() - 1];
        }
    }

    fn get_topmost_face_up_card_index(
        board: &Board,
        pile_type: PileType,
        pile_index: usize,
    ) -> usize {
        match pile_type {
            PileType::Tableau => board
                .get_tableau_pile(pile_index)
                .and_then(|p| {
                    p.cards
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(_, card)| card.face_up)
                        .map(|(idx, _)| idx)
                })
                .unwrap_or(0),
            PileType::Foundation => board
                .get_foundation_pile(pile_index)
                .map(|p| if !p.is_empty() { p.len() - 1 } else { 0 })
                .unwrap_or(0),
            PileType::Waste => {
                let len = board.waste.len();
                if len > 0 { len - 1 } else { 0 }
            }
            PileType::Stock => {
                let len = board.stock.len();
                if len > 0 { len - 1 } else { 0 }
            }
        }
    }

    fn get_face_up_card_at_index(
        board: &Board,
        pile_type: PileType,
        pile_index: usize,
        target_index: usize,
    ) -> usize {
        match pile_type {
            PileType::Tableau => {
                if let Some(pile) = board.get_tableau_pile(pile_index) {
                    if target_index < pile.len() && pile.cards[target_index].face_up {
                        return target_index;
                    }

                    let max_index = target_index.min(pile.len());
                    for i in (0..max_index).rev() {
                        if pile.cards[i].face_up {
                            return i;
                        }
                    }

                    Self::get_topmost_face_up_card_index(board, pile_type, pile_index)
                } else {
                    0
                }
            }
            _ => Self::get_topmost_face_up_card_index(board, pile_type, pile_index),
        }
    }
}

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
            visible: true,
        }
    }

    pub fn selection(&self) -> Selection {
        self.selection
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

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    // Navigation methods for keyboard control

    pub fn move_left(&mut self, board: &Board) {
        self.selection = match self.selection.pile {
            PileType::Tableau => {
                if self.selection.pile_index > 0 {
                    let next_pile_index = self.selection.pile_index - 1;
                    let topmost = Self::get_topmost_card_index(board, PileType::Tableau, next_pile_index);
                    let card_index = if self.selection.card_index <= topmost {
                        self.selection.card_index
                    } else {
                        topmost
                    };
                    Selection::new(PileType::Tableau, next_pile_index, card_index)
                } else {
                    self.selection
                }
            }
            PileType::Foundation => {
                if self.selection.pile_index > 0 {
                    let next_pile_index = self.selection.pile_index - 1;
                    let card_index = Self::get_topmost_card_index(board, PileType::Foundation, next_pile_index);
                    Selection::new(PileType::Foundation, next_pile_index, card_index)
                } else {
                    let card_index = Self::get_topmost_card_index(board, PileType::Waste, 0);
                    Selection::new(PileType::Waste, 0, card_index)
                }
            }
            PileType::Waste => {
                let card_index = Self::get_topmost_card_index(board, PileType::Stock, 0);
                Selection::new(PileType::Stock, 0, card_index)
            }
            PileType::Stock => self.selection,
        };
    }

    pub fn move_right(&mut self, board: &Board) {
        self.selection = match self.selection.pile {
            PileType::Tableau => {
                if self.selection.pile_index < 6 {
                    let next_pile_index = self.selection.pile_index + 1;
                    let topmost = Self::get_topmost_card_index(board, PileType::Tableau, next_pile_index);
                    let card_index = if self.selection.card_index <= topmost {
                        self.selection.card_index
                    } else {
                        topmost
                    };
                    Selection::new(PileType::Tableau, next_pile_index, card_index)
                } else {
                    self.selection
                }
            }
            PileType::Foundation => {
                if self.selection.pile_index < 3 {
                    let card_index = Self::get_topmost_card_index(board, PileType::Foundation, self.selection.pile_index + 1);
                    Selection::new(PileType::Foundation, self.selection.pile_index + 1, card_index)
                } else {
                    self.selection
                }
            }
            PileType::Stock => {
                let card_index = Self::get_topmost_card_index(board, PileType::Waste, 0);
                Selection::new(PileType::Waste, 0, card_index)
            }
            PileType::Waste => {
                let card_index = Self::get_topmost_card_index(board, PileType::Foundation, 0);
                Selection::new(PileType::Foundation, 0, card_index)
            }
        };
    }

    pub fn move_up(&mut self, board: &Board) {
        self.selection = match self.selection.pile {
            PileType::Tableau => {
                if self.selection.card_index > 0 {
                    Selection::new(PileType::Tableau, self.selection.pile_index, self.selection.card_index - 1)
                } else {
                    if self.selection.pile_index < 4 {
                        let card_index = Self::get_topmost_card_index(board, PileType::Stock, 0);
                        Selection::new(PileType::Stock, 0, card_index)
                    } else {
                        let card_index = Self::get_topmost_card_index(board, PileType::Foundation, 0);
                        Selection::new(PileType::Foundation, 0, card_index)
                    }
                }
            }
            PileType::Foundation => self.selection,
            PileType::Stock => self.selection,
            PileType::Waste => self.selection,
        };
    }

    pub fn move_down(&mut self, board: &Board) {
        self.selection = match self.selection.pile {
            PileType::Tableau => {
                if let Some(pile) = board.get_tableau_pile(self.selection.pile_index) {
                    let pile_len = pile.len();
                    if pile_len > 0 && self.selection.card_index < pile_len - 1 {
                        Selection::new(PileType::Tableau, self.selection.pile_index, self.selection.card_index + 1)
                    } else {
                        self.selection
                    }
                } else {
                    self.selection
                }
            }
            PileType::Foundation => {
                if self.selection.pile_index < 3 {
                    Selection::new(PileType::Tableau, self.selection.pile_index + 4, 0)
                } else {
                    Selection::new(PileType::Tableau, 6, 0)
                }
            }
            PileType::Stock => {
                Selection::new(PileType::Tableau, 0, 0)
            }
            PileType::Waste => {
                Selection::new(PileType::Tableau, 0, 0)
            }
        };
    }

    fn get_topmost_card_index(board: &Board, pile_type: PileType, pile_index: usize) -> usize {
        match pile_type {
            PileType::Tableau => {
                board.get_tableau_pile(pile_index)
                    .map(|p| if p.len() > 0 { p.len() - 1 } else { 0 })
                    .unwrap_or(0)
            }
            PileType::Foundation => {
                board.get_foundation_pile(pile_index)
                    .map(|p| if p.len() > 0 { p.len() - 1 } else { 0 })
                    .unwrap_or(0)
            }
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
}

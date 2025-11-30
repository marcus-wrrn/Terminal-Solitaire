use crate::game_objects::{Board, PileType, Selection};

pub struct SelectionNavigator;

/// Handles the movement of the selection cursor using keyboard controls
impl SelectionNavigator {
    pub fn move_left(board: &Board, current: Selection) -> Selection {
        match current.pile {
            PileType::Tableau => {
                if current.pile_index > 0 {
                    let next_pile_index = current.pile_index - 1;
                    let topmost = Self::get_topmost_card_index(board, PileType::Tableau, next_pile_index);
                    let card_index = if current.card_index <= topmost {
                        current.card_index
                    } else {
                        topmost
                    };
                    Selection::new(PileType::Tableau, next_pile_index, card_index)
                } else {
                    current
                }
            }
            PileType::Foundation => {
                if current.pile_index > 0 {
                    let next_pile_index = current.pile_index - 1;
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
            PileType::Stock => current,
        }
    }

    pub fn move_right(board: &Board, current: Selection) -> Selection {
        match current.pile {
            PileType::Tableau => {
                if current.pile_index < 6 {
                    let next_pile_index = current.pile_index + 1;
                    let topmost = Self::get_topmost_card_index(board, PileType::Tableau, next_pile_index);
                    let card_index = if current.card_index <= topmost {
                        current.card_index
                    } else {
                        topmost
                    };
                    Selection::new(PileType::Tableau, next_pile_index, card_index)
                } else {
                    current
                }
            }
            PileType::Foundation => {
                if current.pile_index < 3 {
                    let card_index = Self::get_topmost_card_index(board, PileType::Foundation, current.pile_index + 1);
                    Selection::new(PileType::Foundation, current.pile_index + 1, card_index)
                } else {
                    current
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
        }
    }

    pub fn move_up(board: &Board, current: Selection) -> Selection {
        match current.pile {
            PileType::Tableau => {
                if current.card_index > 0 {
                    Selection::new(PileType::Tableau, current.pile_index, current.card_index - 1)
                } else {
                    if current.pile_index < 4 {
                        let card_index = Self::get_topmost_card_index(board, PileType::Stock, 0);
                        Selection::new(PileType::Stock, 0, card_index)
                    } else {
                        let card_index = Self::get_topmost_card_index(board, PileType::Foundation, 0);
                        Selection::new(PileType::Foundation, 0, card_index)
                    }
                }
            }
            PileType::Foundation => current,
            PileType::Stock => current,
            PileType::Waste => current,
        }
    }

    pub fn move_down(board: &Board, current: Selection) -> Selection {
        match current.pile {
            PileType::Tableau => {
                if let Some(pile) = board.get_tableau_pile(current.pile_index) {
                    let pile_len = pile.len();
                    if pile_len > 0 && current.card_index < pile_len - 1 {
                        return Selection::new(PileType::Tableau, current.pile_index, current.card_index + 1);
                    }
                }
                current
            }
            PileType::Foundation => {
                if current.pile_index < 3 {
                    Selection::new(PileType::Tableau, current.pile_index + 4, 0)
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
        }
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

use super::PileType;

/// Represents the currently selected position on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    pub pile: PileType,
    pub pile_index: usize,
    pub card_index: usize,
}

impl Selection {
    pub fn new(pile: PileType, pile_index: usize, card_index: usize) -> Self {
        Self { pile, pile_index, card_index }
    }
}

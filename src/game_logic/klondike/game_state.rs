use crate::game_objects::{Board, Deck, PileType, Selection};
use crate::game_logic::klondike::MoveExecutor;

pub struct GameState {
    board: Board,
}

impl GameState {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();

        let mut board = Board::new();
        board.setup(&mut deck);

        Self {
            board,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn pick_up_cards(&mut self, selection: Selection) -> Result<(), &'static str> {
        let current_selection = selection;

        match current_selection.pile {
            PileType::Tableau => {
                if let Some(pile) = self.board.get_tableau_pile(current_selection.pile_index) {
                    if current_selection.card_index >= pile.len() {
                        return Err("No card at this position");
                    }
                    if let Some(card) = pile.cards.get(current_selection.card_index) {
                        if !card.face_up {
                            return Err("Cannot pick up face-down card");
                        }
                    }
                    Ok(())
                } else {
                    Err("Invalid pile")
                }
            }
            PileType::Waste => {
                if self.board.waste.is_empty() {
                    return Err("No card in waste pile");
                }
                Ok(())
            }
            PileType::Foundation => {
                if let Some(pile) = self.board.get_foundation_pile(current_selection.pile_index) {
                    if pile.is_empty() {
                        return Err("No card in foundation pile");
                    }
                    Ok(())
                } else {
                    Err("Invalid pile")
                }
            }
            PileType::Stock => {
                // Stock can be clicked but not picked up
                Ok(())
            }
        }
    }

    pub fn place_cards(&mut self, source: Selection, target: Selection) -> Result<(), &'static str> {
        if source == target {
            return Ok(());
        }

        match (source.pile, target.pile) {
            (PileType::Tableau, PileType::Tableau) => {
                MoveExecutor::move_card_to_tableau(
                    &mut self.board,
                    source.pile_index,
                    target.pile_index,
                    source.card_index
                )?;
                Ok(())
            }
            (PileType::Tableau, PileType::Foundation) => {
                if let Some(pile) = self.board.get_tableau_pile(source.pile_index) {
                    if source.card_index != pile.len() - 1 {
                        return Err("Can only move top card to foundation");
                    }
                }
                MoveExecutor::move_card_to_foundation(&mut self.board, source.pile_index, target.pile_index)?;
                Ok(())
            }
            (PileType::Waste, PileType::Tableau) => {
                MoveExecutor::move_waste_to_tableau(&mut self.board, target.pile_index)?;
                Ok(())
            }
            (PileType::Waste, PileType::Foundation) => {
                MoveExecutor::move_waste_to_foundation(&mut self.board, target.pile_index)?;
                Ok(())
            }
            (PileType::Foundation, PileType::Tableau) => {
                if let Some(pile) = self.board.get_foundation_pile(source.pile_index) {
                    if source.card_index != pile.len() - 1 {
                        return Err("Can only move top card from foundation");
                    }
                }
                MoveExecutor::move_foundation_to_tableau(&mut self.board, source.pile_index, target.pile_index)?;
                Ok(())
            }
            _ => {
                Err("Invalid move")
            }
        }
    }

    pub fn draw_from_stock(&mut self) -> Result<(), &'static str> {
        if MoveExecutor::draw_from_stock(&mut self.board) {
            Ok(())
        } else {
            if self.board.waste.is_empty() {
                Err("Stock is empty")
            } else {
                MoveExecutor::reset_stock(&mut self.board);
                Ok(())
            }
        }
    }

    pub fn has_won(&self) -> bool {
        self.board.foundation.iter().all(|pile| pile.len() == 13)
    }

    pub fn all_tableau_cards_face_up(&self) -> bool {
        self.board.tableau.iter().all(|pile| {
            pile.cards.iter().all(|card| card.face_up)
        })
    }

    pub fn is_valid_placement(&self, source: Selection, target: Selection) -> bool {
        if source == target {
            return true;
        }

        match (source.pile, target.pile) {
            (PileType::Tableau, PileType::Tableau) => {
                if let Some(source_pile) = self.board.get_tableau_pile(source.pile_index) {
                    if source.card_index >= source_pile.len() {
                        return false;
                    }
                    if let Some(first_card) = source_pile.cards.get(source.card_index) {
                        if let Some(target_pile) = self.board.get_tableau_pile(target.pile_index) {
                            return target_pile.can_place_card(first_card);
                        }
                    }
                }
                false
            }
            (PileType::Tableau, PileType::Foundation) => {
                if let Some(pile) = self.board.get_tableau_pile(source.pile_index) {
                    if source.card_index != pile.len() - 1 {
                        return false;
                    }
                    if let Some(card) = pile.peek() {
                        if let Some(foundation) = self.board.get_foundation_pile(target.pile_index) {
                            return foundation.can_place_card(card);
                        }
                    }
                }
                false
            }
            (PileType::Waste, PileType::Tableau) => {
                if let Some(card) = self.board.waste.peek() {
                    if let Some(pile) = self.board.get_tableau_pile(target.pile_index) {
                        return pile.can_place_card(card);
                    }
                }
                false
            }
            (PileType::Waste, PileType::Foundation) => {
                if let Some(card) = self.board.waste.peek() {
                    if let Some(pile) = self.board.get_foundation_pile(target.pile_index) {
                        return pile.can_place_card(card);
                    }
                }
                false
            }
            (PileType::Foundation, PileType::Tableau) => {
                if let Some(pile) = self.board.get_foundation_pile(source.pile_index) {
                    if source.card_index != pile.len() - 1 {
                        return false;
                    }
                    if let Some(card) = pile.peek() {
                        if let Some(tableau) = self.board.get_tableau_pile(target.pile_index) {
                            return tableau.can_place_card(card);
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }
}

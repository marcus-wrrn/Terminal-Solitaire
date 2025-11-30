use crate::game_objects::{Board, Deck, PileType, Selection};

pub struct GameState {
    board: Board,
    selection: Selection,
    picked_up: Option<Selection>,
}

impl GameState {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();

        let mut board = Board::new();
        board.setup(&mut deck);

        Self {
            board,
            selection: Selection::new(PileType::Tableau, 0, 0),
            picked_up: None,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn selection(&self) -> Selection {
        self.selection
    }

    pub fn set_selection(&mut self, selection: Selection) {
        self.selection = selection;
    }

    pub fn pick_up_cards(&mut self) -> Result<(), &'static str> {
        if self.picked_up.is_some() {
            return Err("Already holding cards");
        }

        let current_selection = self.selection;

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
                    self.picked_up = Some(current_selection);
                    Ok(())
                } else {
                    Err("Invalid pile")
                }
            }
            PileType::Waste => {
                if self.board.waste.is_empty() {
                    return Err("No card in waste pile");
                }
                self.picked_up = Some(current_selection);
                Ok(())
            }
            PileType::Foundation => {
                if let Some(pile) = self.board.get_foundation_pile(current_selection.pile_index) {
                    if pile.is_empty() {
                        return Err("No card in foundation pile");
                    }
                    self.picked_up = Some(current_selection);
                    Ok(())
                } else {
                    Err("Invalid pile")
                }
            }
            PileType::Stock => {
                Err("Cannot pick up cards from stock")
            }
        }
    }

    pub fn place_cards(&mut self) -> Result<(), &'static str> {
        let source = self.picked_up.ok_or("No cards picked up")?;
        let target = self.selection;

        if source == target {
            self.picked_up = None;
            return Ok(());
        }

        match (source.pile, target.pile) {
            (PileType::Tableau, PileType::Tableau) => {
                self.board.move_card_to_tableau(
                    source.pile_index,
                    target.pile_index,
                    source.card_index
                )?;
                self.picked_up = None;
                Ok(())
            }
            (PileType::Tableau, PileType::Foundation) => {
                if let Some(pile) = self.board.get_tableau_pile(source.pile_index) {
                    if source.card_index != pile.len() - 1 {
                        return Err("Can only move top card to foundation");
                    }
                }
                self.board.move_card_to_foundation(source.pile_index, target.pile_index)?;
                self.picked_up = None;
                Ok(())
            }
            (PileType::Waste, PileType::Tableau) => {
                self.board.move_waste_to_tableau(target.pile_index)?;
                self.picked_up = None;
                Ok(())
            }
            (PileType::Waste, PileType::Foundation) => {
                self.board.move_waste_to_foundation(target.pile_index)?;
                self.picked_up = None;
                Ok(())
            }
            (PileType::Foundation, PileType::Tableau) => {
                Err("Moving from foundation to tableau not yet implemented")
            }
            _ => {
                Err("Invalid move")
            }
        }
    }

    pub fn cancel_pickup(&mut self) {
        self.picked_up = None;
    }

    pub fn has_picked_up_cards(&self) -> bool {
        self.picked_up.is_some()
    }

    pub fn draw_from_stock(&mut self) -> Result<(), &'static str> {
        if self.board.draw_from_stock() {
            Ok(())
        } else {
            if self.board.waste.is_empty() {
                Err("Stock is empty")
            } else {
                self.board.reset_stock();
                Ok(())
            }
        }
    }
}

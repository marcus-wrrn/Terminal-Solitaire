use crate::game_objects::{Board, Deck, PileType, Selection};
use crate::ui::DebugLog;
use std::rc::Rc;
use std::cell::RefCell;

pub struct GameState {
    board: Board,
    selection: Selection,
    picked_up: Option<Selection>,
    debug_log: Rc<RefCell<DebugLog>>
}

impl GameState {
    pub fn new(debug_log: Rc<RefCell<DebugLog>>) -> Self {
        let mut deck = Deck::new();
        deck.shuffle();

        let mut board = Board::new();
        board.setup(&mut deck);

        let mut state = Self {
            board,
            selection: Selection::new(PileType::Tableau, 0, 0),
            picked_up: None,
            debug_log
        };

        state.select_card(state.selection);
        state
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn selection(&self) -> Selection {
        self.selection
    }

    pub fn set_selection(&mut self, selection: Selection) {
        let old_selection = self.selection;
        self.debug_log.borrow_mut().log(&format!(
            "Selection changed from {:?}[{}][{}] to {:?}[{}][{}]",
            old_selection.pile, old_selection.pile_index, old_selection.card_index,
            selection.pile, selection.pile_index, selection.card_index
        ));
        self.selection = selection;
        self.deselect_card(old_selection);
        self.select_card(selection);
    }

    fn deselect_card(&mut self, selection: Selection) {
        match selection.pile {
            PileType::Tableau => {
                if let Some(pile) = self.board.tableau.get_mut(selection.pile_index) {
                    if let Some(card) = pile.cards.get_mut(selection.card_index) {
                        card.is_selected = false;
                    }
                }
            }
            PileType::Foundation => {
                if let Some(pile) = self.board.foundation.get_mut(selection.pile_index) {
                    if let Some(card) = pile.cards.get_mut(selection.card_index) {
                        card.is_selected = false;
                    }
                }
            }
            PileType::Waste => {
                if let Some(card) = self.board.waste.cards.get_mut(selection.card_index) {
                    card.is_selected = false;
                }
            }
            PileType::Stock => {
                if let Some(card) = self.board.stock.cards.get_mut(selection.card_index) {
                    card.is_selected = false;
                }
            }
        }
    }

    fn select_card(&mut self, selection: Selection) {
        match selection.pile {
            PileType::Tableau => {
                if let Some(pile) = self.board.tableau.get_mut(selection.pile_index) {
                    if let Some(card) = pile.cards.get_mut(selection.card_index) {
                        card.is_selected = true;
                    }
                }
            }
            PileType::Foundation => {
                if let Some(pile) = self.board.foundation.get_mut(selection.pile_index) {
                    if let Some(card) = pile.cards.get_mut(selection.card_index) {
                        card.is_selected = true;
                    }
                }
            }
            PileType::Waste => {
                if let Some(card) = self.board.waste.cards.get_mut(selection.card_index) {
                    card.is_selected = true;
                }
            }
            PileType::Stock => {
                if let Some(card) = self.board.stock.cards.get_mut(selection.card_index) {
                    card.is_selected = true;
                }
            }
        }
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

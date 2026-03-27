use crate::game_objects::{Card, Deck, Pile, PileType, Selection};

pub struct Board {
    pub tableau: [Pile; 7],
    pub foundation: [Pile; 4],
    pub stock: Pile,
    pub waste: Pile,
}

impl Board {
    pub fn new() -> Self {
        Board {
            tableau: [
                Pile::new(PileType::Tableau),
                Pile::new(PileType::Tableau),
                Pile::new(PileType::Tableau),
                Pile::new(PileType::Tableau),
                Pile::new(PileType::Tableau),
                Pile::new(PileType::Tableau),
                Pile::new(PileType::Tableau),
            ],
            foundation: [
                Pile::new(PileType::Foundation),
                Pile::new(PileType::Foundation),
                Pile::new(PileType::Foundation),
                Pile::new(PileType::Foundation),
            ],
            stock: Pile::new(PileType::Stock),
            waste: Pile::new(PileType::Waste),
        }
    }

    pub fn setup(&mut self, deck: &mut Deck) {
        // Deal cards to tableau piles
        for i in 0..7 {
            for j in 0..=i {
                if let Some(mut card) = deck.draw_card() {
                    if j == i {
                        card.face_up = true;
                    }
                    self.tableau[i].add_card(card);
                }
            }
        }

        // Remaining cards go to stock
        while let Some(card) = deck.draw_card() {
            self.stock.add_card(card);
        }
    }

    pub fn get_tableau_pile(&self, index: usize) -> Option<&Pile> {
        self.tableau.get(index)
    }

    pub fn get_foundation_pile(&self, index: usize) -> Option<&Pile> {
        self.foundation.get(index)
    }

    pub fn get_card_at_selection(&self, selection: &Selection) -> Option<&Card> {
        if let Some(pile) = self.get_pile(selection.pile, selection.pile_index) {
            return pile.cards.get(selection.card_index);
        }
        None
    }

    pub fn get_pile(&self, pile_type: PileType, pile_index: usize) -> Option<&Pile> {
        let pile = match pile_type {
            PileType::Tableau => self.tableau.get(pile_index)?,
            PileType::Foundation => self.foundation.get(pile_index)?,
            PileType::Stock => &self.stock,
            PileType::Waste => &self.waste
        };
        Some(pile)
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

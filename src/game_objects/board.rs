use crate::game_objects::{Deck, Pile, PileType, Card};

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
                    self.tableau[i].push(card);
                }
            }
        }

        // Remaining cards go to stock
        while let Some(card) = deck.draw_card() {
            self.stock.push(card);
        }
    }

    pub fn draw_from_stock(&mut self) -> bool {
        if let Some(mut card) = self.stock.pop() {
            card.face_up = true;
            self.waste.push(card);
            true
        } else {
            false
        }
    }

    pub fn reset_stock(&mut self) {
        while let Some(mut card) = self.waste.pop() {
            card.face_up = false;
            self.stock.push(card);
        }
    }

    pub fn get_tableau_pile(&self, index: usize) -> Option<&Pile> {
        self.tableau.get(index)
    }

    pub fn get_foundation_pile(&self, index: usize) -> Option<&Pile> {
        self.foundation.get(index)
    }

    pub fn move_card_to_tableau(&mut self, from_pile: usize, to_pile: usize, card_index: usize) -> Result<(), &'static str> {
        if from_pile >= 7 || to_pile >= 7 {
            return Err("Invalid pile index");
        }

        // Check if the move is valid
        let cards_to_move = if let Some(pile) = self.tableau.get(from_pile) {
            if card_index >= pile.len() {
                return Err("Invalid card index");
            }
            pile.cards[card_index..].to_vec()
        } else {
            return Err("Invalid from pile");
        };

        if let Some(first_card) = cards_to_move.first() {
            if !self.tableau[to_pile].can_place_card(first_card) {
                return Err("Cannot place card on target pile");
            }
        }

        let cards = self.tableau[from_pile].take_cards_from(card_index);
        self.tableau[to_pile].add_cards(cards);

        // Flip the top card of the from pile if it exists and is face down
        self.tableau[from_pile].flip_top_card();

        Ok(())
    }

    pub fn move_card_to_foundation(&mut self, from_tableau: usize, foundation_index: usize) -> Result<(), &'static str> {
        if from_tableau >= 7 || foundation_index >= 4 {
            return Err("Invalid pile index");
        }

        let card = self.tableau[from_tableau].peek()
            .ok_or("No card to move")?
            .clone();

        if !self.foundation[foundation_index].can_place_card(&card) {
            return Err("Cannot place card on foundation");
        }

        if let Some(card) = self.tableau[from_tableau].pop() {
            self.foundation[foundation_index].push(card);
            self.tableau[from_tableau].flip_top_card();
            Ok(())
        } else {
            Err("Failed to move card")
        }
    }

    pub fn move_waste_to_tableau(&mut self, tableau_index: usize) -> Result<(), &'static str> {
        if tableau_index >= 7 {
            return Err("Invalid tableau index");
        }

        let card = self.waste.peek()
            .ok_or("No card in waste")?
            .clone();

        if !self.tableau[tableau_index].can_place_card(&card) {
            return Err("Cannot place card on tableau");
        }

        if let Some(card) = self.waste.pop() {
            self.tableau[tableau_index].push(card);
            Ok(())
        } else {
            Err("Failed to move card")
        }
    }

    pub fn move_waste_to_foundation(&mut self, foundation_index: usize) -> Result<(), &'static str> {
        if foundation_index >= 4 {
            return Err("Invalid foundation index");
        }

        let card = self.waste.peek()
            .ok_or("No card in waste")?
            .clone();

        if !self.foundation[foundation_index].can_place_card(&card) {
            return Err("Cannot place card on foundation");
        }

        if let Some(card) = self.waste.pop() {
            self.foundation[foundation_index].push(card);
            Ok(())
        } else {
            Err("Failed to move card")
        }
    }

    pub fn get_all_cards(&self) -> Vec<&Card> {
      self.tableau.iter()
          .chain(self.foundation.iter())
          .chain(std::iter::once(&self.stock))
          .chain(std::iter::once(&self.waste))
          .flat_map(|pile| pile.cards.iter())
          .collect()
  }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

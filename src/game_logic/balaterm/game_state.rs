use crate::game_objects::{Card, CardRow, Rank, Suit};

pub struct BalatermState {
    pub hands: Vec<CardRow>,
}

impl BalatermState {
    pub fn new() -> Self {
        let mut hand = CardRow::new(5);
        for i in 0..5 {
            let mut card = Card::new(Suit::Spades, Rank::Ace);
            card.face_up = true;
            hand.place_card(i, card);
        }
        Self {
            hands: vec![hand],
        }
    }

    pub fn add_hand(&mut self, capacity: usize) {
        self.hands.push(CardRow::new(capacity));
    }

    pub fn remove_hand(&mut self) -> Option<CardRow> {
        self.hands.pop()
    }
}

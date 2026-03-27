use crate::game_objects::{Card, Hand, Rank, Suit};

pub struct BalatermState {
    pub hands: Vec<Hand>,
}

impl BalatermState {
    pub fn new() -> Self {
        let mut hand = Hand::new(5);
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
        self.hands.push(Hand::new(capacity));
    }

    pub fn remove_hand(&mut self) -> Option<Hand> {
        self.hands.pop()
    }
}

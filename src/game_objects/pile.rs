use crate::game_objects::card::{Card, Rank};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PileType {
    Tableau,
    Foundation,
    Stock,
    Waste,
}

pub struct Pile {
    pub pile_type: PileType,
    pub cards: Vec<Card>,
}

impl Pile {
    pub fn new(pile_type: PileType) -> Self {
        Pile {
            pile_type,
            cards: Vec::new(),
        }
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn peek(&self) -> Option<&Card> {
        self.cards.last()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn can_place_card(&self, card: &Card) -> bool {
        match self.pile_type {
            PileType::Tableau => self.can_place_on_tableau(card),
            PileType::Foundation => self.can_place_on_foundation(card),
            PileType::Stock | PileType::Waste => false,
        }
    }

    fn can_place_on_tableau(&self, card: &Card) -> bool {
        match self.peek() {
            None => card.rank == Rank::King,
            Some(top_card) => {
                card.is_opposite_color(top_card) && card.is_one_rank_lower(top_card)
            }
        }
    }

    fn can_place_on_foundation(&self, card: &Card) -> bool {
        match self.peek() {
            None => card.rank == Rank::Ace,
            Some(top_card) => {
                card.suit == top_card.suit && card.rank.value() == top_card.rank.value() + 1
            }
        }
    }

    pub fn take_cards_from(&mut self, index: usize) -> Vec<Card> {
        if index >= self.cards.len() {
            return Vec::new();
        }
        self.cards.split_off(index)
    }

    pub fn add_cards(&mut self, mut cards: Vec<Card>) {
        self.cards.append(&mut cards);
    }

    pub fn flip_top_card(&mut self) {
        if let Some(card) = self.cards.last_mut() {
            card.face_up = true;
        }
    }
}
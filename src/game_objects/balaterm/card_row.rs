use crate::game_objects::Card;

pub struct CardSlot {
    pub card: Option<Card>,
}

impl CardSlot {
    pub fn new() -> Self {
        Self { card: None }
    }

    pub fn is_empty(&self) -> bool {
        self.card.is_none()
    }

    pub fn place(&mut self, card: Card) -> bool {
        if self.card.is_some() {
            return false;
        }
        self.card = Some(card);
        true
    }

    pub fn take(&mut self) -> Option<Card> {
        self.card.take()
    }
}

/// A horizontal row of card slots. Each slot holds at most one card.
/// The number of slots can grow or shrink dynamically.
pub struct CardRow {
    pub slots: Vec<CardSlot>,
}

impl CardRow {
    pub fn new(capacity: usize) -> Self {
        Self {
            slots: (0..capacity).map(|_| CardSlot::new()).collect(),
        }
    }

    pub fn capacity(&self) -> usize {
        self.slots.len()
    }

    pub fn add_slot(&mut self) {
        self.slots.push(CardSlot::new());
    }

    /// Remove the last slot. Returns any card that was in the removed slot.
    pub fn remove_slot(&mut self) -> Option<Card> {
        self.slots.pop().and_then(|mut s| s.take())
    }

    pub fn place_card(&mut self, index: usize, card: Card) -> bool {
        match self.slots.get_mut(index) {
            Some(slot) => slot.place(card),
            None => false,
        }
    }

    pub fn take_card(&mut self, index: usize) -> Option<Card> {
        self.slots.get_mut(index)?.take()
    }

    pub fn is_full(&self) -> bool {
        self.slots.iter().all(|s| !s.is_empty())
    }

    pub fn is_empty(&self) -> bool {
        self.slots.iter().all(|s| s.is_empty())
    }

    /// Returns the index of the first empty slot, if any.
    pub fn first_empty_slot(&self) -> Option<usize> {
        self.slots.iter().position(|s| s.is_empty())
    }
}

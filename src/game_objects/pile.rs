use crate::game_objects::card::{Card, Rank};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
};

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

    // ======== Rendering ========
    
    pub fn render(&self, buf: &mut Buffer, x: u16, y: u16) {
        match self.pile_type {
            PileType::Tableau => self.render_tableau(buf, x, y),
            PileType::Foundation => self.render_foundation(buf, x, y),
            PileType::Stock => self.render_stock(buf, x, y),
            PileType::Waste => self.render_waste(buf, x, y),
        }
    }

    fn render_tableau(&self, buf: &mut Buffer, x: u16, y: u16) {
        const VERTICAL_OVERLAP: u16 = 2;

        if self.is_empty() {
            self.render_empty_pile(buf, x, y);
            return;
        }

        for (idx, card) in self.cards.iter().enumerate() {
            let card_y = y + (idx as u16) * VERTICAL_OVERLAP;
            let is_last_card = idx == self.cards.len() - 1;

            if is_last_card {
                card.render(buf, x, card_y);
            } else {
                card.render_overlapped(buf, x, card_y, VERTICAL_OVERLAP);
            }
        }
    }

    fn render_foundation(&self, buf: &mut Buffer, x: u16, y: u16) {
        if self.is_empty() {
            self.render_empty_pile(buf, x, y);
        } else if let Some(card) = self.peek() {
            card.render(buf, x, y);
        }
    }

    fn render_stock(&self, buf: &mut Buffer, x: u16, y: u16) {
        if self.is_empty() {
            self.render_empty_pile(buf, x, y);
        } else if let Some(card) = self.peek() {
            card.render(buf, x, y);
        }
    }

    fn render_waste(&self, buf: &mut Buffer, x: u16, y: u16) {
        if self.is_empty() {
            self.render_empty_pile(buf, x, y);
        } else if let Some(card) = self.peek() {
            card.render(buf, x, y);
        }
    }

    fn render_empty_pile(&self, buf: &mut Buffer, x: u16, y: u16) {
        let area = Rect {
            x,
            y,
            width: Card::WIDTH,
            height: Card::HEIGHT,
        };

        if area.x + area.width <= buf.area.width && area.y + area.height <= buf.area.height {
            let horizontal_count = area.width.saturating_sub(2) as usize;
            let horizontal = "┄".repeat(horizontal_count);

            buf.set_string(area.x, area.y, &format!("┌{}┐", horizontal), Style::default().fg(Color::DarkGray));

            for row in 1..area.height.saturating_sub(1) {
                buf.set_string(area.x, area.y + row, "┊", Style::default().fg(Color::DarkGray));
                buf.set_string(area.x + area.width - 1, area.y + row, "┊", Style::default().fg(Color::DarkGray));
            }

            buf.set_string(area.x, area.y + area.height - 1, &format!("└{}┘", horizontal), Style::default().fg(Color::DarkGray));
        }
    }
}
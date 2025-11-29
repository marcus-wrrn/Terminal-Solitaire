use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub fn is_red(&self) -> bool {
        matches!(self, Suit::Hearts | Suit::Diamonds)
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

impl Rank {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub face_up: bool,
    pub is_selected: bool,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card {
            suit,
            rank,
            face_up: false,
            is_selected: false,
        }
    }

    pub fn is_opposite_color(&self, other: &Card) -> bool {
        self.suit.is_red() != other.suit.is_red()
    }

    pub fn is_one_rank_lower(&self, other: &Card) -> bool {
        self.rank.value() + 1 == other.rank.value()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.face_up {
            write!(f, "{}{}", self.rank, self.suit)
        } else {
            write!(f, "🂠")
        }
    }
}

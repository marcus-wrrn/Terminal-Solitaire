use std::fmt;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
};

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

    pub fn is_black(&self) -> bool {
        !self.is_red()
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
}

impl Card {
    pub const WIDTH: u16 = 9;
    pub const HEIGHT: u16 = 7;

    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card {
            suit,
            rank,
            face_up: false,
        }
    }

    pub fn flip(&mut self) {
        self.face_up = !self.face_up;
    }

    pub fn is_opposite_color(&self, other: &Card) -> bool {
        self.suit.is_red() != other.suit.is_red()
    }

    pub fn is_one_rank_lower(&self, other: &Card) -> bool {
        self.rank.value() + 1 == other.rank.value()
    }

    pub fn render(&self, buf: &mut Buffer, x: u16, y: u16) {
        let card_area = Rect {
            x,
            y,
            width: Self::WIDTH.min(buf.area.width.saturating_sub(x)),
            height: Self::HEIGHT.min(buf.area.height.saturating_sub(y)),
        };

        if card_area.width < 3 || card_area.height < 3 {
            return;
        }

        self.render_border(buf, card_area);

        if self.face_up {
            self.render_face_up(buf, card_area);
        } else {
            self.render_face_down(buf, card_area);
        }
    }

    fn render_border(&self, buf: &mut Buffer, area: Rect) {
        let horizontal_count = area.width.saturating_sub(2) as usize;
        let horizontal = "─".repeat(horizontal_count);

        buf.set_string(area.x, area.y, &format!("┌{}┐", horizontal), Style::default());

        for y in 1..area.height.saturating_sub(1) {
            buf.set_string(area.x, area.y + y, "│", Style::default());
            buf.set_string(area.x + area.width - 1, area.y + y, "│", Style::default());
        }

        buf.set_string(area.x, area.y + area.height - 1, &format!("└{}┘", horizontal), Style::default());
    }

    fn render_face_up(&self, buf: &mut Buffer, area: Rect) {
        let color = if self.suit.is_red() {
            Color::Red
        } else {
            Color::White
        };
        let style = Style::default().fg(color);

        let rank_str = format!("{}", self.rank);
        let suit_str = format!("{}", self.suit);

        if area.height >= 3 && area.width >= 3 {
            buf.set_string(area.x + 1, area.y + 1, &rank_str, style);
            buf.set_string(area.x + 2, area.y + 1, &suit_str, style);
        }

        if area.height >= 5 && area.width >= 5 {
            let center_y = area.y + area.height / 2;
            let center_x = area.x + area.width / 2;
            buf.set_string(center_x, center_y, &suit_str, style);
        }

        if area.height >= 3 && area.width >= 5 {
            let bottom_y = area.y + area.height - 2;
            let bottom_x = area.x + area.width - 3;
            buf.set_string(bottom_x, bottom_y, &rank_str, style);
            buf.set_string(bottom_x + 1, bottom_y, &suit_str, style);
        }
    }

    fn render_face_down(&self, buf: &mut Buffer, area: Rect) {
        let style = Style::default().fg(Color::Blue);

        if area.height >= 3 && area.width >= 3 {
            let pattern = "🂠";
            let center_y = area.y + area.height / 2;
            let center_x = area.x + area.width / 2;
            buf.set_string(center_x, center_y, pattern, style);
        }
    }

    pub fn render_overlapped(&self, buf: &mut Buffer, x: u16, y: u16, overlap: u16) {
        let visible_height = overlap;

        if visible_height < 2 {
            return;
        }

        let horizontal_count = Self::WIDTH.saturating_sub(2) as usize;
        let horizontal = "─".repeat(horizontal_count);
        

        buf.set_string(x, y, &format!("┌{}┐", horizontal), Style::default());

        if self.face_up {
            let color = if self.suit.is_red() {
                Color::Red
            } else {
                Color::White
            };
            let style = Style::default().fg(color);

            let rank_str = format!("{}", self.rank);
            let suit_str = format!("{}", self.suit);

            if visible_height >= 2 && Self::WIDTH >= 3 {
                buf.set_string(x + 1, y + 1, &rank_str, style);
                buf.set_string(x + 2, y + 1, &suit_str, style);
            }

            for row in 1..visible_height {
                buf.set_string(x, y + row, "│", Style::default());
                buf.set_string(x + Self::WIDTH - 1, y + row, "│", Style::default());
            }
        } else {
            let style = Style::default().fg(Color::Blue);

            for row in 1..visible_height {
                buf.set_string(x, y + row, "│", Style::default());
                buf.set_string(x + Self::WIDTH - 1, y + row, "│", Style::default());
            }

            if visible_height >= 2 {
                let pattern = "🂠";
                buf.set_string(x + Self::WIDTH / 2, y + 1, pattern, style);
            }
        }
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

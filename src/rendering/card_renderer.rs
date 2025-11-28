use crate::game_objects::Card;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
};

/// Renderer for individual Card objects
/// Handles all visual presentation of cards without modifying game state
pub struct CardRenderer;

impl CardRenderer {
    pub const WIDTH: u16 = 9;
    pub const HEIGHT: u16 = 7;

    /// Renders a complete card at the specified position
    pub fn render(card: &Card, buf: &mut Buffer, x: u16, y: u16) {
        let card_area = Rect {
            x,
            y,
            width: Self::WIDTH.min(buf.area.width.saturating_sub(x)),
            height: Self::HEIGHT.min(buf.area.height.saturating_sub(y)),
        };

        if card_area.width < 3 || card_area.height < 3 {
            return;
        }

        Self::render_border(buf, card_area, card.is_selected);

        if card.face_up {
            Self::render_face_up(card, buf, card_area);
        } else {
            Self::render_face_down(buf, card_area);
        }
    }

    /// Renders a card that is overlapped by another card (shows only partial top)
    /// Used for tableau piles where cards overlap vertically
    pub fn render_overlapped(card: &Card, buf: &mut Buffer, x: u16, y: u16, overlap: u16) {
        let visible_height = overlap;

        if visible_height < 2 {
            return;
        }

        let horizontal_count = Self::WIDTH.saturating_sub(2) as usize;

        let (top_left, top_right, horizontal, vertical) = if card.is_selected {
            ("╔", "╗", "═".repeat(horizontal_count), "║")
        } else {
            ("┌", "┐", "─".repeat(horizontal_count), "│")
        };

        let border_style = if card.is_selected {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        buf.set_string(
            x,
            y,
            &format!("{}{}{}", top_left, horizontal, top_right),
            border_style,
        );

        if card.face_up {
            let color = if card.suit.is_red() {
                Color::Red
            } else {
                Color::White
            };
            let style = Style::default().fg(color);

            let rank_str = format!("{}", card.rank);
            let suit_str = format!("{}", card.suit);

            if visible_height >= 2 && Self::WIDTH >= 3 {
                buf.set_string(x + 1, y + 1, &rank_str, style);
                buf.set_string(x + 2, y + 1, &suit_str, style);
            }

            for row in 1..visible_height {
                buf.set_string(x, y + row, vertical, border_style);
                buf.set_string(x + Self::WIDTH - 1, y + row, vertical, border_style);
            }
        } else {
            let style = Style::default().fg(Color::Blue);

            for row in 1..visible_height {
                buf.set_string(x, y + row, vertical, border_style);
                buf.set_string(x + Self::WIDTH - 1, y + row, vertical, border_style);
            }

            if visible_height >= 2 {
                let pattern = "🂠";
                buf.set_string(x + Self::WIDTH / 2, y + 1, pattern, style);
            }
        }
    }

    /// Renders the border of a card
    fn render_border(buf: &mut Buffer, area: Rect, is_selected: bool) {
        let horizontal_count = area.width.saturating_sub(2) as usize;

        let (top_left, top_right, bottom_left, bottom_right, horizontal, vertical) = if is_selected {
            ("╔", "╗", "╚", "╝", "═".repeat(horizontal_count), "║")
        } else {
            ("┌", "┐", "└", "┘", "─".repeat(horizontal_count), "│")
        };

        let border_style = if is_selected {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        buf.set_string(
            area.x,
            area.y,
            &format!("{}{}{}", top_left, horizontal, top_right),
            border_style,
        );

        for y in 1..area.height.saturating_sub(1) {
            buf.set_string(area.x, area.y + y, vertical, border_style);
            buf.set_string(
                area.x + area.width - 1,
                area.y + y,
                vertical,
                border_style,
            );
        }

        buf.set_string(
            area.x,
            area.y + area.height - 1,
            &format!("{}{}{}", bottom_left, horizontal, bottom_right),
            border_style,
        );
    }

    /// Renders a face-up card with rank and suit visible
    fn render_face_up(card: &Card, buf: &mut Buffer, area: Rect) {
        let color = if card.suit.is_red() {
            Color::Red
        } else {
            Color::White
        };
        let style = Style::default().fg(color);

        let rank_str = format!("{}", card.rank);
        let suit_str = format!("{}", card.suit);

        // Top-left rank and suit
        if area.height >= 3 && area.width >= 3 {
            buf.set_string(area.x + 1, area.y + 1, &rank_str, style);
            buf.set_string(area.x + 2, area.y + 1, &suit_str, style);
        }

        // Center suit symbol
        if area.height >= 5 && area.width >= 5 {
            let center_y = area.y + area.height / 2;
            let center_x = area.x + area.width / 2;
            buf.set_string(center_x, center_y, &suit_str, style);
        }

        // Bottom-right rank and suit
        if area.height >= 3 && area.width >= 5 {
            let bottom_y = area.y + area.height - 2;
            let bottom_x = area.x + area.width - 3;
            buf.set_string(bottom_x, bottom_y, &rank_str, style);
            buf.set_string(bottom_x + 1, bottom_y, &suit_str, style);
        }
    }

    /// Renders a face-down card with card back pattern
    fn render_face_down(buf: &mut Buffer, area: Rect) {
        let style = Style::default().fg(Color::Blue);

        if area.height >= 3 && area.width >= 3 {
            let pattern = "🂠";
            let center_y = area.y + area.height / 2;
            let center_x = area.x + area.width / 2;
            buf.set_string(center_x, center_y, pattern, style);
        }
    }
}

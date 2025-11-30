use crate::game_objects::Card;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
};

/// Renderer for individual Card objects
/// Handles all visual presentation of cards without modifying game state
pub struct CardRenderer {
    // debug_log: &'a DebugLog,
}

impl CardRenderer {
    pub const WIDTH: u16 = 9;
    pub const HEIGHT: u16 = 7;

    pub fn new() -> Self {
        Self { }
    }

    /// Renders a complete card at the specified position
    pub fn render(&self, card: &Card, is_selected: bool, buf: &mut Buffer, x: u16, y: u16) {
        let card_area = Rect {
            x,
            y,
            width: Self::WIDTH.min(buf.area.width.saturating_sub(x)),
            height: Self::HEIGHT.min(buf.area.height.saturating_sub(y)),
        };

        if card_area.width < 3 || card_area.height < 3 {
            return;
        }

        self.render_border(buf, card_area, is_selected);

        if card.face_up {
            self.render_face_up(card, buf, card_area);
        } else {
            self.render_face_down(buf, card_area);
        }
    }

    /// Renders a card that is overlapped by another card (shows only partial top)
    /// Used for tableau piles where cards overlap vertically
    pub fn render_overlapped(&self, card: &Card, is_selected: bool, buf: &mut Buffer, x: u16, y: u16, overlap: u16) {
        let visible_height = overlap;

        if visible_height < 2 {
            return;
        }

        let ((top_border, vertical, _), border_style) =
            Self::get_border_chars(is_selected, Self::WIDTH);

        buf.set_string(x, y, &top_border, border_style);

        if card.face_up {
            if visible_height >= 2 && Self::WIDTH >= 3 {
                self.render_label(card, buf, x + 1, y + 1);
            }

            for row in 1..visible_height {
                buf.set_string(x, y + row, &vertical, border_style);
                buf.set_string(x + Self::WIDTH - 1, y + row, &vertical, border_style);
            }
        } else {
            let style = Style::default().fg(Color::Blue);

            for row in 1..visible_height {
                buf.set_string(x, y + row, &vertical, border_style);
                buf.set_string(x + Self::WIDTH - 1, y + row, &vertical, border_style);
            }

            if visible_height >= 2 {
                let pattern = "🂠";
                buf.set_string(x + Self::WIDTH / 2, y + 1, pattern, style);
            }
        }
    }

    /// Renders the border of a card
    fn render_border(&self, buf: &mut Buffer, area: Rect, is_selected: bool) {
        let ((top_border, vertical, bottom_border), border_style) =
            Self::get_border_chars(is_selected, area.width);

        buf.set_string(area.x, area.y, &top_border, border_style);

        for y in 1..area.height.saturating_sub(1) {
            buf.set_string(area.x, area.y + y, &vertical, border_style);
            buf.set_string(area.x + area.width - 1, area.y + y, &vertical, border_style);
        }

        buf.set_string(
            area.x,
            area.y + area.height - 1,
            &bottom_border,
            border_style,
        );
    }

    /// Renders a face-up card with rank and suit visible
    fn render_face_up(&self, card: &Card, buf: &mut Buffer, area: Rect) {
        let color = if card.suit.is_red() {
            Color::Red
        } else {
            Color::White
        };
        let style = Style::default().fg(color);

        let suit_str = format!("{}", card.suit);

        // Top-left rank and suit
        if area.height >= 3 && area.width >= 3 {
            self.render_label(card, buf, area.x + 1, area.y + 1);
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
            let rank_str = format!("{}", card.rank);
            let rank_width = rank_str.len() as u16;
            let bottom_x = area.x + area.width - 1 - rank_width - 1;
            self.render_label(card, buf, bottom_x, bottom_y);
        }
    }

    /// Renders a face-down card with card back pattern
    fn render_face_down(&self, buf: &mut Buffer, area: Rect) {
        let style = Style::default().fg(Color::Blue);

        if area.height >= 3 && area.width >= 3 {
            let pattern = "🂠";
            let center_y = area.y + area.height / 2;
            let center_x = area.x + area.width / 2;
            buf.set_string(center_x, center_y, pattern, style);
        }
    }

    /// Renders rank and suit label for a card at the specified position
    /// Accommodates multi-character ranks (e.g., "10", "100")
    fn render_label(&self, card: &Card, buf: &mut Buffer, x: u16, y: u16) {
        let color = if card.suit.is_red() {
            Color::Red
        } else {
            Color::White
        };
        let style = Style::default().fg(color);

        let rank_str = format!("{}", card.rank);
        let suit_str = format!("{}", card.suit);
        let rank_width = rank_str.len() as u16;

        buf.set_string(x, y, &rank_str, style);
        buf.set_string(x + rank_width, y, &suit_str, style);
    }

    /// Returns border characters and style based on selection state
    fn get_border_chars(is_selected: bool, width: u16) -> ((String, String, String), Style) {
        let horizontal_count = width.saturating_sub(2) as usize;

        let chars = if is_selected {
            (
                format!("╔{}╗", "═".repeat(horizontal_count)),
                "║".to_string(),
                format!("╚{}╝", "═".repeat(horizontal_count)),
            )
        } else {
            (
                format!("┌{}┐", "─".repeat(horizontal_count)),
                "│".to_string(),
                format!("└{}┘", "─".repeat(horizontal_count)),
            )
        };

        let style = if is_selected {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        (chars, style)
    }
}

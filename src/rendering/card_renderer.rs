use crate::game_objects::Card;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

pub struct CardRenderer {
    card: Card,
    position: (u16, u16),
    width: u16,
    height: u16,
}

impl CardRenderer {
    pub fn new(card: Card, position: (u16, u16), width: u16, height: u16) -> Self {
        Self {
            card,
            position,
            width,
            height,
        }
    }

    fn render_card_border(&self, buf: &mut Buffer, area: Rect) {
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
        let color = if self.card.suit.is_red() {
            Color::Red
        } else {
            Color::White
        };
        let style = Style::default().fg(color);

        let rank_str = format!("{}", self.card.rank);
        let suit_str = format!("{}", self.card.suit);

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
}

impl Widget for CardRenderer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let card_area = Rect {
            x: self.position.0,
            y: self.position.1,
            width: self.width.min(area.width.saturating_sub(self.position.0)),
            height: self.height.min(area.height.saturating_sub(self.position.1)),
        };

        if card_area.width < 3 || card_area.height < 3 {
            return;
        }

        self.render_card_border(buf, card_area);

        if self.card.face_up {
            self.render_face_up(buf, card_area);
        } else {
            self.render_face_down(buf, card_area);
        }
    }
}

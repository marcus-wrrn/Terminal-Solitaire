use crate::game_objects::Hand;
use crate::rendering::card_renderer::CardRenderer;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
};

pub struct HandRenderer {
    card_renderer: CardRenderer,
}

impl HandRenderer {
    pub const SLOT_SPACING: u16 = 1;

    pub fn new() -> Self {
        Self {
            card_renderer: CardRenderer::new(),
        }
    }

    /// Renders a Hand as a horizontal row of card slots.
    /// `selected_slot` is the index of the currently selected slot, if any.
    pub fn render(
        &self,
        hand: &Hand,
        selected_slot: Option<usize>,
        buf: &mut Buffer,
        area: Rect,
    ) {
        for (i, slot) in hand.slots.iter().enumerate() {
            let x = area.x + (i as u16) * (CardRenderer::WIDTH + Self::SLOT_SPACING);
            if x + CardRenderer::WIDTH > buf.area.width {
                break;
            }

            let is_selected = selected_slot == Some(i);

            match &slot.card {
                Some(card) => {
                    self.card_renderer.render(card, is_selected, false, buf, x, area.y);
                }
                None => {
                    self.render_empty_slot(buf, x, area.y, is_selected);
                }
            }
        }
    }

    /// Returns the total width a hand with `capacity` slots would occupy.
    pub fn hand_width(capacity: usize) -> u16 {
        if capacity == 0 {
            return 0;
        }
        (capacity as u16) * CardRenderer::WIDTH + (capacity as u16 - 1) * Self::SLOT_SPACING
    }

    fn render_empty_slot(&self, buf: &mut Buffer, x: u16, y: u16, is_selected: bool) {
        let area = Rect {
            x,
            y,
            width: CardRenderer::WIDTH.min(buf.area.width.saturating_sub(x)),
            height: CardRenderer::HEIGHT.min(buf.area.height.saturating_sub(y)),
        };

        if area.width < 3 || area.height < 3 {
            return;
        }

        let horizontal_count = area.width.saturating_sub(2) as usize;

        let (top_border, vertical, bottom_border, border_style) = if is_selected {
            (
                format!("╔{}╗", "═".repeat(horizontal_count)),
                "║".to_string(),
                format!("╚{}╝", "═".repeat(horizontal_count)),
                Style::default().fg(Color::Yellow),
            )
        } else {
            (
                format!("┌{}┐", "┄".repeat(horizontal_count)),
                "┊".to_string(),
                format!("└{}┘", "┄".repeat(horizontal_count)),
                Style::default().fg(Color::DarkGray),
            )
        };

        buf.set_string(area.x, area.y, &top_border, border_style);

        for row in 1..area.height.saturating_sub(1) {
            buf.set_string(area.x, area.y + row, &vertical, border_style);
            buf.set_string(area.x + area.width - 1, area.y + row, &vertical, border_style);
        }

        buf.set_string(area.x, area.y + area.height - 1, &bottom_border, border_style);
    }
}

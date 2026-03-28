use crate::game_objects::CardRow;
use crate::rendering::balaterm::CardRowRenderer;
use crate::rendering::card_renderer::CardRenderer;
use ratatui::{buffer::Buffer, layout::Rect};

pub struct BalatermBoardRenderer {
    card_row_renderer: CardRowRenderer,
}

impl BalatermBoardRenderer {
    const ROW_SPACING: u16 = 1;

    pub fn new() -> Self {
        Self {
            card_row_renderer: CardRowRenderer::new(),
        }
    }

    pub fn render(
        &self,
        hands: &[CardRow],
        selected_hand: Option<usize>,
        selected_slot: Option<usize>,
        buf: &mut Buffer,
        area: Rect,
    ) {
        let total_height = Self::total_height(hands.len());
        let y_start = area.y + area.height.saturating_sub(total_height) / 2;

        for (i, hand) in hands.iter().enumerate() {
            let row_width = CardRowRenderer::row_width(hand.slots.len());
            let x_start = area.x + area.width.saturating_sub(row_width) / 2;
            let y = y_start + i as u16 * (CardRenderer::HEIGHT + Self::ROW_SPACING);

            let hand_area = Rect {
                x: x_start,
                y,
                width: row_width,
                height: CardRenderer::HEIGHT,
            };

            let slot = if selected_hand == Some(i) { selected_slot } else { None };
            self.card_row_renderer.render(hand, slot, buf, hand_area);
        }
    }

    fn total_height(hand_count: usize) -> u16 {
        if hand_count == 0 {
            return 0;
        }
        hand_count as u16 * CardRenderer::HEIGHT + (hand_count as u16 - 1) * Self::ROW_SPACING
    }
}

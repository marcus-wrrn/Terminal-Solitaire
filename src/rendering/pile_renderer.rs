use crate::game_objects::{Pile, PileType, Selection};
use crate::rendering::card_renderer::CardRenderer;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
};

/// Renderer for Pile objects
/// Handles rendering of different pile types and delegates card rendering to CardRenderer
pub struct PileRenderer;

impl PileRenderer {
    /// Vertical overlap distance for tableau cards
    pub const VERTICAL_OVERLAP: u16 = 2;

    pub fn render(pile: &Pile, pile_index: usize, current_selection: &Selection, buf: &mut Buffer, area: Rect) {
        let is_pile_selected = current_selection.pile == pile.pile_type
                            && current_selection.pile_index == pile_index;

        match pile.pile_type {
            PileType::Tableau => Self::render_tableau(pile, is_pile_selected, buf, area),
            PileType::Foundation => Self::render_foundation(pile, is_pile_selected, buf, area),
            PileType::Stock => Self::render_stock(pile, is_pile_selected, buf, area),
            PileType::Waste => Self::render_waste(pile, is_pile_selected, buf, area),
        }
    }

    fn render_tableau(pile: &Pile, is_pile_selected: bool, buf: &mut Buffer, area: Rect) {
        if pile.is_empty() {
            Self::render_empty_pile(buf, area, is_pile_selected);
            return;
        }

        for (idx, card) in pile.cards.iter().enumerate() {
            let card_y = area.y + (idx as u16) * Self::VERTICAL_OVERLAP;
            let is_last_card = idx == pile.cards.len() - 1;

            if is_last_card {
                CardRenderer::render(card, buf, area.x, card_y);
            } else {
                CardRenderer::render_overlapped(card, buf, area.x, card_y, Self::VERTICAL_OVERLAP);
            }
        }
    }

    fn render_foundation(pile: &Pile, is_pile_selected: bool, buf: &mut Buffer, area: Rect) {
        if pile.is_empty() {
            Self::render_empty_pile(buf, area, is_pile_selected);
        } else if let Some(card) = pile.peek() {
            CardRenderer::render(card, buf, area.x, area.y);
        }
    }

    fn render_stock(pile: &Pile, is_pile_selected: bool, buf: &mut Buffer, area: Rect) {
        if pile.is_empty() {
            Self::render_empty_pile(buf, area, is_pile_selected);
        } else if let Some(card) = pile.peek() {
            CardRenderer::render(card, buf, area.x, area.y);
        }
    }

    fn render_waste(pile: &Pile, is_pile_selected: bool, buf: &mut Buffer, area: Rect) {
        if pile.is_empty() {
            Self::render_empty_pile(buf, area, is_pile_selected);
        } else if let Some(card) = pile.peek() {
            CardRenderer::render(card, buf, area.x, area.y);
        }
    }

    fn render_empty_pile(buf: &mut Buffer, area: Rect, is_selected: bool) {
        let card_area = Rect {
            x: area.x,
            y: area.y,
            width: CardRenderer::WIDTH.min(area.width),
            height: CardRenderer::HEIGHT.min(area.height),
        };

        if card_area.x + card_area.width <= buf.area.width && card_area.y + card_area.height <= buf.area.height {
            let horizontal_count = card_area.width.saturating_sub(2) as usize;

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

            buf.set_string(
                card_area.x,
                card_area.y,
                &top_border,
                border_style,
            );

            for row in 1..card_area.height.saturating_sub(1) {
                buf.set_string(card_area.x, card_area.y + row, &vertical, border_style);
                buf.set_string(
                    card_area.x + card_area.width - 1,
                    card_area.y + row,
                    &vertical,
                    border_style,
                );
            }

            buf.set_string(
                card_area.x,
                card_area.y + card_area.height - 1,
                &bottom_border,
                border_style,
            );
        }
    }
}

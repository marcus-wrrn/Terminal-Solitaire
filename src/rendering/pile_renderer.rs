use crate::game_objects::{Pile, PileType};
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

    /// Renders a pile within the specified area
    /// Rendering behavior depends on pile type (Tableau, Foundation, Stock, Waste)
    pub fn render(pile: &Pile, buf: &mut Buffer, area: Rect) {
        match pile.pile_type {
            PileType::Tableau => Self::render_tableau(pile, buf, area),
            PileType::Foundation => Self::render_foundation(pile, buf, area),
            PileType::Stock => Self::render_stock(pile, buf, area),
            PileType::Waste => Self::render_waste(pile, buf, area),
        }
    }

    /// Renders a tableau pile with overlapping cards
    fn render_tableau(pile: &Pile, buf: &mut Buffer, area: Rect) {
        if pile.is_empty() {
            Self::render_empty_pile(buf, area);
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

    /// Renders a foundation pile (shows only top card)
    fn render_foundation(pile: &Pile, buf: &mut Buffer, area: Rect) {
        if pile.is_empty() {
            Self::render_empty_pile(buf, area);
        } else if let Some(card) = pile.peek() {
            CardRenderer::render(card, buf, area.x, area.y);
        }
    }

    /// Renders the stock pile (shows only top card)
    fn render_stock(pile: &Pile, buf: &mut Buffer, area: Rect) {
        if pile.is_empty() {
            Self::render_empty_pile(buf, area);
        } else if let Some(card) = pile.peek() {
            CardRenderer::render(card, buf, area.x, area.y);
        }
    }

    /// Renders the waste pile (shows only top card)
    fn render_waste(pile: &Pile, buf: &mut Buffer, area: Rect) {
        if pile.is_empty() {
            Self::render_empty_pile(buf, area);
        } else if let Some(card) = pile.peek() {
            CardRenderer::render(card, buf, area.x, area.y);
        }
    }

    /// Renders an empty pile placeholder with dashed border
    fn render_empty_pile(buf: &mut Buffer, area: Rect) {
        let card_area = Rect {
            x: area.x,
            y: area.y,
            width: CardRenderer::WIDTH.min(area.width),
            height: CardRenderer::HEIGHT.min(area.height),
        };

        if card_area.x + card_area.width <= buf.area.width && card_area.y + card_area.height <= buf.area.height {
            let horizontal_count = card_area.width.saturating_sub(2) as usize;
            let horizontal = "┄".repeat(horizontal_count);

            buf.set_string(
                card_area.x,
                card_area.y,
                &format!("┌{}┐", horizontal),
                Style::default().fg(Color::DarkGray),
            );

            for row in 1..card_area.height.saturating_sub(1) {
                buf.set_string(card_area.x, card_area.y + row, "┊", Style::default().fg(Color::DarkGray));
                buf.set_string(
                    card_area.x + card_area.width - 1,
                    card_area.y + row,
                    "┊",
                    Style::default().fg(Color::DarkGray),
                );
            }

            buf.set_string(
                card_area.x,
                card_area.y + card_area.height - 1,
                &format!("└{}┘", horizontal),
                Style::default().fg(Color::DarkGray),
            );
        }
    }
}

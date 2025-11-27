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
    const VERTICAL_OVERLAP: u16 = 2;

    /// Renders a pile at the specified position
    /// Rendering behavior depends on pile type (Tableau, Foundation, Stock, Waste)
    pub fn render(pile: &Pile, buf: &mut Buffer, x: u16, y: u16) {
        match pile.pile_type {
            PileType::Tableau => Self::render_tableau(pile, buf, x, y),
            PileType::Foundation => Self::render_foundation(pile, buf, x, y),
            PileType::Stock => Self::render_stock(pile, buf, x, y),
            PileType::Waste => Self::render_waste(pile, buf, x, y),
        }
    }

    /// Renders a tableau pile with overlapping cards
    fn render_tableau(pile: &Pile, buf: &mut Buffer, x: u16, y: u16) {
        if pile.is_empty() {
            Self::render_empty_pile(buf, x, y);
            return;
        }

        for (idx, card) in pile.cards.iter().enumerate() {
            let card_y = y + (idx as u16) * Self::VERTICAL_OVERLAP;
            let is_last_card = idx == pile.cards.len() - 1;

            if is_last_card {
                CardRenderer::render(card, buf, x, card_y);
            } else {
                CardRenderer::render_overlapped(card, buf, x, card_y, Self::VERTICAL_OVERLAP);
            }
        }
    }

    /// Renders a foundation pile (shows only top card)
    fn render_foundation(pile: &Pile, buf: &mut Buffer, x: u16, y: u16) {
        if pile.is_empty() {
            Self::render_empty_pile(buf, x, y);
        } else if let Some(card) = pile.peek() {
            CardRenderer::render(card, buf, x, y);
        }
    }

    /// Renders the stock pile (shows only top card)
    fn render_stock(pile: &Pile, buf: &mut Buffer, x: u16, y: u16) {
        if pile.is_empty() {
            Self::render_empty_pile(buf, x, y);
        } else if let Some(card) = pile.peek() {
            CardRenderer::render(card, buf, x, y);
        }
    }

    /// Renders the waste pile (shows only top card)
    fn render_waste(pile: &Pile, buf: &mut Buffer, x: u16, y: u16) {
        if pile.is_empty() {
            Self::render_empty_pile(buf, x, y);
        } else if let Some(card) = pile.peek() {
            CardRenderer::render(card, buf, x, y);
        }
    }

    /// Renders an empty pile placeholder with dashed border
    fn render_empty_pile(buf: &mut Buffer, x: u16, y: u16) {
        let area = Rect {
            x,
            y,
            width: CardRenderer::WIDTH,
            height: CardRenderer::HEIGHT,
        };

        if area.x + area.width <= buf.area.width && area.y + area.height <= buf.area.height {
            let horizontal_count = area.width.saturating_sub(2) as usize;
            let horizontal = "┄".repeat(horizontal_count);

            buf.set_string(
                area.x,
                area.y,
                &format!("┌{}┐", horizontal),
                Style::default().fg(Color::DarkGray),
            );

            for row in 1..area.height.saturating_sub(1) {
                buf.set_string(area.x, area.y + row, "┊", Style::default().fg(Color::DarkGray));
                buf.set_string(
                    area.x + area.width - 1,
                    area.y + row,
                    "┊",
                    Style::default().fg(Color::DarkGray),
                );
            }

            buf.set_string(
                area.x,
                area.y + area.height - 1,
                &format!("└{}┘", horizontal),
                Style::default().fg(Color::DarkGray),
            );
        }
    }
}

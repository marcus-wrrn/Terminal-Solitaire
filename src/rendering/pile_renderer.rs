use crate::game_objects::{Pile, PileType, Selection};
use crate::rendering::card_renderer::CardRenderer;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
};

/// Renderer for Pile objects
/// Handles rendering of different pile types and delegates card rendering to CardRenderer
pub struct PileRenderer {
    card_renderer: CardRenderer,
}

impl PileRenderer {
    /// Vertical overlap distance for tableau cards
    pub const VERTICAL_OVERLAP: u16 = 2;

    pub fn new() -> Self {
        Self {
            card_renderer: CardRenderer::new(),
        }
    }

    pub fn render(&self, pile: &Pile, pile_index: usize, current_selection: &Selection, hover_selection: Option<&Selection>, buf: &mut Buffer, area: Rect) {
        let is_pile_selected = current_selection.pile == pile.pile_type
                            && current_selection.pile_index == pile_index;

        let is_hover_target = hover_selection
            .map(|h| h.pile == pile.pile_type && h.pile_index == pile_index)
            .unwrap_or(false);

        match pile.pile_type {
            PileType::Tableau => self.render_tableau(pile, is_pile_selected, is_hover_target, buf, area),
            PileType::Foundation => self.render_foundation(pile, is_pile_selected, is_hover_target, buf, area),
            PileType::Stock => self.render_stock(pile, is_pile_selected, is_hover_target, buf, area),
            PileType::Waste => self.render_waste(pile, is_pile_selected, is_hover_target, buf, area),
        }
    }

    fn render_tableau(&self, pile: &Pile, is_pile_selected: bool, is_hover_target: bool, buf: &mut Buffer, area: Rect) {
        if pile.is_empty() {
            self.render_empty_pile(buf, area, is_pile_selected, is_hover_target);
            return;
        }

        for (idx, card) in pile.cards.iter().enumerate() {
            let card_y = area.y + (idx as u16) * Self::VERTICAL_OVERLAP;
            let is_last_card = idx == pile.cards.len() - 1;

            if is_last_card {
                self.card_renderer.render(card, buf, area.x, card_y);
            } else {
                self.card_renderer.render_overlapped(card, buf, area.x, card_y, Self::VERTICAL_OVERLAP);
            }
        }

        if is_hover_target && !pile.is_empty() {
            let mut area_cp = area.clone();
            area_cp.y = area.y + (pile.len() as u16) * Self::VERTICAL_OVERLAP;
            self.render_hover_highlight(buf, area_cp);
        }
    }

    fn render_foundation(&self, pile: &Pile, is_pile_selected: bool, is_hover_target: bool, buf: &mut Buffer, area: Rect) {
        if pile.is_empty() {
            self.render_empty_pile(buf, area, is_pile_selected, is_hover_target);
        } else if let Some(card) = pile.peek() {
            self.card_renderer.render(card, buf, area.x, area.y);
            if is_hover_target {
                self.render_hover_highlight(buf, area);
            }
        }
    }

    fn render_stock(&self, pile: &Pile, is_pile_selected: bool, is_hover_target: bool, buf: &mut Buffer, area: Rect) {
        if pile.is_empty() {
            self.render_empty_pile(buf, area, is_pile_selected, is_hover_target);
        } else if let Some(card) = pile.peek() {
            self.card_renderer.render(card, buf, area.x, area.y);
            if is_hover_target {
                self.render_hover_highlight(buf, area);
            }
        }
    }

    fn render_waste(&self, pile: &Pile, is_pile_selected: bool, is_hover_target: bool, buf: &mut Buffer, area: Rect) {
        if pile.is_empty() {
            self.render_empty_pile(buf, area, is_pile_selected, is_hover_target);
        } else if let Some(card) = pile.peek() {
            self.card_renderer.render(card, buf, area.x, area.y);
            if is_hover_target {
                self.render_hover_highlight(buf, area);
            }
        }
    }

    fn render_empty_pile(&self, buf: &mut Buffer, area: Rect, is_selected: bool, is_hover_target: bool) {
        let card_area = Rect {
            x: area.x,
            y: area.y,
            width: CardRenderer::WIDTH.min(area.width),
            height: CardRenderer::HEIGHT.min(area.height),
        };

        if card_area.x + card_area.width <= buf.area.width && card_area.y + card_area.height <= buf.area.height {
            let horizontal_count = card_area.width.saturating_sub(2) as usize;

            let (top_border, vertical, bottom_border, border_style) = if is_hover_target {
                (
                    format!("╔{}╗", "═".repeat(horizontal_count)),
                    "║".to_string(),
                    format!("╚{}╝", "═".repeat(horizontal_count)),
                    Style::default().fg(Color::Green),
                )
            } else if is_selected {
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

    fn render_hover_highlight(&self, buf: &mut Buffer, area: Rect) {
        let card_area = Rect {
            x: area.x,
            y: area.y,
            width: CardRenderer::WIDTH.min(area.width),
            height: CardRenderer::HEIGHT.min(area.height),
        };

        if card_area.x + card_area.width > buf.area.width || card_area.y + card_area.height > buf.area.height {
            return;
        }

        let horizontal_count = card_area.width.saturating_sub(2) as usize;
        let border_style = Style::default().fg(Color::Green);

        let top_border = format!("╔{}╗", "═".repeat(horizontal_count));
        let vertical = "║";
        let bottom_border = format!("╚{}╝", "═".repeat(horizontal_count));

        buf.set_string(card_area.x, card_area.y, &top_border, border_style);

        for row in 1..card_area.height.saturating_sub(1) {
            buf.set_string(card_area.x, card_area.y + row, vertical, border_style);
            buf.set_string(
                card_area.x + card_area.width - 1,
                card_area.y + row,
                vertical,
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

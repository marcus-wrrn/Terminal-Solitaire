use crate::game_objects::{HoverState, Pile, PileType, Selection};
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

    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &self,
        pile: &Pile,
        pile_index: usize,
        selection: Option<&Selection>,
        picked_up: Option<&Selection>,
        hover_state: &HoverState,
        buf: &mut Buffer,
        area: Rect,
    ) {
        let (is_hover_target, is_valid) = match hover_state {
            HoverState::Valid(sel)
                if sel.pile == pile.pile_type && sel.pile_index == pile_index =>
            {
                (true, true)
            }
            HoverState::Invalid(sel)
                if sel.pile == pile.pile_type && sel.pile_index == pile_index =>
            {
                (true, false)
            }
            _ => (false, false),
        };

        let is_pile_selected = selection
            .map(|sel| sel.pile == pile.pile_type && sel.pile_index == pile_index)
            .unwrap_or(false);

        if pile.is_empty() {
            self.render_empty_pile(buf, area, is_pile_selected, is_hover_target, is_valid);
            return;
        }

        if pile.pile_type == PileType::Tableau {
            for (idx, card) in pile.cards.iter().enumerate() {
                let card_y = area.y + (idx as u16) * Self::VERTICAL_OVERLAP;
                let is_last_card = idx == pile.cards.len() - 1;
                let is_card_selected =
                    is_pile_selected && selection.map(|sel| sel.card_index == idx).unwrap_or(false);
                let is_card_picked_up = picked_up
                    .map(|sel| {
                        sel.pile == pile.pile_type
                            && sel.pile_index == pile_index
                            && sel.card_index == idx
                    })
                    .unwrap_or(false);

                if is_last_card {
                    self.card_renderer.render(
                        card,
                        is_card_selected,
                        is_card_picked_up,
                        buf,
                        area.x,
                        card_y,
                    );
                } else {
                    self.card_renderer.render_overlapped(
                        card,
                        is_card_selected,
                        is_card_picked_up,
                        buf,
                        area.x,
                        card_y,
                        Self::VERTICAL_OVERLAP,
                    );
                }
            }

            if is_hover_target && !pile.is_empty() {
                let mut area_cp = area;
                area_cp.y = area.y + (pile.len() as u16) * Self::VERTICAL_OVERLAP;
                self.render_hover_highlight(buf, area_cp, is_valid);
            }

            if is_pile_selected
                && !pile.is_empty()
                && let Some(sel) = selection
                && sel.card_index == pile.len()
            {
                let mut area_cp = area;
                area_cp.y = area.y + (pile.len() as u16) * Self::VERTICAL_OVERLAP;
                self.render_selection_highlight(buf, area_cp);
            }
        } else if let Some(card) = pile.peek() {
            let is_card_selected = is_pile_selected
                && selection
                    .map(|sel| sel.card_index == pile.len() - 1)
                    .unwrap_or(false);
            let is_card_picked_up = picked_up
                .map(|sel| {
                    sel.pile == pile.pile_type
                        && sel.pile_index == pile_index
                        && sel.card_index == pile.len() - 1
                })
                .unwrap_or(false);
            self.card_renderer.render(
                card,
                is_card_selected,
                is_card_picked_up,
                buf,
                area.x,
                area.y,
            );
            if is_hover_target {
                self.render_hover_highlight(buf, area, is_valid);
            }

            if is_pile_selected
                && let Some(sel) = selection
                && sel.card_index == pile.len()
            {
                self.render_selection_highlight(buf, area);
            }
        }
    }

    fn render_empty_pile(
        &self,
        buf: &mut Buffer,
        area: Rect,
        is_selected: bool,
        is_hover_target: bool,
        is_valid: bool,
    ) {
        let card_area = Rect {
            x: area.x,
            y: area.y,
            width: CardRenderer::WIDTH.min(area.width),
            height: CardRenderer::HEIGHT.min(area.height),
        };

        if card_area.x + card_area.width <= buf.area.width
            && card_area.y + card_area.height <= buf.area.height
        {
            let horizontal_count = card_area.width.saturating_sub(2) as usize;

            let (top_border, vertical, bottom_border, border_style) = if is_hover_target {
                let color = if is_valid { Color::Green } else { Color::Red };
                (
                    format!("╔{}╗", "═".repeat(horizontal_count)),
                    "║".to_string(),
                    format!("╚{}╝", "═".repeat(horizontal_count)),
                    Style::default().fg(color),
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

            buf.set_string(card_area.x, card_area.y, &top_border, border_style);

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

    fn render_selection_highlight(&self, buf: &mut Buffer, area: Rect) {
        let card_area = Rect {
            x: area.x,
            y: area.y,
            width: CardRenderer::WIDTH.min(area.width),
            height: CardRenderer::HEIGHT.min(area.height),
        };

        if card_area.x + card_area.width > buf.area.width
            || card_area.y + card_area.height > buf.area.height
        {
            return;
        }

        let horizontal_count = card_area.width.saturating_sub(2) as usize;
        let border_style = Style::default().fg(Color::Yellow);

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

    fn render_hover_highlight(&self, buf: &mut Buffer, area: Rect, is_valid: bool) {
        let card_area = Rect {
            x: area.x,
            y: area.y,
            width: CardRenderer::WIDTH.min(area.width),
            height: CardRenderer::HEIGHT.min(area.height),
        };

        if card_area.x + card_area.width > buf.area.width
            || card_area.y + card_area.height > buf.area.height
        {
            return;
        }

        let horizontal_count = card_area.width.saturating_sub(2) as usize;
        let color = if is_valid { Color::Green } else { Color::Red };
        let border_style = Style::default().fg(color);

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

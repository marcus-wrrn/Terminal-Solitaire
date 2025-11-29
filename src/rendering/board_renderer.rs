use crate::game_objects::{Board, PileType, Selection};
use crate::rendering::{card_renderer::CardRenderer, pile_renderer::PileRenderer};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
};

/// Stores the bounds of a pile on the screen
#[derive(Debug, Clone, Copy)]
pub struct PileBounds {
    pub pile_type: PileType,
    pub pile_index: usize,
    pub rect: Rect,
}

/// Renderer for the solitaire board (stock, waste, foundation, and tableau piles)
pub struct BoardRenderer {
    pile_renderer: PileRenderer,
    pile_bounds: Vec<PileBounds>,
}

impl BoardRenderer {
    const HORIZONTAL_SPACING: u16 = 2;

    pub fn new() -> Self {
        Self {
            pile_renderer: PileRenderer::new(),
            pile_bounds: Vec::new(),
        }
    }

    /// Renders the complete board with hover highlighting
    pub fn render(&mut self, board: &Board, selection: &Selection, hover_selection: Option<&Selection>, buf: &mut Buffer, area: Rect) {
        self.pile_bounds.clear();
        let vertical_sections = Layout::vertical([
            Constraint::Length(CardRenderer::HEIGHT + 1),   // Stock/Waste/Foundation + labels
            Constraint::Length(2),                          // Spacing
            Constraint::Min(CardRenderer::HEIGHT + 1),      // Tableau (grows as needed)
        ])
        .flex(Flex::Center)
        .split(area);

        let top_row_sections = Layout::horizontal([
            Constraint::Length(CardRenderer::WIDTH),   // Stock
            Constraint::Length(CardRenderer::WIDTH),   // Waste
            Constraint::Length(9),                     // Gap
            Constraint::Length(CardRenderer::WIDTH),   // Foundation 1
            Constraint::Length(CardRenderer::WIDTH),   // Foundation 2
            Constraint::Length(CardRenderer::WIDTH),   // Foundation 3
            Constraint::Length(CardRenderer::WIDTH),   // Foundation 4
        ])
        .spacing(Self::HORIZONTAL_SPACING)
        .flex(Flex::Center)
        .split(vertical_sections[0]);

        self.render_stock_and_waste(board, selection, hover_selection, buf, top_row_sections[0], top_row_sections[1]);
        self.render_foundations(board, selection, hover_selection, buf, &top_row_sections[3..7]);

        let tableau_sections = Layout::horizontal([
            Constraint::Length(CardRenderer::WIDTH),
            Constraint::Length(CardRenderer::WIDTH),
            Constraint::Length(CardRenderer::WIDTH),
            Constraint::Length(CardRenderer::WIDTH),
            Constraint::Length(CardRenderer::WIDTH),
            Constraint::Length(CardRenderer::WIDTH),
            Constraint::Length(CardRenderer::WIDTH),
        ])
        .spacing(Self::HORIZONTAL_SPACING)
        .flex(Flex::Center)
        .split(vertical_sections[2]);

        self.render_tableau(board, selection, hover_selection, buf, &tableau_sections);
    }

    fn render_pile_label(&self, buf: &mut Buffer, area: Rect, label: &str) {
        buf.set_string(area.x, area.y, label, Style::default().fg(Color::Gray));
    }

    fn render_stock_and_waste(&mut self, board: &Board, selection: &Selection, hover_selection: Option<&Selection>, buf: &mut Buffer, stock_area: Rect, waste_area: Rect) {
        self.render_pile_label(buf, stock_area, "Stock");
        let stock_pile_area = Rect { x: stock_area.x, y: stock_area.y + 1, ..stock_area };
        self.pile_bounds.push(PileBounds {
            pile_type: PileType::Stock,
            pile_index: 0,
            rect: stock_pile_area,
        });
        self.pile_renderer.render(&board.stock, 0, selection, hover_selection, buf, stock_pile_area);

        self.render_pile_label(buf, waste_area, "Waste");
        let waste_pile_area = Rect { x: waste_area.x, y: waste_area.y + 1, ..waste_area };
        self.pile_bounds.push(PileBounds {
            pile_type: PileType::Waste,
            pile_index: 0,
            rect: waste_pile_area,
        });
        self.pile_renderer.render(&board.waste, 0, selection, hover_selection, buf, waste_pile_area);
    }

    fn render_foundations(&mut self, board: &Board, selection: &Selection, hover_selection: Option<&Selection>, buf: &mut Buffer, foundation_areas: &[Rect]) {
        for (i, (pile, area)) in board.foundation.iter().zip(foundation_areas).enumerate() {
            self.render_pile_label(buf, *area, &format!("F{}", i + 1));
            let pile_area = Rect { x: area.x, y: area.y + 1, ..*area };
            self.pile_bounds.push(PileBounds {
                pile_type: PileType::Foundation,
                pile_index: i,
                rect: pile_area,
            });
            self.pile_renderer.render(pile, i, selection, hover_selection, buf, pile_area);
        }
    }

    fn render_tableau(&mut self, board: &Board, selection: &Selection, hover_selection: Option<&Selection>, buf: &mut Buffer, tableau_areas: &[Rect]) {
        for (i, (pile, area)) in board.tableau.iter().zip(tableau_areas).enumerate() {
            self.render_pile_label(buf, *area, &format!("T{}", i + 1));
            let pile_area = Rect { x: area.x, y: area.y + 1, ..*area };
            self.pile_bounds.push(PileBounds {
                pile_type: PileType::Tableau,
                pile_index: i,
                rect: pile_area,
            });
            self.pile_renderer.render(pile, i, selection, hover_selection, buf, pile_area);
        }
    }

    /// Converts screen coordinates to a Selection, if a pile is at that position
    pub fn coordinate_to_selection(&self, board: &Board, x: u16, y: u16) -> Option<Selection> {
        for bounds in &self.pile_bounds {
            if x >= bounds.rect.x
                && x < bounds.rect.x + bounds.rect.width
                && y >= bounds.rect.y
                && y < bounds.rect.y + bounds.rect.height
            {
                let card_index = match bounds.pile_type {
                    PileType::Tableau => {
                        if let Some(pile) = board.get_tableau_pile(bounds.pile_index) {
                            if pile.is_empty() {
                                0
                            } else {
                                let offset_y = y.saturating_sub(bounds.rect.y);
                                let estimated_index = (offset_y / PileRenderer::VERTICAL_OVERLAP) as usize;
                                estimated_index.min(pile.len() - 1)
                            }
                        } else {
                            0
                        }
                    }
                    PileType::Foundation => {
                        if let Some(pile) = board.get_foundation_pile(bounds.pile_index) {
                            if pile.is_empty() { 0 } else { pile.len() - 1 }
                        } else {
                            0
                        }
                    }
                    PileType::Waste => {
                        let len = board.waste.len();
                        if len > 0 { len - 1 } else { 0 }
                    }
                    PileType::Stock => {
                        let len = board.stock.len();
                        if len > 0 { len - 1 } else { 0 }
                    }
                };

                return Some(Selection::new(
                    bounds.pile_type,
                    bounds.pile_index,
                    card_index,
                ));
            }
        }
        None
    }

    // /// Returns the bounds of a specific pile, if it exists
    // pub fn get_pile_bounds(&self, pile_type: PileType, pile_index: usize) -> Option<Rect> {
    //     self.pile_bounds
    //         .iter()
    //         .find(|b| b.pile_type == pile_type && b.pile_index == pile_index)
    //         .map(|b| b.rect)
    // }
}

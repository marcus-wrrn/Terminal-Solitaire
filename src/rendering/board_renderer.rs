use crate::game_objects::{Board, Pile, PileType, Selection, HoverState};
use crate::rendering::render_instructions::BoardRenderingIntr;
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
    pub fn render(
        &mut self, 
        instr: &BoardRenderingIntr,
        buf: &mut Buffer, 
        area: Rect
    ) {
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

        //self.render_stock_and_waste(board, selection, hover_state, buf, top_row_sections[0], top_row_sections[1]);
        self.render_pile(Some("Stock"), &instr.board.stock, 0, instr.selection, instr.picked_up, instr.hover_state, top_row_sections[0], buf);
        self.render_pile(Some("Waste"), &instr.board.waste, 0, instr.selection, instr.picked_up, instr.hover_state, top_row_sections[1], buf);

        self.render_foundations(instr.board, instr.selection, instr.picked_up, instr.hover_state, buf, &top_row_sections[3..7]);

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

        self.render_tableau(instr.board, instr.selection, instr.picked_up, instr.hover_state, buf, &tableau_sections);
    }

    fn render_foundations(&mut self, board: &Board, selection: Option<&Selection>, picked_up: Option<&Selection>, hover_state: &HoverState, buf: &mut Buffer, foundation_areas: &[Rect]) {
        for (i, (pile, area)) in board.foundation.iter().zip(foundation_areas).enumerate() {
            self.render_pile(Some(&format!("F{}", i + 1)), pile, i, selection, picked_up, hover_state, *area, buf);
        }
    }

    fn render_tableau(&mut self, board: &Board, selection: Option<&Selection>, picked_up: Option<&Selection>, hover_state: &HoverState, buf: &mut Buffer, tableau_areas: &[Rect]) {
        for (i, (pile, area)) in board.tableau.iter().zip(tableau_areas).enumerate() {
            self.render_pile(Some(&format!("T{}", i)), pile, i, selection, picked_up, hover_state, *area, buf);
        }
    }

    fn render_pile_label(&self, buf: &mut Buffer, area: Rect, label: &str) {
        buf.set_string(area.x, area.y, label, Style::default().fg(Color::Gray));
    }

    fn render_pile(&mut self,
        label: Option<&str>,
        pile: &Pile,
        index: usize,
        selection: Option<&Selection>,
        picked_up: Option<&Selection>,
        hover_state: &HoverState,
        area: Rect,
        buf: &mut Buffer
    ) {
        if let Some(lab) = label {
            self.render_pile_label(buf, area, lab);
        }
        let pile_area = Rect { x: area.x, y: area.y + 1, ..area };
        self.pile_bounds.push(PileBounds {
            pile_type: pile.pile_type,
            pile_index: index,
            rect: pile_area,
        });
        self.pile_renderer.render(pile, index, selection, picked_up, hover_state, buf, pile_area);
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
}

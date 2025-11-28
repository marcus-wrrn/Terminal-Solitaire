use crate::game_objects::{Board, Selection};
use crate::rendering::{card_renderer::CardRenderer, pile_renderer::PileRenderer};
use crate::ui::DebugLog;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
};

/// Renderer for the solitaire board (stock, waste, foundation, and tableau piles)
pub struct BoardRenderer<'a> {
    debug_log: &'a DebugLog,
    pile_renderer: &'a PileRenderer<'a>,
}

impl<'a> BoardRenderer<'a> {
    const HORIZONTAL_SPACING: u16 = 2;

    pub fn new(debug_log: &'a DebugLog, pile_renderer: &'a PileRenderer<'a>) -> Self {
        Self { debug_log, pile_renderer }
    }

    /// Renders the complete board within the specified area
    pub fn render(&self, board: &Board, selection: &Selection, buf: &mut Buffer, area: Rect) {
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

        self.render_stock_and_waste(board, selection, buf, top_row_sections[0], top_row_sections[1]);
        self.render_foundations(board, selection, buf, &top_row_sections[3..7]);

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

        self.render_tableau(board, selection, buf, &tableau_sections);
    }

    fn render_pile_label(&self, buf: &mut Buffer, area: Rect, label: &str) {
        buf.set_string(area.x, area.y, label, Style::default().fg(Color::Gray));
    }

    fn render_stock_and_waste(&self, board: &Board, selection: &Selection, buf: &mut Buffer, stock_area: Rect, waste_area: Rect) {
        self.render_pile_label(buf, stock_area, "Stock");
        let stock_pile_area = Rect { x: stock_area.x, y: stock_area.y + 1, ..stock_area };
        self.pile_renderer.render(&board.stock, 0, selection, buf, stock_pile_area);

        self.render_pile_label(buf, waste_area, "Waste");
        let waste_pile_area = Rect { x: waste_area.x, y: waste_area.y + 1, ..waste_area };
        self.pile_renderer.render(&board.waste, 0, selection, buf, waste_pile_area);
    }

    fn render_foundations(&self, board: &Board, selection: &Selection, buf: &mut Buffer, foundation_areas: &[Rect]) {
        for (i, (pile, area)) in board.foundation.iter().zip(foundation_areas).enumerate() {
            self.render_pile_label(buf, *area, &format!("F{}", i + 1));
            let pile_area = Rect { x: area.x, y: area.y + 1, ..*area };
            self.pile_renderer.render(pile, i, selection, buf, pile_area);
        }
    }

    fn render_tableau(&self, board: &Board, selection: &Selection, buf: &mut Buffer, tableau_areas: &[Rect]) {
        for (i, (pile, area)) in board.tableau.iter().zip(tableau_areas).enumerate() {
            self.render_pile_label(buf, *area, &format!("T{}", i + 1));
            let pile_area = Rect { x: area.x, y: area.y + 1, ..*area };
            self.pile_renderer.render(pile, i, selection, buf, pile_area);
        }
    }
}

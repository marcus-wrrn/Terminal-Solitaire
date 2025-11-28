use crate::game_objects::Board;
use crate::rendering::{card_renderer::CardRenderer, pile_renderer::PileRenderer};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    widgets::Widget,
};

/// Primary Rendering manager for the Solitair board
pub struct BoardRenderer<'a> {
    board: &'a Board,
}

impl<'a> BoardRenderer<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self { board }
    }

    const HORIZONTAL_SPACING: u16 = 2;

    /// Renders a centered text string within the given area
    fn render_centered_text(&self, buf: &mut Buffer, area: Rect, text: &str, style: Style) {
        if area.width >= text.len() as u16 {
            let x = area.x + (area.width - text.len() as u16) / 2;
            let y = area.y;
            buf.set_string(x, y, text, style);
        }
    }

    fn render_pile_label(&self, buf: &mut Buffer, area: Rect, label: &str) {
        buf.set_string(area.x, area.y, label, Style::default().fg(Color::Gray));
    }

    fn render_stock_and_waste(&self, buf: &mut Buffer, stock_area: Rect, waste_area: Rect) {
        self.render_pile_label(buf, stock_area, "Stock");
        let stock_pile_area = Rect { x: stock_area.x, y: stock_area.y + 1, ..stock_area };
        PileRenderer::render(&self.board.stock, buf, stock_pile_area);

        self.render_pile_label(buf, waste_area, "Waste");
        let waste_pile_area = Rect { x: waste_area.x, y: waste_area.y + 1, ..waste_area };
        PileRenderer::render(&self.board.waste, buf, waste_pile_area);
    }

    fn render_foundations(&self, buf: &mut Buffer, foundation_areas: &[Rect]) {
        for (i, (pile, area)) in self.board.foundation.iter().zip(foundation_areas).enumerate() {
            self.render_pile_label(buf, *area, &format!("F{}", i + 1));
            let pile_area = Rect { x: area.x, y: area.y + 1, ..*area };
            PileRenderer::render(pile, buf, pile_area);
        }
    }

    fn render_tableau(&self, buf: &mut Buffer, tableau_areas: &[Rect]) {
        for (i, (pile, area)) in self.board.tableau.iter().zip(tableau_areas).enumerate() {
            self.render_pile_label(buf, *area, &format!("T{}", i + 1));
            let pile_area = Rect { x: area.x, y: area.y + 1, ..*area };
            PileRenderer::render(pile, buf, pile_area);
        }
    }
}

impl<'a> Widget for BoardRenderer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Create main vertical layout
        let vertical_sections = Layout::vertical([
            Constraint::Length(1),                          // Title
            Constraint::Length(1),                          // Spacing
            Constraint::Length(CardRenderer::HEIGHT + 1),   // Stock/Waste/Foundation + labels
            Constraint::Length(2),                          // Spacing
            Constraint::Min(CardRenderer::HEIGHT + 1),      // Tableau (grows as needed)
            // Constraint::Length(1),                          // Instructions
        ])
        .flex(Flex::Center)
        .split(area);

        // Render title
        self.render_centered_text(
            buf,
            vertical_sections[0],
            "=== Solitaire ===",
            Style::default().fg(Color::Yellow),
        );

        // Create horizontal layout for stock/waste/foundation row
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
        .split(vertical_sections[2]);

        self.render_stock_and_waste(buf, top_row_sections[0], top_row_sections[1]);
        self.render_foundations(buf, &top_row_sections[3..7]);

        // Create horizontal layout for tableau row
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
        .split(vertical_sections[4]);

        // Render tableau piles
        self.render_tableau(buf, &tableau_sections);
    }
}

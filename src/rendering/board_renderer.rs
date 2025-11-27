use crate::game_objects::Board;
use crate::rendering::{card_renderer::CardRenderer, pile_renderer::PileRenderer};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

pub struct BoardRenderer<'a> {
    board: &'a Board,
}

impl<'a> BoardRenderer<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self { board }
    }

    const HORIZONTAL_SPACING: u16 = 2;
    const VERTICAL_SPACING: u16 = 2;

    fn render_pile_label(&self, buf: &mut Buffer, x: u16, y: u16, label: &str) {
        if y < buf.area.height {
            buf.set_string(x, y, label, Style::default().fg(Color::Gray));
        }
    }

    fn render_stock_and_waste(&self, buf: &mut Buffer, start_y: u16) {
        let stock_x = 2;
        let waste_x = stock_x + CardRenderer::WIDTH + Self::HORIZONTAL_SPACING;

        // Stock pile
        self.render_pile_label(buf, stock_x, start_y, "Stock");
        PileRenderer::render(&self.board.stock, buf, stock_x, start_y + 1);

        // Waste pile
        self.render_pile_label(buf, waste_x, start_y, "Waste");
        PileRenderer::render(&self.board.waste, buf, waste_x, start_y + 1);
    }

    fn render_foundations(&self, buf: &mut Buffer, start_y: u16) {
        let start_x = 30;

        for (i, pile) in self.board.foundation.iter().enumerate() {
            let x = start_x + (i as u16) * (CardRenderer::WIDTH + Self::HORIZONTAL_SPACING);

            self.render_pile_label(buf, x, start_y, &format!("F{}", i + 1));
            PileRenderer::render(pile, buf, x, start_y + 1);
        }
    }

    fn render_tableau(&self, buf: &mut Buffer, start_y: u16) {
        let start_x = 2;

        for (i, pile) in self.board.tableau.iter().enumerate() {
            let x = start_x + (i as u16) * (CardRenderer::WIDTH + Self::HORIZONTAL_SPACING);

            self.render_pile_label(buf, x, start_y, &format!("T{}", i + 1));
            PileRenderer::render(pile, buf, x, start_y + 1);
        }
    }
}

impl<'a> Widget for BoardRenderer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Title
        let title = "=== Solitaire ===";
        if area.width >= title.len() as u16 {
            let title_x = (area.width - title.len() as u16) / 2;
            buf.set_string(area.x + title_x, area.y, title, Style::default().fg(Color::Yellow));
        }

        // Stock and Waste (top left)
        let stock_waste_y = area.y + 2;
        self.render_stock_and_waste(buf, stock_waste_y);

        // Foundation piles (top right)
        self.render_foundations(buf, stock_waste_y);

        // Tableau piles (below)
        let tableau_y = stock_waste_y + CardRenderer::HEIGHT + Self::VERTICAL_SPACING + 1;
        self.render_tableau(buf, tableau_y);

        // User Instructions
        let instructions = "Press Ctrl+C to quit";
        let instr_y = area.height.saturating_sub(1);
        if area.width >= instructions.len() as u16 {
            let instr_x = (area.width - instructions.len() as u16) / 2;
            buf.set_string(area.x + instr_x, instr_y, instructions, Style::default().fg(Color::DarkGray));
        }
    }
}

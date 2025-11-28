use crate::game_objects::{Board, Selection};
use super::board_renderer::BoardRenderer;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    widgets::Widget,
};

pub struct GameRenderer<'a> {
    board: &'a Board,
    selection: &'a Selection,
}

impl<'a> GameRenderer<'a> {
    pub fn new(board: &'a Board, selection: &'a Selection) -> Self {
        Self { board, selection }
    }

    fn render_centered_text(&self, buf: &mut Buffer, area: Rect, text: &str, style: Style) {
        if area.width >= text.len() as u16 {
            let x = area.x + (area.width - text.len() as u16) / 2;
            let y = area.y;
            buf.set_string(x, y, text, style);
        }
    }
}

impl<'a> Widget for GameRenderer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical_sections = Layout::vertical([
            Constraint::Length(1),  // Title
            Constraint::Length(1),  // Spacing
            Constraint::Min(0),     // Board area (grows as needed)
        ])
        .flex(Flex::Center)
        .split(area);

        self.render_centered_text(
            buf,
            vertical_sections[0],
            "=== Solitaire ===",
            Style::default().fg(Color::Yellow),
        );

        BoardRenderer::render(self.board, self.selection, buf, vertical_sections[2]);
    }
}

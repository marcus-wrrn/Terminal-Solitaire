use crate::rendering::RenderingInstructions;
use crate::game_objects::{Board, Selection};
use super::board_renderer::BoardRenderer;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
};

pub struct GameRenderer {
    board_renderer: BoardRenderer
}

impl GameRenderer {
    pub fn new() -> Self {
        Self {
            board_renderer: BoardRenderer::new(),
        }
    }

    pub fn coordinate_to_selection(&self, board: &Board, x: u16, y: u16) -> Option<Selection> {
        self.board_renderer.coordinate_to_selection(board, x, y)
    }

    fn render_centered_text(&self, buf: &mut Buffer, area: Rect, text: &str, style: Style) {
        if area.width >= text.len() as u16 {
            let x = area.x + (area.width - text.len() as u16) / 2;
            let y = area.y;
            buf.set_string(x, y, text, style);
        }
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer, render_instr: &RenderingInstructions) {
        let vertical_sections = Layout::vertical([
            Constraint::Length(1),  // Title
            Constraint::Length(1),  // Spacing
            Constraint::Min(0),     // Board area (grows as needed)
            Constraint::Length(8), // Debug log area
        ])
        .flex(Flex::Center)
        .split(area);

        self.render_centered_text(
            buf,
            vertical_sections[0],
            "=== Solitaire ===",
            Style::default().fg(Color::Yellow),
        );

        self.board_renderer.render(&render_instr.board_rendering_instr, buf, vertical_sections[2]);

        if render_instr.debug_log.visible {
            render_instr.debug_log.render(vertical_sections[3], buf);
        }
    }
}

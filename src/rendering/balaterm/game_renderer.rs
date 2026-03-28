use crate::rendering::balaterm::board_renderer::BalatermBoardRenderer;
use crate::rendering::balaterm::render_instructions::BalatermRenderingInstructions;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
};

pub struct BalatermGameRenderer {
    board_renderer: BalatermBoardRenderer,
}

impl BalatermGameRenderer {
    pub fn new() -> Self {
        Self {
            board_renderer: BalatermBoardRenderer::new(),
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer, instr: &BalatermRenderingInstructions) {
        let vertical_sections = Layout::vertical([
            Constraint::Length(1),  // Title
            Constraint::Length(1),  // Spacing
            Constraint::Min(0),     // Board area
            Constraint::Length(8),  // Debug log area
        ])
        .flex(Flex::Center)
        .split(area);

        self.render_centered_text(
            buf,
            vertical_sections[0],
            instr.title,
            Style::default().fg(Color::Yellow),
        );

        self.board_renderer.render(
            instr.hands,
            instr.selected_hand,
            instr.selected_slot,
            buf,
            vertical_sections[2],
        );

        if instr.debug_log.visible {
            instr.debug_log.render(vertical_sections[3], buf);
        }
    }

    fn render_centered_text(&self, buf: &mut Buffer, area: Rect, text: &str, style: Style) {
        if area.width >= text.len() as u16 {
            let x = area.x + (area.width - text.len() as u16) / 2;
            buf.set_string(x, area.y, text, style);
        }
    }
}

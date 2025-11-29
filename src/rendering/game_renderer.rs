use crate::game_objects::{Board, Selection};
use crate::ui::DebugLog;
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
    hover_selection: Option<&'a Selection>,
    debug_log: &'a DebugLog,
    board_renderer: &'a mut BoardRenderer,
}

impl<'a> GameRenderer<'a> {
    pub fn new(
        board: &'a Board,
        selection: &'a Selection,
        debug_log: &'a DebugLog,
        board_renderer: &'a mut BoardRenderer
    ) -> Self {
        Self {
            board,
            selection,
            hover_selection: None,
            debug_log,
            board_renderer,
        }
    }

    pub fn with_hover(
        board: &'a Board,
        selection: &'a Selection,
        hover_selection: Option<&'a Selection>,
        debug_log: &'a DebugLog,
        board_renderer: &'a mut BoardRenderer
    ) -> Self {
        Self {
            board,
            selection,
            hover_selection,
            debug_log,
            board_renderer,
        }
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
            Constraint::Length(12), // Debug log area
        ])
        .flex(Flex::Center)
        .split(area);

        self.render_centered_text(
            buf,
            vertical_sections[0],
            "=== Solitaire ===",
            Style::default().fg(Color::Yellow),
        );

        self.board_renderer.render(self.board, self.selection, self.hover_selection, buf, vertical_sections[2]);

        self.debug_log.render(vertical_sections[3], buf);
    }
}

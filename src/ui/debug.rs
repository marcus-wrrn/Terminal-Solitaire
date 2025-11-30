use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

pub struct DebugLog {
    messages: Vec<String>,
    max_lines: usize,
}

impl DebugLog {
    pub fn new(max_lines: usize) -> Self {
        Self {
            messages: Vec::new(),
            max_lines,
        }
    }

    pub fn log(&mut self, message: impl Into<String>) {
        self.messages.push(message.into());

        if self.messages.len() > self.max_lines {
            self.messages.remove(0);
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let lines: Vec<Line> = self
            .messages
            .iter()
            .map(|msg| Line::from(msg.clone()))
            .collect();

        let text = Text::from(lines);

        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Debug Log")
                    .style(Style::default().fg(Color::Cyan))
            )
            .style(Style::default().fg(Color::White));

        paragraph.render(area, buf);
    }
}

impl Default for DebugLog {
    fn default() -> Self {
        Self::new(10)
    }
}

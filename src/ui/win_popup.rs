use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

pub struct WinPopup {
    is_visible: bool,
    title: Option<String>,
    description: Vec<Line<'static>>
}

impl WinPopup {
    pub fn new_with_title(title: String) -> Self {
        Self {
            is_visible: false,
            title: Some(title),
            description: Vec::new()
        }
    }

    pub fn new() -> Self {
        Self {
            is_visible: false,
            title: None,
            description: Vec::new()
        }
    }

    pub fn add_line(&mut self, line: Line<'static>) {
        self.description.push(line);
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if !self.is_visible {
            return;
        }

        let popup_area = Self::centered_rect(40, 30, area);

        Clear.render(popup_area, buf);

        let mut lines = vec![Line::from("")];

        if let Some(title) = &self.title {
            lines.push(Line::from(vec![
                Span::styled(
                    title,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::from(""));
        } else {
            lines.push(Line::from(vec![
                Span::styled(
                    "CONGRATULATIONS!",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::from(""));
        }

        if !self.description.is_empty() {
            for line in &self.description {
                lines.push(line.clone());
            }
            lines.push(Line::from(""));
        } else {
            lines.push(Line::from(vec![
                Span::styled(
                    "YOU WON!",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::from(""));
        }

        // lines.push(Line::from(vec![
        //     Span::styled(
        //         "Press R to restart",
        //         Style::default().fg(Color::White),
        //     ),
        // ]));
        // lines.push(Line::from(vec![
        //     Span::styled(
        //         "Press Q to quit",
        //         Style::default().fg(Color::White),
        //     ),
        // ]));

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Victory!")
                    .style(Style::default().fg(Color::Cyan))
            )
            .centered();

        paragraph.render(popup_area, buf);
    }

    fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
        let popup_layout = Layout::vertical([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .flex(Flex::Center)
        .split(area);

        Layout::horizontal([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .flex(Flex::Center)
        .split(popup_layout[1])[1]
    }
}

impl Default for WinPopup {
    fn default() -> Self {
        Self::new()
    }
}

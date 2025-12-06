use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget, BorderType},
};

pub struct VictoryScreen {
    is_visible: bool,
}

impl VictoryScreen {
    pub fn new() -> Self {
        Self {
            is_visible: false,
        }
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    // pub fn hide(&mut self) {
    //     self.is_visible = false;
    // }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if !self.is_visible {
            return;
        }

        let popup_area = Self::centered_rect(50, 25, area);

        Clear.render(popup_area, buf);

        let victory_text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "VICTORY!",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "You won the game!",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ];

        let victory_paragraph = Paragraph::new(victory_text)
            .centered();

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title_bottom(Line::from(" Press R to restart ").left_aligned())
            .title_bottom(Line::from(" Press Q to quit ").right_aligned().style(Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::LightYellow));

        let inner_area = block.inner(popup_area);
        let inner_layout = Layout::vertical([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(inner_area);

        block.render(popup_area, buf);

        victory_paragraph.render(inner_layout[0], buf);
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

impl Default for VictoryScreen {
    fn default() -> Self {
        Self::new()
    }
}

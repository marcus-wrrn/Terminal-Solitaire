use crate::resources::FIGLET_3D_FONT;
use crate::ui::figlet::FIGfont;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget},
};
use unicode_width::UnicodeWidthStr;

pub struct VictoryPop {
    is_visible: bool,
}

impl VictoryPop {
    pub fn new() -> Self {
        Self { is_visible: false }
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

        let font = FIGfont::from_content(FIGLET_3D_FONT).unwrap();
        let congratulations = font.convert("CONGRATULATIONS").unwrap();
        let you_won = font.convert("YOU WON!").unwrap();

        let congrats_text = congratulations.to_string();
        let win_text = you_won.to_string();

        let congrats_width = congrats_text
            .lines()
            .map(|l| l.trim_end().width())
            .max()
            .unwrap_or(0) as u16;

        let test_popup_area = Self::centered_rect(90, 10, area);
        let test_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick);
        let available_width = test_block.inner(test_popup_area).width;

        let mut victory_text: Vec<Line> = Vec::new();

        if congrats_width <= available_width {
            for line in congrats_text.lines() {
                victory_text.push(Line::from(Span::styled(
                    line,
                    Style::default().fg(Color::Yellow),
                )));
            }

            victory_text.push(Line::from(""));
        }

        for line in win_text.lines() {
            victory_text.push(Line::from(Span::styled(
                line,
                Style::default().fg(Color::Yellow),
            )));
        }

        let text_height = victory_text.len() as u16;
        let needed_height = text_height + 4;
        let height_percent = ((needed_height * 100) / area.height).min(90);

        let popup_area = Self::centered_rect(90, height_percent, area);

        Clear.render(popup_area, buf);

        let victory_paragraph = Paragraph::new(victory_text).centered();

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title_bottom(Line::from(" Press R to restart ").left_aligned())
            .title_bottom(
                Line::from(" Press Q to quit ")
                    .right_aligned()
                    .style(Style::default().fg(Color::Red)),
            )
            .style(Style::default().fg(Color::LightYellow));

        let inner_area = block.inner(popup_area);

        block.render(popup_area, buf);

        victory_paragraph.render(inner_area, buf);
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

impl Default for VictoryPop {
    fn default() -> Self {
        Self::new()
    }
}

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget, BorderType},
};
use std::rc::Rc;
use crate::controller::KeyBindings;

pub struct StartupPop {
    is_visible: bool,
    key_binds: Rc<KeyBindings>
}

impl StartupPop {
    pub fn new(key_bindings: Rc<KeyBindings>) -> Self {
        Self {
            is_visible: false,
            key_binds: key_bindings
        }
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

        let popup_area = Self::centered_rect(70, 80, area);

        Clear.render(popup_area, buf);

        let bindings = &self.key_binds;

        let key_style = Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);
        let label_style = Style::default().fg(Color::White);
        let header_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);

        let help_text = vec![
            Line::from(vec![
                Span::styled("Welcome to Solitaire!", header_style),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Movement", header_style),
            ]),
            Line::from(vec![
                Span::styled(format!("{:?}", bindings.move_left), key_style),
                Span::styled(" / ", label_style),
                Span::styled(format!("{:?}", bindings.move_right), key_style),
                Span::styled(" / ", label_style),
                Span::styled(format!("{:?}", bindings.move_up), key_style),
                Span::styled(" / ", label_style),
                Span::styled(format!("{:?}", bindings.move_down), key_style),
                Span::styled(" - Move selection", label_style),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Focus Areas", header_style),
            ]),
            Line::from(vec![
                Span::styled(format!("{:?}", bindings.focus_tableau), key_style),
                Span::styled(" - Focus Tableau", label_style),
            ]),
            Line::from(vec![
                Span::styled(format!("{:?}", bindings.focus_foundation), key_style),
                Span::styled(" - Focus Foundation", label_style),
            ]),
            Line::from(vec![
                Span::styled(format!("{:?}", bindings.focus_stock), key_style),
                Span::styled(" - Focus Stock", label_style),
            ]),
            Line::from(vec![
                Span::styled(format!("{:?}", bindings.focus_waste), key_style),
                Span::styled(" - Focus Waste", label_style),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Actions", header_style),
            ]),
            Line::from(vec![
                Span::styled(format!("{:?}", bindings.select), key_style),
                Span::styled(" - Select card", label_style),
            ]),
            Line::from(vec![
                Span::styled(format!("{:?}", bindings.enter), key_style),
                Span::styled(" - Confirm action", label_style),
            ]),
            Line::from(vec![
                Span::styled(format!("{:?}", bindings.cancel), key_style),
                Span::styled(" - Cancel selection", label_style),
            ]),
            Line::from(vec![
                Span::styled(format!("{:?}", bindings.draw_stock), key_style),
                Span::styled(" - Draw from stock", label_style),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Game Controls", header_style),
            ]),
            Line::from(vec![
                Span::styled(format!("{:?}", bindings.undo), key_style),
                Span::styled(" - Undo last move", label_style),
            ]),
            Line::from(vec![
                Span::styled(format!("{:?}", bindings.restart), key_style),
                Span::styled(" - Restart game", label_style),
            ]),
            Line::from(vec![
                Span::styled(format!("{:?}", bindings.open_menu), key_style),
                Span::styled(" - Open menu", label_style),
            ]),
            Line::from(vec![
                Span::styled(format!("{:?}", bindings.options_menu), key_style),
                Span::styled(" - Options menu", label_style),
            ]),
            Line::from(vec![
                Span::styled(format!("{:?}", bindings.quit), key_style),
                Span::styled(" - Quit game", label_style),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Mouse controls are also supported!", Style::default().fg(Color::Green)),
            ]),
        ];

        let help_paragraph = Paragraph::new(help_text);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title_top(Line::from(" Game Controls ").centered())
            .title_bottom(Line::from(" Press any key to start ").centered())
            .style(Style::default().fg(Color::LightYellow));

        let inner_area = block.inner(popup_area);

        block.render(popup_area, buf);
        help_paragraph.render(inner_area, buf);
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
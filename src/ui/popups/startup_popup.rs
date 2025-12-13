use ratatui::{
    buffer::Buffer,
    layout::{self, Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Widget},
};
use std::rc::Rc;
use std::cell::RefCell;
use crate::controller::KeyBindings;

pub struct StartupPop {
    is_visible: bool,
    key_binds: Rc<RefCell<KeyBindings>>
}

impl StartupPop {
    pub fn new(key_bindings: Rc<RefCell<KeyBindings>>) -> Self {
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

        let popup_area = Self::centered_rect(40, 50, area);

        Clear.render(popup_area, buf);

        let bindings = self.key_binds.borrow();

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
                Span::styled(bindings.move_left_str(), key_style),
                Span::styled(" / ", label_style),
                Span::styled(bindings.move_right_str(), key_style),
                Span::styled(" / ", label_style),
                Span::styled(bindings.move_up_str(), key_style),
                Span::styled(" / ", label_style),
                Span::styled(bindings.move_down_str(), key_style),
                Span::styled(" - For moving around", label_style),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Actions", header_style),
            ]),
            Line::from(vec![
                Span::styled(format!("{}/{}", bindings.select_str(), bindings.enter_str()), key_style),
                Span::styled(" - Select card", label_style),
            ]),
            Line::from(vec![
                Span::styled(bindings.cancel_str(), key_style),
                Span::styled(" - Cancel selection", label_style),
            ]),
            Line::from(vec![
                Span::styled(bindings.draw_stock_str(), key_style),
                Span::styled(" - Draw from stock", label_style),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Game Controls", header_style),
            ]),
            Line::from(vec![
                Span::styled(bindings.undo_str(), key_style),
                Span::styled(" - Undo last move", label_style),
            ]),
            Line::from(vec![
                Span::styled(bindings.restart_str(), key_style),
                Span::styled(" - Restart Your game", label_style),
            ]),
            Line::from(vec![
                Span::styled(bindings.options_menu_str(), key_style),
                Span::styled(" - Options menu", label_style),
            ]),
            Line::from(vec![
                Span::styled(bindings.quit_str(), key_style),
                Span::styled(" - Quit game", label_style),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Mouse controls are also supported!", Style::default().fg(Color::Green)),
            ]),
        ];

        let help_paragraph = Paragraph::new(help_text).alignment(layout::Alignment::Center);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title_top(Line::from(" Game Controls ").left_aligned())
            .title_bottom(Line::from(" Press any key to start ").right_aligned())
            .padding(Padding::vertical(2))
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
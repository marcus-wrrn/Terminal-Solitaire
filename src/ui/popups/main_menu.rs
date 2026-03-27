use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph, Widget},
};
use unicode_width::UnicodeWidthStr;
use crate::ui::figlet::FIGfont;
use crate::resources::FIGLET_3D_FONT;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MainMenuOption {
    Play,
    Settings,
    Quit,
}

impl MainMenuOption {
    fn all() -> Vec<MainMenuOption> {
        vec![
            MainMenuOption::Play,
            MainMenuOption::Settings,
            MainMenuOption::Quit,
        ]
    }

    fn label(&self) -> &str {
        match self {
            MainMenuOption::Play => "Start Game",
            MainMenuOption::Settings => "Settings",
            MainMenuOption::Quit => "Quit",
        }
    }
}

pub struct MainMenu {
    selected_index: usize,
    is_visible: bool,
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            is_visible: false,
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

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        let options = MainMenuOption::all();
        if self.selected_index < options.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn selected_option(&self) -> MainMenuOption {
        MainMenuOption::all()[self.selected_index]
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if !self.is_visible {
            return;
        }

        Clear.render(area, buf);

        let [left, right] = Layout::horizontal([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .areas(area);

        self.render_title(left, buf);
        self.render_options(right, buf);
    }

    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        let font = FIGfont::from_content(FIGLET_3D_FONT).unwrap();
        let menu_text = font.convert("").unwrap().to_string();

        let text_width = menu_text
            .lines()
            .map(|l| l.trim_end().width())
            .max()
            .unwrap_or(0) as u16;

        let lines: Vec<Line> = menu_text
            .lines()
            .map(|l| {
                Line::from(Span::styled(
                    l.to_string(),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
            })
            .collect();

        let text_height = lines.len() as u16;

        let title_paragraph = Paragraph::new(lines).centered();

        // Center the figlet text vertically and horizontally in the left panel
        let h_margin = area.width.saturating_sub(text_width) / 2;
        let v_margin = area.height.saturating_sub(text_height) / 2;

        let centered = Rect {
            x: area.x + h_margin,
            y: area.y + v_margin,
            width: area.width.saturating_sub(h_margin * 2),
            height: text_height.min(area.height),
        };

        title_paragraph.render(centered, buf);
    }

    fn render_options(&self, area: Rect, buf: &mut Buffer) {
        let options = MainMenuOption::all();
        let items: Vec<ListItem> = options
            .iter()
            .enumerate()
            .map(|(i, option)| {
                let style = if i == self.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if i == self.selected_index { "> " } else { "  " };
                let line = Line::from(vec![
                    Span::raw(prefix),
                    Span::styled(option.label(), style),
                ]);

                ListItem::new(line)
            })
            .collect();

        // Vertically center the list in the right panel
        let list_height = (options.len() as u16) + 4; // items + block padding
        let [_, center, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(list_height),
            Constraint::Fill(1),
        ])
        .flex(Flex::Center)
        .areas(area);

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::LEFT)
                .border_type(BorderType::Thick)
                .padding(Padding::new(4, 2, 1, 1))
                .style(Style::default().fg(Color::DarkGray)),
        );

        list.render(center, buf);
    }
}

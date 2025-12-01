use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Widget},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuOption {
    Restart,
    RebindKeys,
    DeveloperMode,
    Help,
}

impl MenuOption {
    fn all() -> Vec<MenuOption> {
        vec![
            MenuOption::Restart,
            MenuOption::RebindKeys,
            MenuOption::Help,
            MenuOption::DeveloperMode,
        ]
    }

    fn label(&self) -> &str {
        match self {
            MenuOption::Restart => "Restart Game",
            MenuOption::RebindKeys => "Rebind Keys",
            MenuOption::DeveloperMode => "Developer Mode",
            MenuOption::Help => "Help",
        }
    }
}

pub struct OptionsMenu {
    selected_index: usize,
    is_visible: bool,
}

impl OptionsMenu {
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

    pub fn toggle(&mut self) {
        self.is_visible = !self.is_visible;
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
        let options = MenuOption::all();
        if self.selected_index < options.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn selected_option(&self) -> MenuOption {
        MenuOption::all()[self.selected_index]
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if !self.is_visible {
            return;
        }

        let popup_area = Self::centered_rect(50, 40, area);

        Clear.render(popup_area, buf);

        let options = MenuOption::all();
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

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Options Menu")
                    .style(Style::default().fg(Color::Cyan))
            );

        list.render(popup_area, buf);
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

impl Default for OptionsMenu {
    fn default() -> Self {
        Self::new()
    }
}

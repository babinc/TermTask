use crate::models::{ColorTheme, DateFormat};
use crate::ui::styling::{ThemeStyles, ThemeColors};
use super::multi_select::MultiSelect;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

#[derive(Debug, Clone)]
pub enum SettingItem {
    Theme(ColorTheme),
    DateFormat(DateFormat),
    VimMode(bool),
    CompactMode(bool),
}

impl SettingItem {
    fn display(&self) -> String {
        match self {
            SettingItem::Theme(theme) => format!("Theme: {}", theme.name()),
            SettingItem::DateFormat(format) => format!("Date Format: {}", format.name()),
            SettingItem::VimMode(enabled) => format!("Vim Mode: {}", if *enabled { "Enabled" } else { "Disabled" }),
            SettingItem::CompactMode(enabled) => format!("Compact Mode: {}", if *enabled { "Enabled" } else { "Disabled" }),
        }
    }
}

pub struct SettingsModal {
    pub active: bool,
    pub selected_index: usize,
    settings: Vec<SettingItem>,
    theme_selector: MultiSelect<ColorTheme>,
    date_format_selector: MultiSelect<DateFormat>,
    pub multi_select_active: bool,
}

impl SettingsModal {
    pub fn new() -> Self {
        let default_theme = ColorTheme::default();
        let default_date_format = DateFormat::default();
        Self {
            active: false,
            selected_index: 0,
            settings: vec![
                SettingItem::Theme(default_theme.clone()),
                SettingItem::DateFormat(default_date_format.clone()),
                SettingItem::VimMode(false),
                SettingItem::CompactMode(false),
            ],
            theme_selector: MultiSelect::new("Select Theme".to_string(), default_theme),
            date_format_selector: MultiSelect::new("Select Date Format".to_string(), default_date_format),
            multi_select_active: false,
        }
    }

    pub fn open(&mut self, current_theme: &ColorTheme, date_format: &DateFormat, vim_mode: bool, compact_mode: bool) {
        self.active = true;
        self.selected_index = 0;
        self.multi_select_active = false;
        self.settings = vec![
            SettingItem::Theme(current_theme.clone()),
            SettingItem::DateFormat(date_format.clone()),
            SettingItem::VimMode(vim_mode),
            SettingItem::CompactMode(compact_mode),
        ];
        self.theme_selector = MultiSelect::new("Select Theme".to_string(), current_theme.clone());
        self.date_format_selector = MultiSelect::new("Select Date Format".to_string(), date_format.clone());
    }

    pub fn close(&mut self) {
        self.active = false;
        self.multi_select_active = false;
        self.theme_selector.close();
        self.date_format_selector.close();
    }

    pub fn next_item(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.settings.len();
    }

    pub fn previous_item(&mut self) {
        self.selected_index = if self.selected_index == 0 {
            self.settings.len() - 1
        } else {
            self.selected_index - 1
        };
    }

    pub fn toggle_selected(&mut self) {
        match &mut self.settings[self.selected_index] {
            SettingItem::Theme(_) => {
                self.multi_select_active = true;
                self.theme_selector.open();
            }
            SettingItem::DateFormat(_) => {
                self.multi_select_active = true;
                self.date_format_selector.open();
            }
            SettingItem::VimMode(enabled) => {
                *enabled = !*enabled;
            }
            SettingItem::CompactMode(enabled) => {
                *enabled = !*enabled;
            }
        }
    }

    pub fn handle_multi_select_input(&mut self, key: crossterm::event::KeyCode) -> bool {
        if !self.multi_select_active {
            return false;
        }

        match key {
            crossterm::event::KeyCode::Esc => {
                self.multi_select_active = false;
                self.theme_selector.close();
                self.date_format_selector.close();
                true
            }
            crossterm::event::KeyCode::Enter => {
                if self.theme_selector.active {
                    let new_theme = self.theme_selector.select_current();
                    self.settings[self.selected_index] = SettingItem::Theme(new_theme);
                } else if self.date_format_selector.active {
                    let new_format = self.date_format_selector.select_current();
                    self.settings[self.selected_index] = SettingItem::DateFormat(new_format);
                }
                self.multi_select_active = false;
                true
            }
            crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                if self.theme_selector.active {
                    self.theme_selector.previous_item();
                } else if self.date_format_selector.active {
                    self.date_format_selector.previous_item();
                }
                true
            }
            crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                if self.theme_selector.active {
                    self.theme_selector.next_item();
                } else if self.date_format_selector.active {
                    self.date_format_selector.next_item();
                }
                true
            }
            crossterm::event::KeyCode::Tab => {
                if self.theme_selector.active {
                    self.theme_selector.next_item();
                } else if self.date_format_selector.active {
                    self.date_format_selector.next_item();
                }
                true
            }
            crossterm::event::KeyCode::BackTab => {
                if self.theme_selector.active {
                    self.theme_selector.previous_item();
                } else if self.date_format_selector.active {
                    self.date_format_selector.previous_item();
                }
                true
            }
            _ => false,
        }
    }

    pub fn get_theme(&self) -> ColorTheme {
        for setting in &self.settings {
            if let SettingItem::Theme(theme) = setting {
                return theme.clone();
            }
        }
        ColorTheme::CatppuccinMocha
    }

    pub fn get_vim_mode(&self) -> bool {
        for setting in &self.settings {
            if let SettingItem::VimMode(enabled) = setting {
                return *enabled;
            }
        }
        false
    }

    pub fn get_date_format(&self) -> DateFormat {
        for setting in &self.settings {
            if let SettingItem::DateFormat(format) = setting {
                return format.clone();
            }
        }
        DateFormat::default()
    }

    pub fn get_compact_mode(&self) -> bool {
        for setting in &self.settings {
            if let SettingItem::CompactMode(enabled) = setting {
                return *enabled;
            }
        }
        false
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, styles: &ThemeStyles, colors: &ThemeColors) {
        if !self.active {
            return;
        }

        let width = (area.width as f32 * 0.8) as u16;
        let height = (area.height as f32 * 0.6) as u16;
        let popup_area = Rect {
            x: (area.width - width) / 2,
            y: (area.height - height) / 3,
            width,
            height,
        };

        frame.render_widget(Clear, popup_area);

        let modal_bg = Block::default()
            .style(Style::default().bg(colors.modal_bg));
        frame.render_widget(modal_bg, popup_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(1),
            ])
            .split(popup_area);

        let prompt_block = Block::default()
            .title(" Settings ")
            .borders(Borders::ALL)
            .border_set(ratatui::symbols::border::Set {
                top_left: "┌",
                top_right: "┐",
                bottom_right: "┤",
                bottom_left: "├",
                vertical_left: "│",
                vertical_right: "│",
                horizontal_top: "─",
                horizontal_bottom: "─",
            })
            .border_style(Style::default().fg(colors.modal_border))
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg));

        let prompt_input = Paragraph::new("> Configure application settings")
            .block(prompt_block)
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg));

        frame.render_widget(prompt_input, chunks[0]);

        let mut settings_display = String::new();
        for (i, setting) in self.settings.iter().enumerate() {
            let prefix = if i == self.selected_index { "  ► " } else { "    " };
            settings_display.push_str(&format!("{}{}\n", prefix, setting.display()));
        }

        let results_block = Block::default()
            .title(" Options ")
            .borders(Borders::ALL)
            .border_set(ratatui::symbols::border::Set {
                top_left: "├",
                top_right: "┤",
                bottom_right: "┘",
                bottom_left: "└",
                vertical_left: "│",
                vertical_right: "│",
                horizontal_top: "─",
                horizontal_bottom: "─",
            })
            .border_style(Style::default().fg(colors.modal_border))
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg));

        let settings_paragraph = Paragraph::new(settings_display)
            .block(results_block)
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg))
            .wrap(Wrap { trim: false });

        frame.render_widget(settings_paragraph, chunks[1]);

        let help_text = "↑↓/jk Navigate | Enter/Space Toggle | Esc Close";
        let status_line = Paragraph::new(help_text)
            .style(Style::default().fg(colors.muted).bg(colors.modal_bg))
            .alignment(Alignment::Left);

        frame.render_widget(status_line, chunks[2]);

        if self.multi_select_active {
            self.theme_selector.render(frame, area, styles, colors);
            self.date_format_selector.render(frame, area, styles, colors);
        }
    }
}
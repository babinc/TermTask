use crate::models::{ColorTheme, DateFormat};
use crate::ui::themes::{ThemeStyles, ThemeColors};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
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
    themes: Vec<ColorTheme>,
    date_formats: Vec<DateFormat>,
}

impl SettingsModal {
    pub fn new() -> Self {
        let themes = ColorTheme::all();
        let date_formats = DateFormat::all();
        Self {
            active: false,
            selected_index: 0,
            settings: vec![
                SettingItem::Theme(themes[0].clone()),
                SettingItem::DateFormat(date_formats[0].clone()),
                SettingItem::VimMode(false),
                SettingItem::CompactMode(false),
            ],
            themes,
            date_formats,
        }
    }

    pub fn open(&mut self, current_theme: &ColorTheme, date_format: &DateFormat, vim_mode: bool, compact_mode: bool) {
        self.active = true;
        self.selected_index = 0;
        self.settings = vec![
            SettingItem::Theme(current_theme.clone()),
            SettingItem::DateFormat(date_format.clone()),
            SettingItem::VimMode(vim_mode),
            SettingItem::CompactMode(compact_mode),
        ];
    }

    pub fn close(&mut self) {
        self.active = false;
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
            SettingItem::Theme(current_theme) => {
                let current_index = self.themes
                    .iter()
                    .position(|t| t == current_theme)
                    .unwrap_or(0);
                let next_index = (current_index + 1) % self.themes.len();
                *current_theme = self.themes[next_index].clone();
            }
            SettingItem::DateFormat(current_format) => {
                let current_index = self.date_formats
                    .iter()
                    .position(|f| f == current_format)
                    .unwrap_or(0);
                let next_index = (current_index + 1) % self.date_formats.len();
                *current_format = self.date_formats[next_index].clone();
            }
            SettingItem::VimMode(enabled) => {
                *enabled = !*enabled;
            }
            SettingItem::CompactMode(enabled) => {
                *enabled = !*enabled;
            }
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
                Constraint::Length(3),   // Prompt section
                Constraint::Min(5),      // Results section
                Constraint::Length(1),   // Status line
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
            .style(styles.normal.bg(colors.modal_bg));

        let prompt_input = Paragraph::new("> Configure application settings")
            .block(prompt_block)
            .style(styles.normal);

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
            .style(styles.normal.bg(colors.modal_bg));

        let settings_paragraph = Paragraph::new(settings_display)
            .block(results_block)
            .style(styles.normal)
            .wrap(Wrap { trim: false });

        frame.render_widget(settings_paragraph, chunks[1]);

        let help_text = "↑↓/jk Navigate | Enter/Space Toggle | Esc Close";
        let status_line = Paragraph::new(help_text)
            .style(styles.muted)
            .alignment(Alignment::Left);

        frame.render_widget(status_line, chunks[2]);
    }
}
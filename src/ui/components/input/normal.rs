use crate::ui::themes::{ThemeStyles, ThemeColors};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Title,
    Description,
}

pub struct NormalInput {
    pub active: bool,
    pub title: String,
    pub description: Vec<String>,
    pub cursor_position: usize,
    pub mode: InputMode,
    pub current_line: usize,
    pub cursor_col: usize,
    pub is_editing: bool,
}

impl NormalInput {
    pub fn new() -> Self {
        Self {
            active: false,
            title: String::new(),
            description: vec![String::new()],
            cursor_position: 0,
            mode: InputMode::Title,
            current_line: 0,
            cursor_col: 0,
            is_editing: false,
        }
    }

    pub fn open(&mut self) {
        self.active = true;
        self.title.clear();
        self.description = vec![String::new()];
        self.cursor_position = 0;
        self.mode = InputMode::Title;
        self.current_line = 0;
        self.cursor_col = 0;
        self.is_editing = false;
    }

    pub fn open_with_data(&mut self, title: &str, description: Option<&str>) {
        self.active = true;
        self.title = title.to_string();
        self.cursor_position = title.len();
        
        if let Some(desc) = description {
            self.description = desc.lines().map(|s| s.to_string()).collect();
            if self.description.is_empty() {
                self.description = vec![String::new()];
            }
        } else {
            self.description = vec![String::new()];
        }
        
        self.mode = InputMode::Title;
        self.current_line = 0;
        self.cursor_col = 0;
        self.is_editing = true;
    }

    pub fn close(&mut self) {
        self.active = false;
    }

    pub fn handle_char(&mut self, c: char) {
        match self.mode {
            InputMode::Title => {
                self.title.insert(self.cursor_position, c);
                self.cursor_position += 1;
            }
            InputMode::Description => {
                let current_line = &mut self.description[self.current_line];
                current_line.insert(self.cursor_col, c);
                self.cursor_col += 1;
            }
        }
    }

    pub fn handle_backspace(&mut self) {
        match self.mode {
            InputMode::Title => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                    self.title.remove(self.cursor_position);
                }
            }
            InputMode::Description => {
                if self.cursor_col > 0 {
                    let current_line = &mut self.description[self.current_line];
                    self.cursor_col -= 1;
                    current_line.remove(self.cursor_col);
                } else if self.current_line > 0 {
                    let current_content = self.description.remove(self.current_line);
                    self.current_line -= 1;
                    let prev_line_len = self.description[self.current_line].len();
                    self.description[self.current_line].push_str(&current_content);
                    self.cursor_col = prev_line_len;
                }
            }
        }
    }

    pub fn handle_enter(&mut self) -> bool {
        match self.mode {
            InputMode::Title => {
                self.mode = InputMode::Description;
                false
            }
            InputMode::Description => {
                let current_line = &mut self.description[self.current_line];
                let remaining = current_line.split_off(self.cursor_col);
                self.description.insert(self.current_line + 1, remaining);
                self.current_line += 1;
                self.cursor_col = 0;
                false
            }
        }
    }

    pub fn move_cursor_left(&mut self) {
        match self.mode {
            InputMode::Title => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }
            InputMode::Description => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                } else if self.current_line > 0 {
                    self.current_line -= 1;
                    self.cursor_col = self.description[self.current_line].len();
                }
            }
        }
    }

    pub fn move_cursor_right(&mut self) {
        match self.mode {
            InputMode::Title => {
                if self.cursor_position < self.title.len() {
                    self.cursor_position += 1;
                }
            }
            InputMode::Description => {
                if self.cursor_col < self.description[self.current_line].len() {
                    self.cursor_col += 1;
                } else if self.current_line < self.description.len() - 1 {
                    self.current_line += 1;
                    self.cursor_col = 0;
                }
            }
        }
    }

    pub fn move_cursor_up(&mut self) {
        if matches!(self.mode, InputMode::Description) && self.current_line > 0 {
            self.current_line -= 1;
            self.cursor_col = self.cursor_col.min(self.description[self.current_line].len());
        }
    }

    pub fn move_cursor_down(&mut self) {
        if matches!(self.mode, InputMode::Description) && self.current_line < self.description.len() - 1 {
            self.current_line += 1;
            self.cursor_col = self.cursor_col.min(self.description[self.current_line].len());
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_description(&self) -> Option<String> {
        let desc = self.description.join("\n").trim().to_string();
        if desc.is_empty() {
            None
        } else {
            Some(desc)
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, styles: &ThemeStyles, colors: &ThemeColors) {
        if !self.active {
            return;
        }

        // Telescope-style dimensions and positioning
        let width = (area.width as f32 * 0.8) as u16;
        let height = (area.height as f32 * 0.6) as u16;
        let popup_area = Rect {
            x: (area.width - width) / 2,
            y: (area.height - height) / 3,
            width,
            height,
        };
        
        frame.render_widget(Clear, popup_area);
        
        // Render background for the entire modal
        let modal_bg = Block::default()
            .style(Style::default().bg(colors.modal_bg));
        frame.render_widget(modal_bg, popup_area);

        // Telescope-style layout: Prompt at top, then results/input area
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),   // Prompt section (title input)
                Constraint::Min(5),      // Results section (description)
                Constraint::Length(1),   // Status line
            ])
            .split(popup_area);

        // Telescope-style prompt section
        let prompt_text = if matches!(self.mode, InputMode::Title) {
            let mut display_title = self.title.clone();
            display_title.insert(self.cursor_position, '_');
            format!("> {}", display_title)
        } else {
            format!("> {}", self.title)
        };

        let prompt_border_chars = if matches!(self.mode, InputMode::Title) {
            ["─", "│", "─", "│", "┌", "┐", "┘", "└"]
        } else {
            ["─", "│", "─", "│", "├", "┤", "┤", "├"]
        };

        let prompt_block = Block::default()
            .title(" Title ")
            .borders(Borders::ALL)
            .border_set(ratatui::symbols::border::Set {
                top_left: prompt_border_chars[4],
                top_right: prompt_border_chars[5],
                bottom_right: prompt_border_chars[6],
                bottom_left: prompt_border_chars[7],
                vertical_left: prompt_border_chars[1],
                vertical_right: prompt_border_chars[1],
                horizontal_top: prompt_border_chars[0],
                horizontal_bottom: prompt_border_chars[2],
            })
            .border_style(if matches!(self.mode, InputMode::Title) {
                Style::default().fg(colors.modal_border)
            } else {
                Style::default().fg(colors.border)
            })
            .style(styles.normal.bg(colors.modal_bg));

        let prompt_input = Paragraph::new(prompt_text)
            .block(prompt_block)
            .style(styles.normal);

        frame.render_widget(prompt_input, chunks[0]);

        // Telescope-style results/description section
        let mut description_display = String::new();
        for (i, line) in self.description.iter().enumerate() {
            if i == self.current_line && matches!(self.mode, InputMode::Description) {
                let mut display_line = line.clone();
                display_line.insert(self.cursor_col.min(display_line.len()), '│');
                description_display.push_str(&format!("  {}\n", display_line));
            } else {
                description_display.push_str(&format!("  {}\n", line));
            }
        }
        
        // Add hint text if empty
        if self.description.len() == 1 && self.description[0].is_empty() && !matches!(self.mode, InputMode::Description) {
            description_display = "  Type description here (optional)...".to_string();
        }

        let results_border_chars = if matches!(self.mode, InputMode::Description) {
            ["─", "│", "─", "│", "├", "┤", "┘", "└"]
        } else {
            ["─", "│", "─", "│", "├", "┤", "┤", "├"]
        };

        let results_block = Block::default()
            .title(" Description ")
            .borders(Borders::ALL)
            .border_set(ratatui::symbols::border::Set {
                top_left: results_border_chars[4],
                top_right: results_border_chars[5],
                bottom_right: results_border_chars[6],
                bottom_left: results_border_chars[7],
                vertical_left: results_border_chars[1],
                vertical_right: results_border_chars[1],
                horizontal_top: results_border_chars[0],
                horizontal_bottom: results_border_chars[2],
            })
            .border_style(if matches!(self.mode, InputMode::Description) {
                Style::default().fg(colors.modal_border)
            } else {
                Style::default().fg(colors.border)
            })
            .style(styles.normal.bg(colors.modal_bg));

        let description_paragraph = Paragraph::new(description_display)
            .block(results_block)
            .style(if self.description.len() == 1 && self.description[0].is_empty() && !matches!(self.mode, InputMode::Description) {
                styles.muted
            } else {
                styles.normal
            })
            .wrap(Wrap { trim: false });

        frame.render_widget(description_paragraph, chunks[1]);

        // Status line
        let mode_indicator = if self.is_editing { "[EDIT]" } else { "[INSERT]" };
        let help_text = match self.mode {
            InputMode::Title => format!("{} <C-Enter>/<C-s> Save | <Enter>/<Tab> Next | <Esc> Close", mode_indicator),
            InputMode::Description => format!("{} <C-Enter>/<C-s> Save | <Enter> Newline | <Tab> Back to Title | <Esc> Close", mode_indicator),
        };

        let status_line = Paragraph::new(help_text)
            .style(styles.muted)
            .alignment(Alignment::Left);

        frame.render_widget(status_line, chunks[2]);
    }
}
use crate::ui::themes::{ThemeStyles, ThemeColors};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

#[derive(Debug, Clone, PartialEq)]
pub enum VimCommandResult {
    Continue,
    Save,
    SaveAndClose,
    Close,
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Title,
    Description,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VimMode {
    Normal,
    Insert,
    Visual,
    VisualLine,
    VisualBlock,
    Command,
}

pub struct VimInput {
    pub active: bool,
    pub title: String,
    pub description: Vec<String>,
    pub cursor_position: usize,
    pub mode: InputMode,
    pub vim_mode: VimMode,
    pub current_line: usize,
    pub cursor_col: usize,
    pub is_editing: bool,
    // Visual mode selection
    pub visual_start_line: usize,
    pub visual_start_col: usize,
    pub visual_end_line: usize,
    pub visual_end_col: usize,
    // Command mode
    pub command_buffer: String,
    // Clipboard for yank/paste operations
    pub clipboard: Vec<String>,
}

impl VimInput {
    pub fn new() -> Self {
        Self {
            active: false,
            title: String::new(),
            description: vec![String::new()],
            cursor_position: 0,
            mode: InputMode::Title,
            vim_mode: VimMode::Insert,
            current_line: 0,
            cursor_col: 0,
            is_editing: false,
            visual_start_line: 0,
            visual_start_col: 0,
            visual_end_line: 0,
            visual_end_col: 0,
            command_buffer: String::new(),
            clipboard: Vec::new(),
        }
    }

    pub fn open(&mut self) {
        self.active = true;
        self.title.clear();
        self.description = vec![String::new()];
        self.cursor_position = 0;
        self.mode = InputMode::Title;
        self.vim_mode = VimMode::Insert;
        self.current_line = 0;
        self.cursor_col = 0;
        self.is_editing = false;
        self.reset_visual_selection();
        self.command_buffer.clear();
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
        self.vim_mode = VimMode::Insert;
        self.current_line = 0;
        self.cursor_col = 0;
        self.is_editing = true;
        self.reset_visual_selection();
        self.command_buffer.clear();
    }

    pub fn close(&mut self) {
        self.active = false;
    }

    pub fn enter_insert_mode(&mut self) {
        self.vim_mode = VimMode::Insert;
    }

    pub fn enter_normal_mode(&mut self) {
        self.vim_mode = VimMode::Normal;
    }

    pub fn is_insert_mode(&self) -> bool {
        match self.mode {
            InputMode::Title => true,
            InputMode::Description => matches!(self.vim_mode, VimMode::Insert),
        }
    }

    pub fn is_visual_mode(&self) -> bool {
        matches!(self.mode, InputMode::Description) && 
        matches!(self.vim_mode, VimMode::Visual | VimMode::VisualLine | VimMode::VisualBlock)
    }

    pub fn enter_visual_mode(&mut self) {
        self.vim_mode = VimMode::Visual;
        self.start_visual_selection();
    }

    pub fn enter_visual_line_mode(&mut self) {
        self.vim_mode = VimMode::VisualLine;
        self.start_visual_selection();
    }

    pub fn enter_visual_block_mode(&mut self) {
        self.vim_mode = VimMode::VisualBlock;
        self.start_visual_selection();
    }

    pub fn enter_command_mode(&mut self) {
        self.vim_mode = VimMode::Command;
        self.command_buffer.clear();
    }

    pub fn is_command_mode(&self) -> bool {
        matches!(self.mode, InputMode::Description) && matches!(self.vim_mode, VimMode::Command)
    }

    fn start_visual_selection(&mut self) {
        self.visual_start_line = self.current_line;
        self.visual_start_col = self.cursor_col;
        self.visual_end_line = self.current_line;
        self.visual_end_col = self.cursor_col;
    }

    fn reset_visual_selection(&mut self) {
        self.visual_start_line = 0;
        self.visual_start_col = 0;
        self.visual_end_line = 0;
        self.visual_end_col = 0;
    }

    fn update_visual_selection(&mut self) {
        if self.is_visual_mode() {
            self.visual_end_line = self.current_line;
            self.visual_end_col = self.cursor_col;
        }
    }

    pub fn get_vim_mode_display(&self) -> &str {
        match self.vim_mode {
            VimMode::Normal => "NORMAL",
            VimMode::Insert => "INSERT",
            VimMode::Visual => "VISUAL",
            VimMode::VisualLine => "V-LINE",
            VimMode::VisualBlock => "V-BLOCK",
            VimMode::Command => "COMMAND",
        }
    }

    pub fn execute_vim_command(&mut self) -> VimCommandResult {
        let command = self.command_buffer.trim();
        
        match command {
            "w" | "write" => {
                self.vim_mode = VimMode::Normal;
                VimCommandResult::Save
            }
            "x" | "wq" => {
                VimCommandResult::SaveAndClose
            }
            "q" | "quit" => {
                VimCommandResult::Close
            }
            "q!" | "quit!" => {
                VimCommandResult::Close
            }
            "" => {
                self.vim_mode = VimMode::Normal;
                VimCommandResult::Continue
            }
            _ => {
                self.vim_mode = VimMode::Normal;
                VimCommandResult::Error(format!("Unknown command: {}", command))
            }
        }
    }

    pub fn handle_char(&mut self, c: char) {
        if self.is_command_mode() {
            self.command_buffer.push(c);
            return;
        }

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
        if self.is_command_mode() {
            if !self.command_buffer.is_empty() {
                self.command_buffer.pop();
            }
            return;
        }

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
                self.vim_mode = VimMode::Insert;
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
                self.update_visual_selection();
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
                self.update_visual_selection();
            }
        }
    }

    pub fn move_cursor_up(&mut self) {
        if matches!(self.mode, InputMode::Description) && self.current_line > 0 {
            self.current_line -= 1;
            self.cursor_col = self.cursor_col.min(self.description[self.current_line].len());
            self.update_visual_selection();
        }
    }

    pub fn move_cursor_down(&mut self) {
        if matches!(self.mode, InputMode::Description) && self.current_line < self.description.len() - 1 {
            self.current_line += 1;
            self.cursor_col = self.cursor_col.min(self.description[self.current_line].len());
            self.update_visual_selection();
        }
    }

    // Vim motions and operations
    pub fn vim_go_to_line_start(&mut self) {
        match self.mode {
            InputMode::Title => self.cursor_position = 0,
            InputMode::Description => self.cursor_col = 0,
        }
        self.update_visual_selection();
    }

    pub fn vim_go_to_line_end(&mut self) {
        match self.mode {
            InputMode::Title => self.cursor_position = self.title.len(),
            InputMode::Description => self.cursor_col = self.description[self.current_line].len(),
        }
        self.update_visual_selection();
    }

    pub fn vim_go_to_first_line(&mut self) {
        if matches!(self.mode, InputMode::Description) {
            self.current_line = 0;
            self.cursor_col = 0;
            self.update_visual_selection();
        }
    }

    pub fn vim_go_to_last_line(&mut self) {
        if matches!(self.mode, InputMode::Description) {
            self.current_line = self.description.len().saturating_sub(1);
            self.cursor_col = self.description[self.current_line].len();
            self.update_visual_selection();
        }
    }

    pub fn vim_delete_line(&mut self) {
        if matches!(self.mode, InputMode::Description) && !self.description.is_empty() {
            let deleted_line = self.description.remove(self.current_line);
            self.clipboard = vec![deleted_line];
            
            if self.description.is_empty() {
                self.description.push(String::new());
                self.current_line = 0;
            } else if self.current_line >= self.description.len() {
                self.current_line = self.description.len() - 1;
            }
            self.cursor_col = 0;
        }
    }

    pub fn vim_yank_line(&mut self) {
        if matches!(self.mode, InputMode::Description) {
            let line = self.description[self.current_line].clone();
            self.clipboard = vec![line];
        }
    }

    pub fn vim_paste_after(&mut self) {
        if !self.clipboard.is_empty() {
            match self.mode {
                InputMode::Description => {
                    for (i, line) in self.clipboard.iter().enumerate() {
                        self.description.insert(self.current_line + 1 + i, line.clone());
                    }
                    self.current_line += 1;
                    self.cursor_col = 0;
                }
                InputMode::Title => {
                    if let Some(text) = self.clipboard.get(0) {
                        self.title.insert_str(self.cursor_position, text);
                        self.cursor_position += text.len();
                    }
                }
            }
        }
    }

    pub fn vim_word_forward(&mut self) {
        match self.mode {
            InputMode::Title => {
                let chars: Vec<char> = self.title.chars().collect();
                let mut pos = self.cursor_position;
                while pos < chars.len() && chars[pos].is_alphanumeric() {
                    pos += 1;
                }
                while pos < chars.len() && !chars[pos].is_alphanumeric() {
                    pos += 1;
                }
                self.cursor_position = pos;
            }
            InputMode::Description => {
                let line = &self.description[self.current_line];
                let chars: Vec<char> = line.chars().collect();
                let mut col = self.cursor_col;
                
                if col >= chars.len() {
                    if self.current_line < self.description.len() - 1 {
                        self.current_line += 1;
                        self.cursor_col = 0;
                    }
                    return;
                }
                
                while col < chars.len() && chars[col].is_alphanumeric() {
                    col += 1;
                }
                while col < chars.len() && !chars[col].is_alphanumeric() {
                    col += 1;
                }
                
                if col >= chars.len() && self.current_line < self.description.len() - 1 {
                    self.current_line += 1;
                    self.cursor_col = 0;
                } else {
                    self.cursor_col = col;
                }
            }
        }
        self.update_visual_selection();
    }

    pub fn vim_word_backward(&mut self) {
        match self.mode {
            InputMode::Title => {
                if self.cursor_position > 0 {
                    let chars: Vec<char> = self.title.chars().collect();
                    let mut pos = self.cursor_position - 1;
                    while pos > 0 && !chars[pos].is_alphanumeric() {
                        pos -= 1;
                    }
                    while pos > 0 && chars[pos - 1].is_alphanumeric() {
                        pos -= 1;
                    }
                    self.cursor_position = pos;
                }
            }
            InputMode::Description => {
                if self.cursor_col > 0 {
                    let line = &self.description[self.current_line];
                    let chars: Vec<char> = line.chars().collect();
                    let mut col = self.cursor_col - 1;
                    while col > 0 && !chars[col].is_alphanumeric() {
                        col -= 1;
                    }
                    while col > 0 && chars[col - 1].is_alphanumeric() {
                        col -= 1;
                    }
                    self.cursor_col = col;
                } else if self.current_line > 0 {
                    self.current_line -= 1;
                    self.cursor_col = self.description[self.current_line].len();
                }
            }
        }
        self.update_visual_selection();
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
        let constraints = if self.is_command_mode() {
            vec![
                Constraint::Length(3),   // Prompt section (title input)
                Constraint::Min(4),      // Results section (description)
                Constraint::Length(1),   // Status line
                Constraint::Length(1),   // Command line
            ]
        } else {
            vec![
                Constraint::Length(3),   // Prompt section (title input)
                Constraint::Min(5),      // Results section (description)
                Constraint::Length(1),   // Status line
            ]
        };
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(popup_area);

        // Telescope-style prompt section
        let prompt_text = if matches!(self.mode, InputMode::Title) {
            let mut display_title = self.title.clone();
            if matches!(self.vim_mode, VimMode::Insert) {
                display_title.insert(self.cursor_position, '_');
                format!("> {}", display_title)
            } else {
                if display_title.is_empty() {
                    format!("> █")
                } else {
                    display_title.insert(self.cursor_position.min(display_title.len()), '█');
                    format!("> {}", display_title)
                }
            }
        } else {
            format!("> {}", self.title)
        };

        let prompt_border_chars = if matches!(self.mode, InputMode::Title) {
            ["─", "│", "─", "│", "┌", "┐", "┘", "└"]
        } else {
            ["─", "│", "─", "│", "├", "┤", "┤", "├"]
        };

        let prompt_block = Block::default()
            .title(" Prompt ")
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
                let cursor_char = match self.vim_mode {
                    VimMode::Insert => '│',
                    VimMode::Normal => '█',
                    VimMode::Visual | VimMode::VisualLine | VimMode::VisualBlock => '█',
                    VimMode::Command => '│', // Similar to insert mode
                };
                display_line.insert(self.cursor_col.min(display_line.len()), cursor_char);
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
            .title(" Results ")
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

        // Telescope-style status line
        let mode_indicator = match (&self.vim_mode, self.is_editing) {
            (VimMode::Insert, true) => "[EDIT]",
            (VimMode::Insert, false) => "[INSERT]",
            (VimMode::Normal, _) => "[NORMAL]",
            (VimMode::Visual, _) => "[VISUAL]",
            (VimMode::VisualLine, _) => "[V-LINE]",
            (VimMode::VisualBlock, _) => "[V-BLOCK]",
            (VimMode::Command, _) => "[COMMAND]",
        };
        let help_text = match &self.mode {
            InputMode::Title => format!("{} <C-Enter>/<C-s> Save | <Enter> Next | <Esc> Close", mode_indicator),
            InputMode::Description => match &self.vim_mode {
                VimMode::Insert => format!("{} <C-Enter>/<C-s> Save | <Enter> Newline | <Esc> Normal mode", mode_indicator),
                VimMode::Normal => format!("{} <C-Enter>/<C-s> Save | i Insert | a Append | v Visual | V V-Line | : Command | hjkl/0$/gg/G/w/b Nav | dd Delete | yy Yank | p Paste | <Esc> Close", mode_indicator),
                VimMode::Visual | VimMode::VisualLine | VimMode::VisualBlock => format!("{} hjkl Nav | d Delete | y Yank | <Esc> Normal mode", mode_indicator),
                VimMode::Command => format!("{} Type command and press <Enter> to execute | <Esc> Cancel", mode_indicator),
            }
        };

        let status_line = Paragraph::new(help_text)
            .style(styles.muted)
            .alignment(Alignment::Left);

        frame.render_widget(status_line, chunks[2]);

        // Render command line if in command mode
        if self.is_command_mode() {
            let command_text = format!(":{}", self.command_buffer);
            let command_line = Paragraph::new(command_text)
                .style(styles.normal.bg(colors.vim_command_bg).fg(colors.vim_text))
                .alignment(Alignment::Left);
            frame.render_widget(command_line, chunks[3]);
        }
    }
}
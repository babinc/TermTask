use crate::ui::styling::{ThemeStyles, ThemeColors};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph, Widget},
    Frame,
};
use edtui::{EditorState, EditorView, EditorEventHandler, Lines, EditorMode, EditorTheme};
use crossterm::event::KeyEvent;


#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Title,
    Description,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VimCommand {
    Save,
    SaveAndClose,
    Quit,
}


pub struct VimInput {
    pub active: bool,
    pub title: String,
    pub cursor_position: usize,
    pub mode: InputMode,
    pub is_editing: bool,
    pub command_buffer: String,
    pub description_editor: EditorState,
    pub description_event_handler: EditorEventHandler,
}

impl VimInput {
    pub fn new() -> Self {
        Self {
            active: false,
            title: String::new(),
            cursor_position: 0,
            mode: InputMode::Title,
            is_editing: false,
            command_buffer: String::new(),
            description_editor: EditorState::default(),
            description_event_handler: EditorEventHandler::default(),
        }
    }

    pub fn open(&mut self) {
        self.active = true;
        self.title.clear();
        self.cursor_position = 0;
        self.mode = InputMode::Title;
        self.is_editing = false;
        self.command_buffer.clear();
        self.description_editor = EditorState::default();
        self.description_event_handler = EditorEventHandler::default();
    }

    pub fn open_with_data(&mut self, title: &str, description: Option<&str>) {
        self.active = true;
        self.title = title.to_string();
        self.cursor_position = title.len();
        self.mode = InputMode::Title;
        self.is_editing = true;
        self.command_buffer.clear();

        if let Some(desc) = description {
            let lines = Lines::from(desc);
            self.description_editor = EditorState::new(lines);
        } else {
            self.description_editor = EditorState::default();
        }
        self.description_event_handler = EditorEventHandler::default();
    }

    pub fn close(&mut self) {
        self.active = false;
    }


    pub fn is_insert_mode(&self) -> bool {
        match self.mode {
            InputMode::Title => true,
            InputMode::Description => true,
        }
    }


    pub fn get_vim_mode_display(&self) -> &str {
        if !self.command_buffer.is_empty() {
            "COMMAND"
        } else {
            match self.mode {
                InputMode::Title => "INSERT",
                InputMode::Description => "EDIT",
            }
        }
    }

    pub fn get_command_buffer(&self) -> &str {
        &self.command_buffer
    }


    pub fn handle_key_event(&mut self, key: KeyEvent) -> Option<VimCommand> {
        match self.mode {
            InputMode::Description => {
                if matches!(self.description_editor.mode, EditorMode::Normal) {
                    match key.code {
                        crossterm::event::KeyCode::Char(':') => {
                            self.command_buffer.clear();
                            self.command_buffer.push(':');
                            return None;
                        }
                        crossterm::event::KeyCode::Char(c) if !self.command_buffer.is_empty() => {
                            if c == '\n' || c == '\r' {
                                let cmd = self.command_buffer.clone();
                                self.command_buffer.clear();
                                return self.parse_vim_command(&cmd);
                            } else {
                                self.command_buffer.push(c);
                                return None;
                            }
                        }
                        crossterm::event::KeyCode::Enter if !self.command_buffer.is_empty() => {
                            let cmd = self.command_buffer.clone();
                            self.command_buffer.clear();
                            return self.parse_vim_command(&cmd);
                        }
                        crossterm::event::KeyCode::Esc if !self.command_buffer.is_empty() => {
                            self.command_buffer.clear();
                            return None;
                        }
                        _ => {
                            if !self.command_buffer.is_empty() {
                                return None;
                            }
                        }
                    }
                }

                if self.command_buffer.is_empty() {
                    self.description_event_handler.on_key_event(key, &mut self.description_editor);
                }
                None
            }
            InputMode::Title => {
                match key.code {
                    crossterm::event::KeyCode::Char(c) => {
                        self.title.insert(self.cursor_position, c);
                        self.cursor_position += 1;
                    }
                    crossterm::event::KeyCode::Backspace => {
                        if self.cursor_position > 0 {
                            self.cursor_position -= 1;
                            self.title.remove(self.cursor_position);
                        }
                    }
                    crossterm::event::KeyCode::Left => {
                        if self.cursor_position > 0 {
                            self.cursor_position -= 1;
                        }
                    }
                    crossterm::event::KeyCode::Right => {
                        if self.cursor_position < self.title.len() {
                            self.cursor_position += 1;
                        }
                    }
                    crossterm::event::KeyCode::Enter => {
                        self.mode = InputMode::Description;
                    }
                    _ => {}
                }
                None
            }
        }
    }


    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_description(&self) -> Option<String> {
        let text = self.extract_text_from_edtui();
        if text.trim().is_empty() {
            None
        } else {
            Some(text)
        }
    }

    fn extract_text_from_edtui(&self) -> String {
        let lines = &self.description_editor.lines;
        let debug_str = format!("{:?}", lines);

        if let Some(start_pos) = debug_str.find("data: [") {
            let start = start_pos + 7;

            let end = if let Some(end_pos) = debug_str.rfind("]]") {
                end_pos + 2
            } else if let Some(end_pos) = debug_str.rfind("] }") {
                end_pos + 1
            } else {
                debug_str.rfind(']').unwrap_or(debug_str.len())
            };

            if end <= start {
                return String::new();
            }

            let data_content = &debug_str[start..end];

            let mut text_lines = Vec::new();
            let lines_str = data_content.trim_start_matches('[').trim_end_matches(']');

            if lines_str.is_empty() {
                return String::new();
            }

            let line_parts: Vec<&str> = if lines_str.contains("], [") {
                lines_str.split("], [").collect()
            } else {
                vec![lines_str]
            };

            for line_part in line_parts {
                let line_chars = line_part.trim_start_matches('[').trim_end_matches(']');
                let mut line_text = String::new();

                for char_match in line_chars.split(", ") {
                    let char_str = char_match.trim().trim_matches('\'');
                    if char_str.len() == 1 {
                        line_text.push(char_str.chars().next().unwrap());
                    } else if char_str == " " {
                        line_text.push(' ');
                    }
                }

                text_lines.push(line_text);
            }

            text_lines.join("\n")
        } else {
            String::new()
        }
    }

    pub fn is_edtui_in_normal_mode(&self) -> bool {
        matches!(self.description_editor.mode, EditorMode::Normal)
    }

    pub fn switch_to_description(&mut self) {
        self.mode = InputMode::Description;
    }

    pub fn switch_to_title(&mut self) {
        self.mode = InputMode::Title;
    }

    pub fn is_in_title_mode(&self) -> bool {
        matches!(self.mode, InputMode::Title)
    }

    fn parse_vim_command(&self, cmd: &str) -> Option<VimCommand> {
        match cmd.trim() {
            ":w" | ":write" => Some(VimCommand::Save),
            ":x" | ":wq" => Some(VimCommand::SaveAndClose),
            ":q" | ":quit" => Some(VimCommand::Quit),
            _ => None,
        }
    }


    fn create_editor_theme(colors: &ThemeColors) -> EditorTheme {
        EditorTheme {
            base: Style::default().bg(colors.modal_bg).fg(colors.foreground),
            cursor_style: Style::default().bg(colors.primary).fg(colors.vim_text),
            selection_style: Style::default().bg(colors.accent).fg(colors.vim_text),
            block: None,
            status_line: None,
        }
    }


    pub fn render(&mut self, frame: &mut Frame, area: Rect, styles: &ThemeStyles, colors: &ThemeColors) {
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

        let constraints = vec![
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(1),
        ];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(popup_area);

        let prompt_text = if matches!(self.mode, InputMode::Title) {
            let mut display_title = self.title.clone();
            display_title.insert(self.cursor_position, '█');
            format!("> {}", display_title)
        } else {
            format!("> {}", self.title)
        };

        let title_block = Block::default()
            .title(" Title ")
            .borders(Borders::ALL)
            .border_style(if matches!(self.mode, InputMode::Title) {
                Style::default().fg(colors.modal_border)
            } else {
                Style::default().fg(colors.border)
            })
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg));

        let title_input = Paragraph::new(prompt_text)
            .block(title_block)
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg));

        frame.render_widget(title_input, chunks[0]);

        let desc_block = Block::default()
            .title(" Description ")
            .borders(Borders::ALL)
            .border_style(if matches!(self.mode, InputMode::Description) {
                Style::default().fg(colors.modal_border)
            } else {
                Style::default().fg(colors.border)
            })
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg));

        let inner_area = desc_block.inner(chunks[1]);
        frame.render_widget(desc_block, chunks[1]);
        let editor_theme = Self::create_editor_theme(colors);

        EditorView::new(&mut self.description_editor)
            .theme(editor_theme)
            .wrap(true)
            .render(inner_area, frame.buffer_mut());

        let mode_indicator = self.get_vim_mode_display();
        let help_text = if !self.command_buffer.is_empty() {
            format!("[{}] {}", mode_indicator, self.command_buffer)
        } else {
            match &self.mode {
                InputMode::Title => format!("[{}] <Enter> Next | <C-Enter>/<C-s> Save | <Esc> Close", mode_indicator),
                InputMode::Description => format!("[{}] Full Vim Editor (:w save, :x save&close, :q quit) | <C-Enter>/<C-s> Save | <Esc> Close", mode_indicator),
            }
        };

        let status_line = Paragraph::new(help_text)
            .style(Style::default().fg(colors.muted).bg(colors.modal_bg))
            .alignment(Alignment::Left);

        frame.render_widget(status_line, chunks[2]);
    }
}
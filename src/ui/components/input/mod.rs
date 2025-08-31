pub mod normal;
pub mod vim;

pub use normal::{NormalInput, InputMode as NormalInputMode};
pub use vim::{VimInput, InputMode as VimInputMode, VimMode, VimCommandResult};

use crate::ui::themes::{ThemeStyles, ThemeColors};
use ratatui::{Frame, layout::Rect};

// Unified input interface that delegates to either normal or vim input
pub enum InputHandler {
    Normal(NormalInput),
    Vim(VimInput),
}

impl InputHandler {
    pub fn new(vim_mode: bool) -> Self {
        if vim_mode {
            Self::Vim(VimInput::new())
        } else {
            Self::Normal(NormalInput::new())
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            Self::Normal(input) => input.active,
            Self::Vim(input) => input.active,
        }
    }

    pub fn open(&mut self) {
        match self {
            Self::Normal(input) => input.open(),
            Self::Vim(input) => input.open(),
        }
    }

    pub fn open_with_data(&mut self, title: &str, description: Option<&str>) {
        match self {
            Self::Normal(input) => input.open_with_data(title, description),
            Self::Vim(input) => input.open_with_data(title, description),
        }
    }

    pub fn close(&mut self) {
        match self {
            Self::Normal(input) => input.close(),
            Self::Vim(input) => input.close(),
        }
    }

    pub fn get_title(&self) -> &str {
        match self {
            Self::Normal(input) => input.get_title(),
            Self::Vim(input) => input.get_title(),
        }
    }

    pub fn get_description(&self) -> Option<String> {
        match self {
            Self::Normal(input) => input.get_description(),
            Self::Vim(input) => input.get_description(),
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, styles: &ThemeStyles, colors: &ThemeColors) {
        match self {
            Self::Normal(input) => input.render(frame, area, styles, colors),
            Self::Vim(input) => input.render(frame, area, styles, colors),
        }
    }

    // Vim-specific methods
    pub fn is_insert_mode(&self) -> bool {
        match self {
            Self::Normal(_) => true, // Normal input is always in "insert" mode
            Self::Vim(input) => input.is_insert_mode(),
        }
    }

    pub fn enter_insert_mode(&mut self) {
        if let Self::Vim(input) = self {
            input.enter_insert_mode();
        }
    }

    pub fn enter_normal_mode(&mut self) {
        if let Self::Vim(input) = self {
            input.enter_normal_mode();
        }
    }

    // Input handling methods
    pub fn handle_char(&mut self, c: char) {
        match self {
            Self::Normal(input) => input.handle_char(c),
            Self::Vim(input) => input.handle_char(c),
        }
    }

    pub fn handle_backspace(&mut self) {
        match self {
            Self::Normal(input) => input.handle_backspace(),
            Self::Vim(input) => input.handle_backspace(),
        }
    }

    pub fn handle_enter(&mut self) -> bool {
        match self {
            Self::Normal(input) => input.handle_enter(),
            Self::Vim(input) => input.handle_enter(),
        }
    }

    pub fn move_cursor_left(&mut self) {
        match self {
            Self::Normal(input) => input.move_cursor_left(),
            Self::Vim(input) => input.move_cursor_left(),
        }
    }

    pub fn move_cursor_right(&mut self) {
        match self {
            Self::Normal(input) => input.move_cursor_right(),
            Self::Vim(input) => input.move_cursor_right(),
        }
    }

    pub fn move_cursor_up(&mut self) {
        match self {
            Self::Normal(input) => input.move_cursor_up(),
            Self::Vim(input) => input.move_cursor_up(),
        }
    }

    pub fn move_cursor_down(&mut self) {
        match self {
            Self::Normal(input) => input.move_cursor_down(),
            Self::Vim(input) => input.move_cursor_down(),
        }
    }

    // Get current input mode for help text
    pub fn get_input_mode(&self) -> String {
        match self {
            Self::Normal(input) => match input.mode {
                NormalInputMode::Title => "Title".to_string(),
                NormalInputMode::Description => "Description".to_string(),
            },
            Self::Vim(input) => match input.mode {
                VimInputMode::Title => "Title".to_string(),
                VimInputMode::Description => "Description".to_string(),
            },
        }
    }

    // Get vim mode display string
    pub fn get_vim_mode_display(&self) -> Option<&str> {
        match self {
            Self::Normal(_) => None,
            Self::Vim(input) => Some(input.get_vim_mode_display()),
        }
    }

    // Vim-specific command mode methods
    pub fn enter_command_mode(&mut self) {
        if let Self::Vim(input) = self {
            input.enter_command_mode();
        }
    }

    pub fn is_command_mode(&self) -> bool {
        match self {
            Self::Normal(_) => false,
            Self::Vim(input) => input.is_command_mode(),
        }
    }

    pub fn execute_vim_command(&mut self) -> Option<VimCommandResult> {
        match self {
            Self::Normal(_) => None,
            Self::Vim(input) => Some(input.execute_vim_command()),
        }
    }

    // Vim motion methods
    pub fn vim_handle_normal_mode_key(&mut self, key: char) {
        if let Self::Vim(input) = self {
            match key {
                ':' => input.enter_command_mode(),
                '0' => input.vim_go_to_line_start(),
                '$' => input.vim_go_to_line_end(),
                'g' => {}, // Handle 'gg' in app logic (need two key sequence)
                'G' => input.vim_go_to_last_line(),
                'w' => input.vim_word_forward(),
                'b' => input.vim_word_backward(),
                'd' => {}, // Handle 'dd' in app logic (need two key sequence)
                'y' => {}, // Handle 'yy' in app logic (need two key sequence)
                'p' => input.vim_paste_after(),
                'v' => input.enter_visual_mode(),
                'V' => input.enter_visual_line_mode(),
                _ => {}
            }
        }
    }

    pub fn vim_delete_line(&mut self) {
        if let Self::Vim(input) = self {
            input.vim_delete_line();
        }
    }

    pub fn vim_yank_line(&mut self) {
        if let Self::Vim(input) = self {
            input.vim_yank_line();
        }
    }

    pub fn vim_go_to_first_line(&mut self) {
        if let Self::Vim(input) = self {
            input.vim_go_to_first_line();
        }
    }
}
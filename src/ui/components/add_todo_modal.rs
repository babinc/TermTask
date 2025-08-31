use crate::ui::components::input::{InputHandler, VimCommandResult};
use crate::ui::themes::{ThemeStyles, ThemeColors};
use ratatui::{Frame, layout::Rect};

pub struct AddTodoModal {
    pub input_handler: InputHandler,
    vim_mode_enabled: bool,
}

// Re-export for backwards compatibility
pub use crate::ui::components::input::{VimInputMode as TodoModalMode};

impl AddTodoModal {
    pub fn new() -> Self {
        // Default to non-vim mode - will be updated when needed
        Self {
            input_handler: InputHandler::new(false),
            vim_mode_enabled: false,
        }
    }

    pub fn new_with_vim_mode(vim_mode: bool) -> Self {
        Self {
            input_handler: InputHandler::new(vim_mode),
            vim_mode_enabled: vim_mode,
        }
    }

    // Delegate all methods to the InputHandler
    pub fn open(&mut self) {
        self.input_handler.open();
    }

    pub fn open_with_data(&mut self, title: &str, description: Option<&str>) {
        self.input_handler.open_with_data(title, description);
    }

    pub fn close(&mut self) {
        self.input_handler.close();
    }

    pub fn active(&self) -> bool {
        self.input_handler.is_active()
    }

    pub fn enter_insert_mode(&mut self) {
        self.input_handler.enter_insert_mode();
    }

    pub fn enter_normal_mode(&mut self) {
        self.input_handler.enter_normal_mode();
    }

    pub fn is_insert_mode(&self) -> bool {
        self.input_handler.is_insert_mode()
    }

    pub fn handle_char(&mut self, c: char) {
        self.input_handler.handle_char(c);
    }

    pub fn handle_backspace(&mut self) {
        self.input_handler.handle_backspace();
    }

    pub fn handle_enter(&mut self) -> bool {
        self.input_handler.handle_enter()
    }

    pub fn move_cursor_left(&mut self) {
        self.input_handler.move_cursor_left();
    }

    pub fn move_cursor_right(&mut self) {
        self.input_handler.move_cursor_right();
    }

    pub fn move_cursor_up(&mut self) {
        self.input_handler.move_cursor_up();
    }

    pub fn move_cursor_down(&mut self) {
        self.input_handler.move_cursor_down();
    }

    pub fn get_title(&self) -> &str {
        self.input_handler.get_title()
    }

    pub fn get_description(&self) -> Option<String> {
        self.input_handler.get_description()
    }

    // Vim-specific methods
    pub fn execute_vim_command(&mut self) -> Option<VimCommandResult> {
        self.input_handler.execute_vim_command()
    }

    pub fn vim_handle_normal_mode_key(&mut self, key: char) {
        self.input_handler.vim_handle_normal_mode_key(key);
    }

    pub fn enter_command_mode(&mut self) {
        self.input_handler.enter_command_mode();
    }

    pub fn is_command_mode(&self) -> bool {
        self.input_handler.is_command_mode()
    }

    // For backwards compatibility - update this getter
    pub fn mode(&self) -> String {
        self.input_handler.get_input_mode()
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, styles: &ThemeStyles, colors: &ThemeColors) {
        self.input_handler.render(frame, area, styles, colors);
    }
}
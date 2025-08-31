use crate::ui::components::input::InputHandler;
use crate::ui::themes::{ThemeStyles, ThemeColors};
use ratatui::{Frame, layout::Rect};

pub struct UnifiedModal {
    pub input_handler: InputHandler,
    pub is_editing: bool,
}

impl UnifiedModal {
    pub fn new(vim_mode: bool) -> Self {
        Self {
            input_handler: InputHandler::new(vim_mode),
            is_editing: false,
        }
    }

    pub fn is_active(&self) -> bool {
        self.input_handler.is_active()
    }

    pub fn open(&mut self) {
        self.input_handler.open();
        self.is_editing = false;
    }

    pub fn open_with_data(&mut self, title: &str, description: Option<&str>) {
        self.input_handler.open_with_data(title, description);
        self.is_editing = true;
    }

    pub fn close(&mut self) {
        self.input_handler.close();
    }

    pub fn get_title(&self) -> &str {
        self.input_handler.get_title()
    }

    pub fn get_description(&self) -> Option<String> {
        self.input_handler.get_description()
    }

    pub fn is_insert_mode(&self) -> bool {
        self.input_handler.is_insert_mode()
    }

    pub fn enter_insert_mode(&mut self) {
        self.input_handler.enter_insert_mode();
    }

    pub fn enter_normal_mode(&mut self) {
        self.input_handler.enter_normal_mode();
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

    pub fn render(&self, frame: &mut Frame, area: Rect, styles: &ThemeStyles, colors: &ThemeColors) {
        self.input_handler.render(frame, area, styles, colors);
    }

    pub fn get_input_mode(&self) -> String {
        self.input_handler.get_input_mode()
    }
}
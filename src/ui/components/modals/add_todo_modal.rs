use crate::ui::components::input::{InputHandler, VimCommand};
use crate::ui::styling::{ThemeStyles, ThemeColors};
use ratatui::{Frame, layout::Rect};

pub struct AddTodoModal {
    pub input_handler: InputHandler,
    original_title: String,
    original_description: Option<String>,
}


impl AddTodoModal {
    pub fn new_with_vim_mode(vim_mode: bool) -> Self {
        Self {
            input_handler: InputHandler::new(vim_mode),
            original_title: String::new(),
            original_description: None,
        }
    }

    pub fn open(&mut self) {
        self.input_handler.open();
        self.original_title = String::new();
        self.original_description = None;
    }

    pub fn open_with_data(&mut self, title: &str, description: Option<&str>) {
        self.input_handler.open_with_data(title, description);
        self.original_title = title.to_string();
        self.original_description = description.map(|d| d.to_string());
    }

    pub fn close(&mut self) {
        self.input_handler.close();
    }

    pub fn active(&self) -> bool {
        self.input_handler.is_active()
    }


    pub fn is_insert_mode(&self) -> bool {
        self.input_handler.is_insert_mode()
    }


    pub fn get_title(&self) -> &str {
        self.input_handler.get_title()
    }

    pub fn get_description(&self) -> Option<String> {
        self.input_handler.get_description()
    }





    pub fn render(&mut self, frame: &mut Frame, area: Rect, styles: &ThemeStyles, colors: &ThemeColors) {
        self.input_handler.render(frame, area, styles, colors);
    }

    pub fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Option<VimCommand> {
        self.input_handler.handle_key_event(key)
    }

    pub fn should_escape_close_modal(&self) -> bool {
        self.input_handler.is_edtui_in_normal_mode()
    }


    pub fn has_unsaved_changes(&self) -> bool {
        let current_title = self.get_title();
        let current_description = self.get_description();

        if current_title != self.original_title {
            return true;
        }

        match (&current_description, &self.original_description) {
            (None, None) => false,
            (Some(current), None) => !current.trim().is_empty(),
            (None, Some(original)) => !original.trim().is_empty(),
            (Some(current), Some(original)) => current != original,
        }
    }

    pub fn switch_to_description(&mut self) {
        self.input_handler.switch_to_description();
    }

    pub fn switch_to_title(&mut self) {
        self.input_handler.switch_to_title();
    }

    pub fn is_in_title_mode(&self) -> bool {
        self.input_handler.is_in_title_mode()
    }
}
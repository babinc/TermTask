pub mod normal;
pub mod vim;

pub use normal::NormalInput;
pub use vim::{VimInput, InputMode as VimInputMode, VimCommand};

use crate::ui::styling::{ThemeStyles, ThemeColors};
use ratatui::{Frame, layout::Rect};
use crossterm::event::KeyEvent;

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

    pub fn render(&mut self, frame: &mut Frame, area: Rect, styles: &ThemeStyles, colors: &ThemeColors) {
        match self {
            Self::Normal(input) => input.render(frame, area, styles, colors),
            Self::Vim(input) => input.render(frame, area, styles, colors),
        }
    }

    pub fn is_insert_mode(&self) -> bool {
        match self {
            Self::Normal(_) => true,
            Self::Vim(input) => input.is_insert_mode(),
        }
    }




    pub fn get_vim_mode_display(&self) -> Option<&str> {
        match self {
            Self::Normal(_) => None,
            Self::Vim(input) => Some(input.get_vim_mode_display()),
        }
    }




    pub fn handle_key_event(&mut self, key: KeyEvent) -> Option<VimCommand> {
        match self {
            Self::Normal(_) => None,
            Self::Vim(input) => {
                input.handle_key_event(key)
            }
        }
    }

    pub fn is_edtui_in_normal_mode(&self) -> bool {
        match self {
            Self::Normal(_) => true,
            Self::Vim(input) => {
                match input.mode {
                    VimInputMode::Title => true,
                    VimInputMode::Description => input.is_edtui_in_normal_mode(),
                }
            }
        }
    }

    pub fn switch_to_description(&mut self) {
        if let Self::Vim(input) = self {
            input.switch_to_description();
        }
    }

    pub fn switch_to_title(&mut self) {
        if let Self::Vim(input) = self {
            input.switch_to_title();
        }
    }

    pub fn is_in_title_mode(&self) -> bool {
        match self {
            Self::Normal(_) => true,
            Self::Vim(input) => input.is_in_title_mode(),
        }
    }


}
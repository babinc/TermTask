use crate::ui::{
    styling::{VimIndicator, ThemeColors, ThemeStyles},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::Paragraph,
    Frame,
};

use super::App;

impl App {
    pub(super) fn draw(&mut self, frame: &mut Frame) {
        let colors = ThemeColors::from_theme(&self.config.theme);
        let styles = ThemeStyles::from_colors(&colors);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1), Constraint::Length(1)])
            .split(frame.area());

        match self.zoomed_pane {
            Some(true) => {
                let mut active_list = self.active_list.clone();
                active_list.render(frame, chunks[1], &self.todos, &styles, true, self.config.ui.compact_mode, &self.config.ui.date_format);
            }
            Some(false) => {
                let mut completed_list = self.completed_list.clone();
                completed_list.render(frame, chunks[1], &self.todos, &styles, true, self.config.ui.compact_mode, &self.config.ui.date_format);
            }
            None => {
                let split_ratio = self.config.ui.split_ratio;
                let main_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(split_ratio), Constraint::Percentage(100 - split_ratio)])
                    .split(chunks[1]);

                let mut active_list = self.active_list.clone();
                let mut completed_list = self.completed_list.clone();
                active_list.render(frame, main_chunks[0], &self.todos, &styles, self.active_pane, self.config.ui.compact_mode, &self.config.ui.date_format);
                completed_list.render(frame, main_chunks[1], &self.todos, &styles, !self.active_pane, self.config.ui.compact_mode, &self.config.ui.date_format);
            }
        }

        self.add_todo_modal.render(frame, frame.area(), &styles, &colors);

        let help_text = if self.confirmation_modal.active {
            "← → / h l Navigate options | Enter Confirm | Esc Cancel"
        } else if self.add_todo_modal.active() {
            if self.config.ui.vim_mode {
                "Title: Normal typing | Description: Vim mode (i Insert, hjkl Navigate, :w Save, :x Save&Close, :q Quit) | Tab Switch | Ctrl+Enter/Ctrl+S Save"
            } else {
                "Tab/Enter Switch fields | Ctrl+Enter/Ctrl+S Save | Esc Cancel | Arrow keys Navigate"
            }
        } else {
            if self.config.ui.vim_mode {
                "+ Add  r Edit  Space Toggle  e Expand  d Delete  Tab/hl Switch  jk Nav  t Theme  s Settings  ? Help  q Quit"
            } else {
                "+ Add  r Edit  Space Toggle  e Expand  d Delete  Tab/←→ Switch  ↑↓ Nav  t Theme  s Settings  ? Help  q Quit"
            }
        };

        let help_paragraph = Paragraph::new(help_text)
            .style(styles.help_text.bg(colors.modal_bg))
            .wrap(ratatui::widgets::Wrap { trim: false });
        frame.render_widget(help_paragraph, chunks[0]);

        let active_count = self.todos.get_active_todos().len();
        let completed_count = self.todos.get_completed_todos().len();
        let vim_status = if self.config.ui.vim_mode { "  Vim: On" } else { "" };
        let status_text = format!("TermTask {}  Active: {}  Completed: {}  Theme: {}{}",
                                 env!("CARGO_PKG_VERSION"), active_count, completed_count, self.config.theme.name(), vim_status);

        let vim_mode = if self.config.ui.vim_mode && self.input_handler.is_active() {
            self.input_handler.get_vim_mode_display()
        } else {
            None
        };

        let file_path_text = self.get_storage_context_display();
        let file_path_paragraph = Paragraph::new(file_path_text)
            .style(styles.muted.bg(colors.modal_bg))
            .alignment(Alignment::Center);
        frame.render_widget(file_path_paragraph, chunks[2]);

        VimIndicator::render_status_with_vim_mode(
            frame,
            chunks[3],
            &styles,
            &colors,
            &status_text,
            vim_mode
        );

        self.toast_manager.render(frame, frame.area(), &styles, &colors);
        self.settings.render(frame, frame.area(), &styles, &colors);
        self.help_modal.render(frame, frame.area(), &styles, &colors, self.config.ui.vim_mode);
        self.preview_modal.render(frame, frame.area(), &styles, &colors);
        self.confirmation_modal.render(frame, frame.area(), &styles, &colors);
    }

}
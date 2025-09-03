use crate::ui::{
    components::ConfirmationAction,
    AppEvent,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use anyhow::Result;

use super::App;

impl App {
    pub(super) fn convert_key_event(&self, key: KeyEvent) -> Option<AppEvent> {
        if self.confirmation_modal.active {
            match key.code {
                KeyCode::Esc => Some(AppEvent::Escape),
                KeyCode::Left | KeyCode::Char('h') => Some(AppEvent::Left),
                KeyCode::Right | KeyCode::Char('l') => Some(AppEvent::Right),
                KeyCode::Enter => Some(AppEvent::Enter),
                _ => None,
            }
        } else if self.add_todo_modal.active() {
            if self.add_todo_modal.is_insert_mode() {
                match key.code {
                    KeyCode::Enter if key.modifiers.contains(KeyModifiers::CONTROL) => Some(AppEvent::CtrlEnter),
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(AppEvent::Save),
                    KeyCode::Tab => Some(AppEvent::Tab),
                    KeyCode::BackTab => Some(AppEvent::BackTab),
                    KeyCode::Enter => Some(AppEvent::Enter),
                    KeyCode::Esc => Some(AppEvent::Escape),
                    KeyCode::Backspace => Some(AppEvent::Backspace),
                    KeyCode::Left => Some(AppEvent::Left),
                    KeyCode::Right => Some(AppEvent::Right),
                    KeyCode::Up => Some(AppEvent::Up),
                    KeyCode::Down => Some(AppEvent::Down),
                    KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => Some(AppEvent::Char(c)),
                    _ => None,
                }
            } else {
                match key.code {
                    KeyCode::Enter if key.modifiers.contains(KeyModifiers::CONTROL) => Some(AppEvent::CtrlEnter),
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(AppEvent::Save),
                    KeyCode::Tab => Some(AppEvent::Tab),
                    KeyCode::BackTab => Some(AppEvent::BackTab),
                    KeyCode::Esc => Some(AppEvent::Escape),
                    KeyCode::Char('i') => Some(AppEvent::Char('i')),
                    KeyCode::Char('a') => Some(AppEvent::Char('a')),
                    KeyCode::Char('h') | KeyCode::Left => Some(AppEvent::Left),
                    KeyCode::Char('l') | KeyCode::Right => Some(AppEvent::Right),
                    KeyCode::Char('k') | KeyCode::Up => Some(AppEvent::Up),
                    KeyCode::Char('j') | KeyCode::Down => Some(AppEvent::Down),
                    KeyCode::Enter => Some(AppEvent::Enter),
                    KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => Some(AppEvent::Char(c)),
                    _ => None,
                }
            }
        } else {
            match self.mode {
                super::AppMode::Normal => {
                    if self.settings.active {
                        match key.code {
                            KeyCode::Esc => Some(AppEvent::Escape),
                            KeyCode::Up | KeyCode::Char('k') => Some(AppEvent::Up),
                            KeyCode::Down | KeyCode::Char('j') => Some(AppEvent::Down),
                            KeyCode::Enter => Some(AppEvent::Enter),
                            _ => None,
                        }
                    } else if self.help_modal.active {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('?') => Some(AppEvent::Escape),
                            KeyCode::Up | KeyCode::Char('k') => Some(AppEvent::Up),
                            KeyCode::Down | KeyCode::Char('j') => Some(AppEvent::Down),
                            _ => None,
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                Some(AppEvent::Quit)
                            }
                            KeyCode::Char('q') => Some(AppEvent::Quit),
                            KeyCode::Up | KeyCode::Char('k') => Some(AppEvent::Up),
                            KeyCode::Down | KeyCode::Char('j') => Some(AppEvent::Down),
                            KeyCode::Enter => Some(AppEvent::Enter),
                            KeyCode::Char(' ') => Some(AppEvent::Space),
                            KeyCode::Delete | KeyCode::Char('d') => Some(AppEvent::Delete),
                            KeyCode::Esc => Some(AppEvent::Escape),
                            KeyCode::Char('t') => Some(AppEvent::ToggleTheme),
                            KeyCode::Char('s') => Some(AppEvent::OpenSettings),
                            KeyCode::Char('?') => Some(AppEvent::ShowHelp),
                            KeyCode::Char('+') => Some(AppEvent::AddTodo),
                            KeyCode::Char('e') => Some(AppEvent::ToggleExpand),
                            KeyCode::Char('E') => Some(AppEvent::ExpandAll),
                            KeyCode::Char('C') => Some(AppEvent::CollapseAll),
                            KeyCode::Char('r') => Some(AppEvent::EditTodo),
                            KeyCode::Char('p') => Some(AppEvent::PreviewTodo),
                            KeyCode::Char('=') => Some(AppEvent::IncreaseSplit),
                            KeyCode::Char('-') => Some(AppEvent::DecreaseSplit),
                            KeyCode::Tab => Some(AppEvent::SwitchPane),
                            KeyCode::Char('h') | KeyCode::Left => Some(AppEvent::GoToLeftPane),
                            KeyCode::Char('l') | KeyCode::Right => Some(AppEvent::GoToRightPane),
                            KeyCode::Char('f') => Some(AppEvent::ToggleZoom),
                            KeyCode::Char('z') if self.config.ui.vim_mode => Some(AppEvent::Char('z')),
                            KeyCode::Char('a') if self.config.ui.vim_mode => Some(AppEvent::Char('a')),
                            _ => None,
                        }
                    }
                }
            }
        }
    }

    pub(super) fn handle_event(&mut self, event: AppEvent) -> Result<()> {
        if self.confirmation_modal.active {
            return self.handle_confirmation_event(event);
        }

        if self.settings.active {
            return self.handle_settings_event(event);
        }

        if self.help_modal.active {
            return self.handle_help_event(event);
        }

        if self.preview_modal.active {
            return self.handle_preview_event(event);
        }

        if self.add_todo_modal.active() {
            return self.handle_modal_event(event);
        }

        match self.mode {
            super::AppMode::Normal => self.handle_normal_event(event)?,
        }

        Ok(())
    }

    pub(super) fn handle_normal_event(&mut self, event: AppEvent) -> Result<()> {
        let current_list = if self.active_pane { &mut self.active_list } else { &mut self.completed_list };

        match event {
            AppEvent::Quit => self.should_quit = true,
            AppEvent::Up => current_list.select_previous(&self.todos),
            AppEvent::Down => current_list.select_next(&self.todos),
            AppEvent::SwitchPane => {
                self.active_pane = !self.active_pane;
                if self.zoomed_pane.is_some() {
                    self.zoomed_pane = Some(self.active_pane);
                }
            }
            AppEvent::GoToLeftPane => {
                self.active_pane = true;
                if self.zoomed_pane.is_some() {
                    self.zoomed_pane = Some(true);
                }
            }
            AppEvent::GoToRightPane => {
                self.active_pane = false;
                if self.zoomed_pane.is_some() {
                    self.zoomed_pane = Some(false);
                }
            }
            AppEvent::Space => {
                if let Some(todo) = current_list.get_selected_todo(&self.todos) {
                    let action = ConfirmationAction::Complete(todo.title.clone());
                    self.confirmation_modal.open(action);
                }
            }
            AppEvent::ToggleExpand => {
                if let Some(todo) = current_list.get_selected_todo(&self.todos) {
                    let todo_id = todo.id;
                    self.todos.toggle_expanded(&todo_id);
                }
            }
            AppEvent::ExpandAll => {
                current_list.expand_all();
            }
            AppEvent::CollapseAll => {
                current_list.collapse_all();
            }
            AppEvent::ToggleZoom => {
                self.toggle_zoom();
            }
            AppEvent::Delete => {
                if let Some(todo) = current_list.get_selected_todo(&self.todos) {
                    let action = ConfirmationAction::Delete(todo.title.clone());
                    self.confirmation_modal.open(action);
                }
            }
            AppEvent::AddTodo => {
                self.add_todo_modal.open();
            }
            AppEvent::ToggleTheme => {
                self.config.theme = self.config.theme.next();
                self.save_config()?;
                self.toast_manager.info(format!("Theme: {}", self.config.theme.name()));
            }
            AppEvent::OpenSettings => {
                self.settings.open(&self.config.theme, &self.config.ui.date_format, self.config.ui.vim_mode, self.config.ui.compact_mode);
            }
            AppEvent::ShowHelp => {
                self.help_modal.open();
            }
            AppEvent::EditTodo => {
                if let Some(todo) = current_list.get_selected_todo(&self.todos) {
                    self.editing_todo_id = Some(todo.id);
                    self.add_todo_modal.open_with_data(&todo.title, todo.description.as_deref());
                }
            }
            AppEvent::PreviewTodo => {
                if let Some(todo) = current_list.get_selected_todo(&self.todos) {
                    self.preview_modal.open(&todo.title, todo.description.as_deref());
                }
            }
            AppEvent::IncreaseSplit => {
                if self.config.ui.split_ratio < 90 {
                    self.config.ui.split_ratio += 10;
                    let _ = self.save_config();
                }
            }
            AppEvent::DecreaseSplit => {
                if self.config.ui.split_ratio > 10 {
                    self.config.ui.split_ratio -= 10;
                    let _ = self.save_config();
                }
            }
            AppEvent::Char(c) => {
                if self.config.ui.vim_mode {
                    match c {
                        'a' => {
                            if let Some(todo) = current_list.get_selected_todo(&self.todos) {
                                let todo_id = todo.id;
                                self.todos.toggle_expanded(&todo_id);
                            }
                        }
                        _ => {
                            self.handle_vim_command(c);
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub(super) fn handle_vim_command(&mut self, c: char) {
        let current_list = if self.active_pane { &mut self.active_list } else { &mut self.completed_list };

        match (self.vim_prefix, c) {
            (None, 'z') => {
                self.vim_prefix = Some('z');
            }
            (Some('z'), 'R') => {
                current_list.expand_all();
                self.vim_prefix = None;
            }
            (Some('z'), 'M') => {
                current_list.collapse_all();
                self.vim_prefix = None;
            }
            (Some('z'), 'o') => {
                if let Some(todo) = current_list.get_selected_todo(&self.todos) {
                    let todo_id = todo.id;
                    self.todos.set_expanded(&todo_id, true);
                }
                self.vim_prefix = None;
            }
            (Some('z'), 'c') => {
                if let Some(todo) = current_list.get_selected_todo(&self.todos) {
                    let todo_id = todo.id;
                    self.todos.set_expanded(&todo_id, false);
                }
                self.vim_prefix = None;
            }
            (Some('z'), 'O') => {
                current_list.expand_all();
                self.vim_prefix = None;
            }
            (Some('z'), 'C') => {
                current_list.collapse_all();
                self.vim_prefix = None;
            }
            (Some('z'), 'A') => {
                current_list.toggle_expand_all();
                self.vim_prefix = None;
            }
            (Some('z'), 'a') => {
                if let Some(todo) = current_list.get_selected_todo(&self.todos) {
                    let todo_id = todo.id;
                    self.todos.toggle_expanded(&todo_id);
                }
                self.vim_prefix = None;
            }
            _ => {
                self.vim_prefix = None;
            }
        }
    }

    pub(super) fn toggle_zoom(&mut self) {
        match self.zoomed_pane {
            None => {
                self.zoomed_pane = Some(self.active_pane);
            }
            Some(_) => {
                self.zoomed_pane = None;
            }
        }
    }

    pub(super) fn handle_confirmation_event(&mut self, event: AppEvent) -> Result<()> {
        match event {
            AppEvent::Escape => {
                self.confirmation_modal.close();
            }
            AppEvent::Left | AppEvent::Right => {
                self.confirmation_modal.toggle_selection();
            }
            AppEvent::Enter => {
                if self.confirmation_modal.is_yes_selected() {
                    if let Some(action) = self.confirmation_modal.action.clone() {
                        self.execute_confirmation_action(action)?;
                    }
                }
                self.confirmation_modal.close();
            }
            _ => {}
        }
        Ok(())
    }

    pub(super) fn execute_confirmation_action(&mut self, action: ConfirmationAction) -> Result<()> {
        match action {
            ConfirmationAction::DiscardUnsavedChanges => {
                self.add_todo_modal.close();
                self.editing_todo_id = None;
                self.toast_manager.info("Changes discarded".to_string());
            }
            ConfirmationAction::Complete(_) | ConfirmationAction::Delete(_) => {
                let current_list = if self.active_pane { &self.active_list } else { &self.completed_list };

                if let Some(todo) = current_list.get_selected_todo(&self.todos) {
                    let todo_id = todo.id;
                    let todo_title = todo.title.clone();
                    let was_completed = todo.completed;

                    match action {
                        ConfirmationAction::Complete(_) => {
                            self.todos.toggle_todo(&todo_id);
                            self.save_todos()?;

                            if was_completed {
                                self.toast_manager.warning(format!("Reopened: {}", todo_title));
                            } else {
                                self.toast_manager.success(format!("Completed: {}", todo_title));
                            }
                        }
                        ConfirmationAction::Delete(_) => {
                            self.todos.remove_todo(&todo_id);
                            self.save_todos()?;
                            self.toast_manager.error(format!("Deleted: {}", todo_title));
                        }
                        _ => {}
                    }

                    self.active_list.validate_selection(&self.todos);
                    self.completed_list.validate_selection(&self.todos);
                }
            }
        }
        Ok(())
    }

    pub(super) fn handle_modal_event(&mut self, event: AppEvent) -> Result<()> {
        match event {
            AppEvent::Escape => {
                if self.add_todo_modal.has_unsaved_changes() {
                    let action = ConfirmationAction::DiscardUnsavedChanges;
                    self.confirmation_modal.open(action);
                } else {
                    self.add_todo_modal.close();
                    self.editing_todo_id = None;
                }
            }
            AppEvent::CtrlEnter | AppEvent::Save => {
                self.save_todo_from_modal()?;
                self.add_todo_modal.close();
                self.editing_todo_id = None;
            }
            AppEvent::Enter => {
            }
            AppEvent::Tab => {
                if self.add_todo_modal.is_in_title_mode() {
                    self.add_todo_modal.switch_to_description();
                } else {
                    self.add_todo_modal.switch_to_title();
                }
            }
            AppEvent::BackTab => {
                if self.add_todo_modal.is_in_title_mode() {
                    self.add_todo_modal.switch_to_description();
                } else {
                    self.add_todo_modal.switch_to_title();
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub(super) fn save_todo_from_modal(&mut self) -> Result<()> {
        let title = self.add_todo_modal.get_title();
        if !title.trim().is_empty() {
            let description = self.add_todo_modal.get_description();

            let was_editing = self.editing_todo_id.is_some();
            if let Some(editing_id) = self.editing_todo_id {
                if let Some(todo) = self.todos.get_todo_by_id_mut(&editing_id) {
                    todo.title = title.to_string();
                    todo.description = description;
                    self.save_todos()?;
                }
            } else {
                self.todos.add_todo(title.to_string(), description);
                self.save_todos()?;
            }

            if was_editing {
                self.toast_manager.success("Todo updated successfully!".to_string());
            } else {
                self.toast_manager.success("Todo created successfully!".to_string());
            }
        }
        Ok(())
    }

    pub(super) fn handle_help_event(&mut self, event: AppEvent) -> Result<()> {
        match event {
            AppEvent::Escape => self.help_modal.close(),
            AppEvent::Up | AppEvent::Char('k') => {
                self.help_modal.scroll_up();
            }
            AppEvent::Down | AppEvent::Char('j') => {
                self.help_modal.scroll_down();
            }
            _ => {}
        }
        Ok(())
    }

    pub(super) fn handle_preview_event(&mut self, event: AppEvent) -> Result<()> {
        match event {
            AppEvent::Escape | AppEvent::Char('p') => self.preview_modal.close(),
            AppEvent::Up | AppEvent::Char('k') => {
                self.preview_modal.scroll_up();
            }
            AppEvent::Down | AppEvent::Char('j') => {
                self.preview_modal.scroll_down();
            }
            _ => {}
        }
        Ok(())
    }

    pub(super) fn handle_settings_event(&mut self, event: AppEvent) -> Result<()> {

        if self.settings.multi_select_active {
            let key_code = match event {
                AppEvent::Escape => crossterm::event::KeyCode::Esc,
                AppEvent::Enter => crossterm::event::KeyCode::Enter,
                AppEvent::Up => crossterm::event::KeyCode::Up,
                AppEvent::Down => crossterm::event::KeyCode::Down,
                AppEvent::Char('k') => crossterm::event::KeyCode::Char('k'),
                AppEvent::Char('j') => crossterm::event::KeyCode::Char('j'),
                AppEvent::Tab => crossterm::event::KeyCode::Tab,
                AppEvent::BackTab => crossterm::event::KeyCode::BackTab,
                _ => return Ok(()),
            };

            if self.settings.handle_multi_select_input(key_code) {

                if !self.settings.multi_select_active {
                    self.config.theme = self.settings.get_theme();
                    self.config.ui.date_format = self.settings.get_date_format();
                    self.save_config()?;
                }
                return Ok(());
            }
        }


        match event {
            AppEvent::Escape => self.settings.close(),
            AppEvent::Up | AppEvent::Char('k') => self.settings.previous_item(),
            AppEvent::Down | AppEvent::Char('j') => self.settings.next_item(),
            AppEvent::Enter | AppEvent::Space => {
                let old_vim_mode = self.config.ui.vim_mode;
                let old_compact_mode = self.config.ui.compact_mode;
                self.settings.toggle_selected();
                self.config.theme = self.settings.get_theme();
                self.config.ui.date_format = self.settings.get_date_format();
                self.config.ui.vim_mode = self.settings.get_vim_mode();
                self.config.ui.compact_mode = self.settings.get_compact_mode();

                if old_vim_mode != self.config.ui.vim_mode {
                    self.add_todo_modal = crate::ui::components::AddTodoModal::new_with_vim_mode(self.config.ui.vim_mode);
                    self.toast_manager.info(format!("Vim mode: {}", if self.config.ui.vim_mode { "Enabled" } else { "Disabled" }));
                }

                if old_compact_mode != self.config.ui.compact_mode {
                    self.toast_manager.info(format!("Compact mode: {}", if self.config.ui.compact_mode { "Enabled" } else { "Disabled" }));
                }

                self.save_config()?;
            }
            _ => {}
        }
        Ok(())
    }
}
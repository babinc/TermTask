use crate::git::GitRepository;
use crate::models::{AppConfig, TodoList};
use crate::prompt::{ProjectInitializer, TodoStorageChoice};
use crate::storage::{ConfigStore, JsonStore};
use crate::ui::{
    components::{AddTodoModal, ConfirmationModal, ConfirmationAction, InputHandler, SettingsModal, TodoListComponent, ToastManager, VimIndicator},
    themes::{ThemeColors, ThemeStyles},
    AppEvent,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io::{stdout, Stdout};

pub enum AppMode {
    Normal,
}

pub struct App {
    pub todos: TodoList,
    pub config: AppConfig,
    pub mode: AppMode,
    pub active_list: TodoListComponent,
    pub completed_list: TodoListComponent,
    pub active_pane: bool,
    pub input_handler: InputHandler,
    pub settings: SettingsModal,
    pub add_todo_modal: AddTodoModal,
    pub confirmation_modal: ConfirmationModal,
    pub toast_manager: ToastManager,
    pub should_quit: bool,
    json_store: JsonStore,
    config_store: ConfigStore,
    editing_todo_id: Option<uuid::Uuid>,
    vim_prefix: Option<char>,
}

impl App {
    pub fn new() -> Result<Self> {
        Self::new_with_options(false)
    }

    pub fn new_with_options(force_global: bool) -> Result<Self> {
        let (json_store, todos) = Self::initialize_storage(force_global)?;
        let config_store = ConfigStore::new(ConfigStore::get_default_path()?);
        let config = config_store.load()?;

        let mut active_list = TodoListComponent::new();
        active_list.show_completed = false;
        let mut completed_list = TodoListComponent::new();
        completed_list.show_completed = true;

        let vim_mode = config.ui.vim_mode;
        Ok(Self {
            todos,
            config,
            mode: AppMode::Normal,
            active_list,
            completed_list,
            active_pane: true,
            input_handler: InputHandler::new(vim_mode),
            settings: SettingsModal::new(),
            add_todo_modal: AddTodoModal::new_with_vim_mode(vim_mode),
            confirmation_modal: ConfirmationModal::new(),
            toast_manager: ToastManager::new(),
            should_quit: false,
            json_store,
            config_store,
            editing_todo_id: None,
            vim_prefix: None,
        })
    }

    fn initialize_storage(force_global: bool) -> Result<(JsonStore, TodoList)> {
        if force_global {
            let store = JsonStore::new(JsonStore::get_default_path()?);
            let todos = store.load()?;
            return Ok((store, todos));
        }

        if let Some(repo) = GitRepository::find_repository() {
            if let Some(existing_todo_path) = repo.has_todo_file() {
                let store = JsonStore::new(existing_todo_path);
                let todos = store.load()?;
                return Ok((store, todos));
            }

            if let Some(choice) = ProjectInitializer::prompt_for_initialization(&repo) {
                match choice {
                    TodoStorageChoice::Global => {
                        let store = JsonStore::new(JsonStore::get_default_path()?);
                        let todos = store.load()?;
                        Ok((store, todos))
                    }
                    TodoStorageChoice::Project | TodoStorageChoice::Personal => {
                        let todo_path = ProjectInitializer::create_todo_file(&repo, choice)?;
                        let store = JsonStore::new(todo_path);
                        let todos = store.load()?;
                        Ok((store, todos))
                    }
                }
            } else {
                let store = JsonStore::new(JsonStore::get_default_path()?);
                let todos = store.load()?;
                Ok((store, todos))
            }
        } else {
            let store = JsonStore::new(JsonStore::get_default_path()?);
            let todos = store.load()?;
            Ok((store, todos))
        }
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_app(&mut terminal);

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            Clear(ClearType::All)
        )?;
        terminal.show_cursor()?;

        result
    }

    fn run_app(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;

            if crossterm::event::poll(std::time::Duration::from_millis(16))? {
                if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                    if let Some(app_event) = self.convert_key_event(key) {
                        self.handle_event(app_event)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn convert_key_event(&self, key: KeyEvent) -> Option<AppEvent> {
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
                // Insert mode - typing and basic navigation
                match key.code {
                    KeyCode::Enter if key.modifiers.contains(KeyModifiers::CONTROL) => Some(AppEvent::CtrlEnter),
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(AppEvent::Save),
                    KeyCode::Tab => Some(AppEvent::Tab),
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
                // Normal/command mode - vim navigation
                match key.code {
                    KeyCode::Enter if key.modifiers.contains(KeyModifiers::CONTROL) => Some(AppEvent::CtrlEnter),
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(AppEvent::Save),
                    KeyCode::Tab => Some(AppEvent::Tab),
                    KeyCode::Esc => Some(AppEvent::Escape),
                    KeyCode::Char('i') => Some(AppEvent::Char('i')), // Enter insert mode
                    KeyCode::Char('a') => Some(AppEvent::Char('a')), // Enter insert mode after cursor
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
                AppMode::Normal => {
                    if self.settings.active {
                        match key.code {
                            KeyCode::Esc => Some(AppEvent::Escape),
                            KeyCode::Up | KeyCode::Char('k') => Some(AppEvent::Up),
                            KeyCode::Down | KeyCode::Char('j') => Some(AppEvent::Down),
                            KeyCode::Enter => Some(AppEvent::Enter),
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
                            KeyCode::Char('+') => Some(AppEvent::AddTodo),
                            KeyCode::Char('e') => Some(AppEvent::ToggleExpand),
                            KeyCode::Char('E') => Some(AppEvent::ExpandAll),
                            KeyCode::Char('C') => Some(AppEvent::CollapseAll),
                            KeyCode::Char('r') => Some(AppEvent::EditTodo),
                            KeyCode::Char('=') => Some(AppEvent::IncreaseSplit),
                            KeyCode::Char('-') => Some(AppEvent::DecreaseSplit),
                            KeyCode::Tab => Some(AppEvent::SwitchPane),
                            KeyCode::Char('z') if self.config.ui.vim_mode => Some(AppEvent::Char('z')),
                            KeyCode::Char('a') if self.config.ui.vim_mode => Some(AppEvent::Char('a')),
                            _ => None,
                        }
                    }
                }
            }
        }
    }

    fn handle_event(&mut self, event: AppEvent) -> Result<()> {
        if self.confirmation_modal.active {
            return self.handle_confirmation_event(event);
        }

        if self.settings.active {
            return self.handle_settings_event(event);
        }

        if self.add_todo_modal.active() {
            return self.handle_modal_event(event);
        }

        match self.mode {
            AppMode::Normal => self.handle_normal_event(event)?,
        }

        Ok(())
    }

    fn handle_normal_event(&mut self, event: AppEvent) -> Result<()> {
        let current_list = if self.active_pane { &mut self.active_list } else { &mut self.completed_list };
        
        match event {
            AppEvent::Quit => self.should_quit = true,
            AppEvent::Up => current_list.select_previous(&self.todos),
            AppEvent::Down => current_list.select_next(&self.todos),
            AppEvent::SwitchPane => {
                self.active_pane = !self.active_pane;
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
                let list_name = if self.active_pane { "active" } else { "completed" };
                self.toast_manager.info(format!("Expanded all {} todos", list_name));
            }
            AppEvent::CollapseAll => {
                current_list.collapse_all();
                let list_name = if self.active_pane { "active" } else { "completed" };
                self.toast_manager.info(format!("Collapsed all {} todos", list_name));
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
                self.settings.open(&self.config.theme, self.config.ui.vim_mode, self.config.ui.compact_mode);
            }
            AppEvent::EditTodo => {
                if let Some(todo) = current_list.get_selected_todo(&self.todos) {
                    self.editing_todo_id = Some(todo.id);
                    self.add_todo_modal.open_with_data(&todo.title, todo.description.as_deref());
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
                                self.toast_manager.info("Toggled todo fold".to_string());
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

    fn handle_vim_command(&mut self, c: char) {
        let current_list = if self.active_pane { &mut self.active_list } else { &mut self.completed_list };
        
        match (self.vim_prefix, c) {
            (None, 'z') => {
                self.vim_prefix = Some('z');
            }
            (Some('z'), 'R') => {
                current_list.expand_all();
                let list_name = if self.active_pane { "active" } else { "completed" };
                self.toast_manager.info(format!("Expanded all {} todos", list_name));
                self.vim_prefix = None;
            }
            (Some('z'), 'M') => {
                current_list.collapse_all();
                let list_name = if self.active_pane { "active" } else { "completed" };
                self.toast_manager.info(format!("Collapsed all {} todos", list_name));
                self.vim_prefix = None;
            }
            (Some('z'), 'o') => {
                if let Some(todo) = current_list.get_selected_todo(&self.todos) {
                    let todo_id = todo.id;
                    self.todos.set_expanded(&todo_id, true);
                    self.toast_manager.info("Expanded todo".to_string());
                }
                self.vim_prefix = None;
            }
            (Some('z'), 'c') => {
                if let Some(todo) = current_list.get_selected_todo(&self.todos) {
                    let todo_id = todo.id;
                    self.todos.set_expanded(&todo_id, false);
                    self.toast_manager.info("Collapsed todo".to_string());
                }
                self.vim_prefix = None;
            }
            (Some('z'), 'O') => {
                current_list.expand_all();
                let list_name = if self.active_pane { "active" } else { "completed" };
                self.toast_manager.info(format!("Opened all {} todo folds", list_name));
                self.vim_prefix = None;
            }
            (Some('z'), 'C') => {
                current_list.collapse_all();
                let list_name = if self.active_pane { "active" } else { "completed" };
                self.toast_manager.info(format!("Closed all {} todo folds", list_name));
                self.vim_prefix = None;
            }
            (Some('z'), 'A') => {
                current_list.toggle_expand_all();
                let list_name = if self.active_pane { "active" } else { "completed" };
                self.toast_manager.info(format!("Toggled all {} todo folds", list_name));
                self.vim_prefix = None;
            }
            (Some('z'), 'a') => {
                if let Some(todo) = current_list.get_selected_todo(&self.todos) {
                    let todo_id = todo.id;
                    self.todos.toggle_expanded(&todo_id);
                    self.toast_manager.info("Toggled todo fold".to_string());
                }
                self.vim_prefix = None;
            }
            _ => {
                self.vim_prefix = None;
            }
        }
    }

    fn handle_confirmation_event(&mut self, event: AppEvent) -> Result<()> {
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

    fn execute_confirmation_action(&mut self, action: ConfirmationAction) -> Result<()> {
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
            }
            
            // Validate and fix selection state after the operation
            self.active_list.validate_selection(&self.todos);
            self.completed_list.validate_selection(&self.todos);
        }
        Ok(())
    }

    fn handle_modal_event(&mut self, event: AppEvent) -> Result<()> {
        match event {
            AppEvent::Escape => {
                if self.add_todo_modal.mode() == "Title" {
                    self.add_todo_modal.close();
                    self.editing_todo_id = None;
                } else if self.config.ui.vim_mode {
                    if self.add_todo_modal.is_command_mode() {
                        self.add_todo_modal.enter_normal_mode();
                    } else if self.add_todo_modal.is_insert_mode() {
                        self.add_todo_modal.enter_normal_mode();
                    } else {
                        self.add_todo_modal.close();
                        self.editing_todo_id = None;
                    }
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
                if self.add_todo_modal.is_command_mode() {
                    // Execute vim command
                    if let Some(result) = self.add_todo_modal.execute_vim_command() {
                        match result {
                            crate::ui::components::input::VimCommandResult::Save => {
                                self.save_todo_from_modal()?;
                            }
                            crate::ui::components::input::VimCommandResult::SaveAndClose => {
                                self.save_todo_from_modal()?;
                                self.add_todo_modal.close();
                                self.editing_todo_id = None;
                            }
                            crate::ui::components::input::VimCommandResult::Close => {
                                self.add_todo_modal.close();
                                self.editing_todo_id = None;
                            }
                            crate::ui::components::input::VimCommandResult::Error(msg) => {
                                self.toast_manager.error(format!("Command error: {}", msg));
                            }
                            _ => {}
                        }
                    }
                } else {
                    self.add_todo_modal.handle_enter();
                }
            }
            AppEvent::Backspace => {
                self.add_todo_modal.handle_backspace();
            }
            AppEvent::Left => {
                self.add_todo_modal.move_cursor_left();
            }
            AppEvent::Right => {
                self.add_todo_modal.move_cursor_right();
            }
            AppEvent::Up => {
                self.add_todo_modal.move_cursor_up();
            }
            AppEvent::Down => {
                self.add_todo_modal.move_cursor_down();
            }
            AppEvent::Tab => {
                if self.add_todo_modal.mode() == "Title" {
                    self.add_todo_modal.handle_enter();
                }
            }
            AppEvent::Char(c) => {
                if self.add_todo_modal.is_insert_mode() {
                    // In insert mode, all characters are typed normally
                    self.add_todo_modal.handle_char(c);
                } else if self.add_todo_modal.is_command_mode() {
                    // In command mode, all characters go to command buffer
                    self.add_todo_modal.handle_char(c);
                } else {
                    // Normal mode commands
                    match c {
                        'i' => {
                            self.add_todo_modal.enter_insert_mode();
                        }
                        'a' => {
                            self.add_todo_modal.enter_insert_mode();
                            // Move cursor right to insert after current character
                            self.add_todo_modal.move_cursor_right();
                        }
                        _ => {
                            // Handle vim motions and commands
                            self.add_todo_modal.vim_handle_normal_mode_key(c);
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn save_todo_from_modal(&mut self) -> Result<()> {
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


    fn handle_settings_event(&mut self, event: AppEvent) -> Result<()> {
        match event {
            AppEvent::Escape => self.settings.close(),
            AppEvent::Up | AppEvent::Char('k') => self.settings.previous_item(),
            AppEvent::Down | AppEvent::Char('j') => self.settings.next_item(),
            AppEvent::Enter | AppEvent::Space => {
                let old_vim_mode = self.config.ui.vim_mode;
                let old_compact_mode = self.config.ui.compact_mode;
                self.settings.toggle_selected();
                self.config.theme = self.settings.get_theme();
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

    fn save_todos(&self) -> Result<()> {
        self.json_store.save(&self.todos)
    }

    fn save_config(&self) -> Result<()> {
        self.config_store.save(&self.config)
    }

    fn draw(&mut self, frame: &mut Frame) {
        let colors = ThemeColors::from_theme(&self.config.theme);
        let styles = ThemeStyles::from_colors(&colors);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3), Constraint::Length(1), Constraint::Length(1)])
            .split(frame.area());

        let split_ratio = self.config.ui.split_ratio;
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(split_ratio), Constraint::Percentage(100 - split_ratio)])
            .split(chunks[0]);

        let mut active_list = self.active_list.clone();
        let mut completed_list = self.completed_list.clone();
        active_list.render(frame, main_chunks[0], &self.todos, &styles, self.active_pane, self.config.ui.compact_mode);
        completed_list.render(frame, main_chunks[1], &self.todos, &styles, !self.active_pane, self.config.ui.compact_mode);


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
            "+: Add  r: Edit  Space: Toggle  e: Expand  d: Delete  Tab: Switch pane  ↑↓/jk: Navigate  t: Theme  s: Settings  =/-: Split  E: Expand all  C: Collapse all  q: Quit"
        };

        let help_paragraph = Paragraph::new(help_text)
            .style(styles.help_text)
            .block(Block::default().borders(Borders::TOP));
        frame.render_widget(help_paragraph, chunks[1]);

        let active_count = self.todos.get_active_todos().len();
        let completed_count = self.todos.get_completed_todos().len();
        let vim_status = if self.config.ui.vim_mode { "  Vim: On" } else { "" };
        let status_text = format!("Active: {}  Completed: {}  Theme: {}{}", 
                                 active_count, completed_count, self.config.theme.name(), vim_status);
        
        // Only show vim mode indicator when actually in input fields
        let vim_mode = if self.config.ui.vim_mode && self.input_handler.is_active() {
            self.input_handler.get_vim_mode_display()
        } else {
            None // No vim mode indicator when just browsing
        };
        
        VimIndicator::render_status_with_vim_mode(
            frame, 
            chunks[2], 
            &styles, 
            &colors, 
            &status_text, 
            vim_mode
        );

        let file_path_text = self.get_storage_context_display();
        let file_path_paragraph = Paragraph::new(file_path_text)
            .style(styles.muted)
            .alignment(Alignment::Center);
        frame.render_widget(file_path_paragraph, chunks[3]);

        self.toast_manager.render(frame, frame.area(), &styles, &colors);
        self.settings.render(frame, frame.area(), &styles, &colors);
        self.confirmation_modal.render(frame, frame.area(), &styles, &colors);
    }

    fn get_storage_context_display(&self) -> String {
        let file_path = self.json_store.get_file_path();
        let file_name = file_path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");

        match file_name {
            "todo.json" => {
                if let Some(repo) = GitRepository::find_repository() {
                    if file_path.starts_with(&repo.root) {
                        return format!("Data: ./todo.json (shared with team)");
                    }
                }
                format!("Data: {} (project todos)", file_path.display())
            }
            ".todo.json" => {
                if let Some(repo) = GitRepository::find_repository() {
                    if file_path.starts_with(&repo.root) {
                        return format!("Data: ./.todo.json (private)");
                    }
                }
                format!("Data: {} (personal notes)", file_path.display())
            }
            "todos.json" => {
                if file_path.to_string_lossy().contains(".local/share/termtask") {
                    format!("Data: ~/.local/share/termtask/todos.json (global)")
                } else {
                    format!("Data: {} (global)", file_path.display())
                }
            }
            _ => format!("Data: {}", file_path.display())
        }
    }
}
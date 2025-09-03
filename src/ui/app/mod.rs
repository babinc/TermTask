mod events;
mod render;
mod state;

use crate::models::{AppConfig, TodoList};
use crate::storage::{ConfigStore, JsonStore};
use crate::ui::components::{AddTodoModal, ConfirmationModal, HelpModal, InputHandler, PreviewModal, SettingsModal, TodoListComponent, ToastManager};
use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io::{stdout, Stdout};

#[derive(Debug, Clone, PartialEq)]
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
    pub help_modal: HelpModal,
    pub preview_modal: PreviewModal,
    pub toast_manager: ToastManager,
    pub should_quit: bool,
    json_store: JsonStore,
    config_store: ConfigStore,
    editing_todo_id: Option<uuid::Uuid>,
    vim_prefix: Option<char>,
    zoomed_pane: Option<bool>,
}

impl App {
    pub fn new() -> Result<Self> {
        Self::new_with_options(false, None)
    }

    pub fn new_with_options(force_global: bool, custom_path: Option<String>) -> Result<Self> {
        let (json_store, todos) = Self::initialize_storage(force_global, custom_path)?;
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
            help_modal: HelpModal::new(),
            preview_modal: PreviewModal::new(),
            toast_manager: ToastManager::new(),
            should_quit: false,
            json_store,
            config_store,
            editing_todo_id: None,
            vim_prefix: None,
            zoomed_pane: None,
        })
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
        terminal.draw(|frame| self.draw(frame))?;

        let mut last_render = std::time::Instant::now();
        let render_interval = std::time::Duration::from_millis(100);

        while !self.should_quit {
            let timeout = std::time::Duration::from_millis(50);
            let needs_periodic_render = self.toast_manager.has_active_toasts();

            if crossterm::event::poll(timeout)? {
                if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                    if key.kind != crossterm::event::KeyEventKind::Press {
                        continue;
                    }

                    if self.add_todo_modal.active() && self.config.ui.vim_mode && !self.confirmation_modal.active {
                        let should_handle_normally = match key.code {
                            crossterm::event::KeyCode::Esc => {
                                self.add_todo_modal.should_escape_close_modal()
                            },
                            crossterm::event::KeyCode::Tab | crossterm::event::KeyCode::BackTab => true,
                            crossterm::event::KeyCode::Enter if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => true,
                            crossterm::event::KeyCode::Char('s') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => true,
                            _ => false
                        };

                        if should_handle_normally {
                            if let Some(app_event) = self.convert_key_event(key) {
                                self.handle_event(app_event)?;
                            }
                        } else {
                            if let Some(vim_cmd) = self.add_todo_modal.handle_key_event(key) {
                                match vim_cmd {
                                    crate::ui::components::input::VimCommand::Save => {
                                        self.save_todo_from_modal()?;
                                    }
                                    crate::ui::components::input::VimCommand::SaveAndClose => {
                                        self.save_todo_from_modal()?;
                                        self.add_todo_modal.close();
                                        self.editing_todo_id = None;
                                    }
                                    crate::ui::components::input::VimCommand::Quit => {
                                        if self.add_todo_modal.has_unsaved_changes() {
                                            let action = crate::ui::components::ConfirmationAction::DiscardUnsavedChanges;
                                            self.confirmation_modal.open(action);
                                        } else {
                                            self.add_todo_modal.close();
                                            self.editing_todo_id = None;
                                        }
                                    }
                                }
                            }
                        }

                        terminal.draw(|frame| self.draw(frame))?;
                    } else {
                        if let Some(app_event) = self.convert_key_event(key) {
                            self.handle_event(app_event)?;
                            terminal.draw(|frame| self.draw(frame))?;
                        }
                    }
                }
            } else {
                if needs_periodic_render && last_render.elapsed() >= render_interval {
                    terminal.draw(|frame| self.draw(frame))?;
                    last_render = std::time::Instant::now();
                }
            }
        }
        Ok(())
    }

}
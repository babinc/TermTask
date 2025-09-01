use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    Quit,
    Up,
    Down,
    Enter,
    Space,
    Delete,
    Tab,
    Escape,
    ToggleTheme,
    OpenSettings,
    AddTodo,
    EditTodo,
    ToggleExpand,
    ExpandAll,
    CollapseAll,
    IncreaseSplit,
    DecreaseSplit,
    SwitchPane,
    GoToLeftPane,
    GoToRightPane,
    Backspace,
    Left,
    Right,
    CtrlEnter,
    Save,
    ShowHelp,
    Char(char),
}

pub struct EventHandler {
    last_tick: Instant,
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        Self {
            last_tick: Instant::now(),
            tick_rate,
        }
    }

    pub fn next_event(&mut self) -> Result<Option<AppEvent>, anyhow::Error> {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                return Ok(self.handle_key_event(key));
            }
        }

        if self.last_tick.elapsed() >= self.tick_rate {
            self.last_tick = Instant::now();
        }

        Ok(None)
    }

    fn handle_key_event(&self, key: KeyEvent) -> Option<AppEvent> {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                Some(AppEvent::Quit)
            }
            KeyCode::Char('q') => Some(AppEvent::Quit),
            KeyCode::Up | KeyCode::Char('k') => Some(AppEvent::Up),
            KeyCode::Down | KeyCode::Char('j') => Some(AppEvent::Down),
            KeyCode::Enter if key.modifiers.contains(KeyModifiers::CONTROL) => Some(AppEvent::CtrlEnter),
            KeyCode::Enter => Some(AppEvent::Enter),
            KeyCode::Char(' ') => Some(AppEvent::Space),
            KeyCode::Delete | KeyCode::Char('d') => Some(AppEvent::Delete),
            KeyCode::Tab => Some(AppEvent::Tab),
            KeyCode::Esc => Some(AppEvent::Escape),
            KeyCode::Char('t') => Some(AppEvent::ToggleTheme),
            KeyCode::Char('s') => Some(AppEvent::OpenSettings),
            KeyCode::Char('?') => Some(AppEvent::ShowHelp),
            KeyCode::Char('+') => Some(AppEvent::AddTodo),
            KeyCode::Char('e') => Some(AppEvent::ToggleExpand),
            KeyCode::Char('E') => Some(AppEvent::ExpandAll),
            KeyCode::Char('C') => Some(AppEvent::CollapseAll),
            KeyCode::Backspace => Some(AppEvent::Backspace),
            KeyCode::Left => Some(AppEvent::Left),
            KeyCode::Right => Some(AppEvent::Right),
            KeyCode::Char(c) => Some(AppEvent::Char(c)),
            _ => None,
        }
    }
}
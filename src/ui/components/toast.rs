use crate::ui::themes::{ThemeStyles, ThemeColors};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::Style,
    text::Text,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum ToastLevel {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub level: ToastLevel,
    pub created_at: Instant,
    pub duration: Duration,
    pub id: usize,
}

impl Toast {
    pub fn new(message: String, level: ToastLevel) -> Self {
        Self {
            message,
            level,
            created_at: Instant::now(),
            duration: Duration::from_secs(3),
            id: 0,
        }
    }

    pub fn success(message: String) -> Self {
        Self::new(message, ToastLevel::Success)
    }

    pub fn info(message: String) -> Self {
        Self::new(message, ToastLevel::Info)
    }

    pub fn warning(message: String) -> Self {
        Self::new(message, ToastLevel::Warning)
    }

    pub fn error(message: String) -> Self {
        Self::new(message, ToastLevel::Error)
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.duration
    }

    pub fn get_progress(&self) -> f32 {
        let elapsed = self.created_at.elapsed().as_secs_f32();
        let total = self.duration.as_secs_f32();
        (elapsed / total).min(1.0)
    }

    pub fn get_opacity(&self) -> f32 {
        let progress = self.get_progress();
        if progress < 0.1 {
            progress / 0.1
        } else if progress > 0.9 {
            1.0 - ((progress - 0.9) / 0.1)
        } else {
            1.0
        }
    }
}

pub struct ToastManager {
    toasts: Vec<Toast>,
    next_id: usize,
}

impl ToastManager {
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_toast(&mut self, mut toast: Toast) {
        toast.id = self.next_id;
        self.next_id += 1;
        self.toasts.push(toast);
    }

    pub fn success(&mut self, message: String) {
        self.add_toast(Toast::success(message));
    }

    pub fn info(&mut self, message: String) {
        self.add_toast(Toast::info(message));
    }

    pub fn warning(&mut self, message: String) {
        self.add_toast(Toast::warning(message));
    }

    pub fn error(&mut self, message: String) {
        self.add_toast(Toast::error(message));
    }

    pub fn update(&mut self) {
        self.toasts.retain(|toast| !toast.is_expired());
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, styles: &ThemeStyles, colors: &ThemeColors) {
        self.update();
        
        let toast_width = area.width.min(50);
        let toast_height = 4;
        let spacing = 1;
        
        for (index, toast) in self.toasts.iter().enumerate() {
            let y_offset = index as u16 * (toast_height + spacing);
            
            if y_offset + toast_height > area.height {
                break;
            }

            let toast_area = Rect {
                x: area.width.saturating_sub(toast_width + 2),
                y: area.y + y_offset + 1,
                width: toast_width,
                height: toast_height,
            };

            if toast_area.x < area.x || toast_area.y < area.y {
                continue;
            }

            frame.render_widget(Clear, toast_area);

            let (border_style, text_style, icon) = match toast.level {
                ToastLevel::Success => (styles.accent, styles.normal, ""),
                ToastLevel::Info => (styles.title, styles.normal, ""),
                ToastLevel::Warning => (Style::default().fg(colors.warning), styles.normal, ""),
                ToastLevel::Error => (Style::default().fg(colors.error), styles.normal, ""),
            };

            let opacity = toast.get_opacity();
            let adjusted_style = if opacity < 1.0 {
                border_style
            } else {
                border_style
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(adjusted_style)
                .style(styles.normal.bg(colors.background));

            let content = format!("{} {}", icon, toast.message);
            let paragraph = Paragraph::new(Text::from(content))
                .style(text_style)
                .wrap(Wrap { trim: true })
                .alignment(Alignment::Center)
                .block(block);

            frame.render_widget(paragraph, toast_area);
        }
    }
}
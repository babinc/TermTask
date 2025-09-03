use crate::ui::styling::{ThemeStyles, ThemeColors};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use tui_markdown::from_str;

pub struct PreviewModal {
    pub active: bool,
    title: String,
    description: String,
    scroll_offset: usize,
}

impl PreviewModal {
    pub fn new() -> Self {
        Self {
            active: false,
            title: String::new(),
            description: String::new(),
            scroll_offset: 0,
        }
    }

    pub fn open(&mut self, title: &str, description: Option<&str>) {
        self.active = true;
        self.title = title.to_string();
        self.description = description.unwrap_or("").to_string();
        self.scroll_offset = 0;
    }

    pub fn close(&mut self) {
        self.active = false;
        self.scroll_offset = 0;
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset += 1;
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, styles: &ThemeStyles, colors: &ThemeColors) {
        if !self.active {
            return;
        }

        let width = (area.width as f32 * 0.8) as u16;
        let height = (area.height as f32 * 0.8) as u16;
        let popup_area = Rect {
            x: (area.width - width) / 2,
            y: (area.height - height) / 2,
            width,
            height,
        };

        frame.render_widget(Clear, popup_area);

        let modal_bg = Block::default()
            .style(Style::default().bg(colors.modal_bg));
        frame.render_widget(modal_bg, popup_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(popup_area);

        let title_block = Block::default()
            .title(" Preview ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors.modal_border))
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg));

        let title_paragraph = Paragraph::new(self.title.as_str())
            .block(title_block)
            .style(Style::default().fg(colors.primary).bg(colors.modal_bg))
            .alignment(Alignment::Center);

        frame.render_widget(title_paragraph, chunks[0]);

        let content_block = Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            .border_style(Style::default().fg(colors.modal_border))
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg));

        let markdown_text = if !self.description.is_empty() {
            from_str(&self.description)
        } else {
            ratatui::text::Text::from("(No description)")
        };

        let content_height = chunks[1].height as usize;
        let total_lines = markdown_text.lines.len();
        let visible_lines = content_height.saturating_sub(2);

        let scroll_offset = self.scroll_offset.min(total_lines.saturating_sub(visible_lines));
        let start = scroll_offset;
        let end = (start + visible_lines).min(total_lines);

        let visible_text = if total_lines <= visible_lines {
            markdown_text
        } else {
            let visible_lines: Vec<_> = markdown_text.lines[start..end].to_vec();
            ratatui::text::Text::from(visible_lines)
        };

        let content_paragraph = Paragraph::new(visible_text)
            .block(content_block)
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg))
            .wrap(Wrap { trim: false });

        frame.render_widget(content_paragraph, chunks[1]);

        let footer_text = if total_lines > visible_lines {
            format!("Press Esc or p to close | [{}/{}] Use ↑/↓ or j/k to scroll",
                (start + visible_lines).min(total_lines),
                total_lines)
        } else {
            "Press Esc or p to close".to_string()
        };

        let footer_paragraph = Paragraph::new(footer_text)
            .style(Style::default().fg(colors.muted).bg(colors.modal_bg))
            .alignment(Alignment::Center);

        frame.render_widget(footer_paragraph, chunks[2]);
    }
}
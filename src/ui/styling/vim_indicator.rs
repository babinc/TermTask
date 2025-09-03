use super::{ThemeColors, ThemeStyles};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Paragraph},
    Frame,
};

pub struct VimIndicator;

impl VimIndicator {
    pub fn render_status_with_vim_mode(
        frame: &mut Frame,
        area: Rect,
        styles: &ThemeStyles,
        colors: &ThemeColors,
        status_text: &str,
        vim_mode: Option<&str>,
    ) {
        if let Some(mode) = vim_mode {
            let status_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length((mode.len() + 2) as u16),
                    Constraint::Min(0),
                ])
                .split(area);

            let vim_bg_color = match mode {
                "NORMAL" => colors.vim_normal_bg,
                "INSERT" => colors.vim_insert_bg,
                "VISUAL" | "V-LINE" | "V-BLOCK" => colors.vim_visual_bg,
                "COMMAND" => colors.vim_command_bg,
                _ => colors.vim_normal_bg,
            };

            let vim_indicator = Paragraph::new(format!(" {} ", mode))
                .style(Style::default()
                    .fg(colors.vim_text)
                    .bg(vim_bg_color)
                    .add_modifier(ratatui::style::Modifier::BOLD))
                .block(Block::default());

            frame.render_widget(vim_indicator, status_chunks[0]);

            let status_paragraph = Paragraph::new(format!(" {}", status_text))
                .style(styles.status_bar);
            frame.render_widget(status_paragraph, status_chunks[1]);
        } else {
            let status_paragraph = Paragraph::new(status_text)
                .style(styles.status_bar);
            frame.render_widget(status_paragraph, area);
        }
    }
}
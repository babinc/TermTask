use crate::ui::themes::{ThemeColors, ThemeStyles};
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
        vim_mode: Option<&str>, // None for non-vim mode, Some("NORMAL"/"INSERT") for vim mode
    ) {
        if let Some(mode) = vim_mode {
            // Split the status area to make room for vim indicator on the left
            let status_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length((mode.len() + 2) as u16), // Vim mode indicator
                    Constraint::Min(0),                           // Rest of status
                ])
                .split(area);

            // Render vim mode indicator with powerline style
            let vim_bg_color = match mode {
                "NORMAL" => colors.vim_normal_bg,
                "INSERT" => colors.vim_insert_bg,
                "VISUAL" | "V-LINE" | "V-BLOCK" => colors.vim_visual_bg,
                "COMMAND" => colors.vim_command_bg,
                _ => colors.vim_normal_bg,
            };

            // Create powerline-style indicator with bold text
            let vim_indicator = Paragraph::new(format!(" {} ", mode))
                .style(Style::default()
                    .fg(colors.vim_text)
                    .bg(vim_bg_color)
                    .add_modifier(ratatui::style::Modifier::BOLD))
                .block(Block::default());

            frame.render_widget(vim_indicator, status_chunks[0]);

            // Render rest of status
            let status_paragraph = Paragraph::new(format!(" {}", status_text))
                .style(styles.status_bar);
            frame.render_widget(status_paragraph, status_chunks[1]);
        } else {
            // No vim mode, render normal status
            let status_paragraph = Paragraph::new(status_text)
                .style(styles.status_bar);
            frame.render_widget(status_paragraph, area);
        }
    }
}
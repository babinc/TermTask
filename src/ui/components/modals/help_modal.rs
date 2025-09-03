use crate::ui::styling::{ThemeStyles, ThemeColors};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub key: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub vim_only: bool,
    pub normal_only: bool,
}

pub struct HelpModal {
    pub active: bool,
    scroll_offset: usize,
}

impl HelpModal {
    pub fn new() -> Self {
        Self {
            active: false,
            scroll_offset: 0,
        }
    }

    pub fn open(&mut self) {
        self.active = true;
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

    fn get_key_bindings(vim_mode: bool) -> Vec<KeyBinding> {
        let mut bindings = vec![
            KeyBinding { key: "↑/k", description: "Move up", category: "Navigation", vim_only: false, normal_only: false },
            KeyBinding { key: "↓/j", description: "Move down", category: "Navigation", vim_only: false, normal_only: false },
            KeyBinding { key: "Tab", description: "Toggle between panes", category: "Navigation", vim_only: false, normal_only: false },
            KeyBinding { key: "h/←", description: "Go to left pane (active)", category: "Navigation", vim_only: false, normal_only: false },
            KeyBinding { key: "l/→", description: "Go to right pane (completed)", category: "Navigation", vim_only: false, normal_only: false },
            KeyBinding { key: "Enter", description: "Select/Edit item", category: "Navigation", vim_only: false, normal_only: false },
            KeyBinding { key: "+", description: "Add new todo", category: "Todo Management", vim_only: false, normal_only: false },
            KeyBinding { key: "r", description: "Edit selected todo", category: "Todo Management", vim_only: false, normal_only: false },
            KeyBinding { key: "p", description: "Preview todo with markdown", category: "Todo Management", vim_only: false, normal_only: false },
            KeyBinding { key: "Space", description: "Toggle todo completion", category: "Todo Management", vim_only: false, normal_only: false },
            KeyBinding { key: "d", description: "Delete selected todo", category: "Todo Management", vim_only: false, normal_only: false },
            KeyBinding { key: "e", description: "Expand/collapse description", category: "Todo Management", vim_only: false, normal_only: false },
            KeyBinding { key: "E", description: "Expand all descriptions", category: "View Options", vim_only: false, normal_only: false },
            KeyBinding { key: "C", description: "Collapse all descriptions", category: "View Options", vim_only: false, normal_only: false },
            KeyBinding { key: "=/+", description: "Increase split ratio", category: "View Options", vim_only: false, normal_only: false },
            KeyBinding { key: "-", description: "Decrease split ratio", category: "View Options", vim_only: false, normal_only: false },
            KeyBinding { key: "f", description: "Focus/zoom pane", category: "View Options", vim_only: false, normal_only: false },
            KeyBinding { key: "t", description: "Toggle theme quickly", category: "Settings", vim_only: false, normal_only: false },
            KeyBinding { key: "s", description: "Open settings modal", category: "Settings", vim_only: false, normal_only: false },
            KeyBinding { key: "Tab/Enter", description: "Switch fields", category: "Modal Controls", vim_only: false, normal_only: true },
            KeyBinding { key: "Ctrl+Enter/Ctrl+S", description: "Save", category: "Modal Controls", vim_only: false, normal_only: true },
            KeyBinding { key: "Esc", description: "Close modal/Cancel", category: "Modal Controls", vim_only: false, normal_only: true },
            KeyBinding { key: "Arrow keys", description: "Navigate", category: "Modal Controls", vim_only: false, normal_only: true },
            KeyBinding { key: "i", description: "Enter insert mode", category: "Modal Controls (Vim)", vim_only: true, normal_only: false },
            KeyBinding { key: "Esc", description: "Enter normal mode", category: "Modal Controls (Vim)", vim_only: true, normal_only: false },
            KeyBinding { key: "hjkl", description: "Navigate (normal mode)", category: "Modal Controls (Vim)", vim_only: true, normal_only: false },
            KeyBinding { key: ":w", description: "Save", category: "Modal Controls (Vim)", vim_only: true, normal_only: false },
            KeyBinding { key: ":x", description: "Save and close", category: "Modal Controls (Vim)", vim_only: true, normal_only: false },
            KeyBinding { key: ":q", description: "Quit/close", category: "Modal Controls (Vim)", vim_only: true, normal_only: false },
            KeyBinding { key: "0", description: "Go to line start", category: "Modal Controls (Vim)", vim_only: true, normal_only: false },
            KeyBinding { key: "$", description: "Go to line end", category: "Modal Controls (Vim)", vim_only: true, normal_only: false },
            KeyBinding { key: "w/b", description: "Word forward/backward", category: "Modal Controls (Vim)", vim_only: true, normal_only: false },
            KeyBinding { key: "G", description: "Go to last line", category: "Modal Controls (Vim)", vim_only: true, normal_only: false },
            KeyBinding { key: "gg", description: "Go to first line", category: "Modal Controls (Vim)", vim_only: true, normal_only: false },
            KeyBinding { key: "dd", description: "Delete line", category: "Modal Controls (Vim)", vim_only: true, normal_only: false },
            KeyBinding { key: "yy", description: "Yank (copy) line", category: "Modal Controls (Vim)", vim_only: true, normal_only: false },
            KeyBinding { key: "p", description: "Paste after cursor", category: "Modal Controls (Vim)", vim_only: true, normal_only: false },
            KeyBinding { key: "v/V", description: "Visual/Visual line mode", category: "Modal Controls (Vim)", vim_only: true, normal_only: false },
            KeyBinding { key: "?", description: "Show this help", category: "Application", vim_only: false, normal_only: false },
            KeyBinding { key: "q", description: "Quit application", category: "Application", vim_only: false, normal_only: false },
            KeyBinding { key: "Ctrl+C", description: "Force quit", category: "Application", vim_only: false, normal_only: false },
        ];
        bindings.retain(|binding| {
            if vim_mode {
                !binding.normal_only
            } else {
                !binding.vim_only
            }
        });

        bindings
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, styles: &ThemeStyles, colors: &ThemeColors, vim_mode: bool) {
        if !self.active {
            return;
        }

        let width = (area.width as f32 * 0.9) as u16;
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
            .title(" TermTask Help ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors.modal_border))
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg));

        let mode_text = if vim_mode { "Keyboard shortcuts and commands (Vim Mode)" } else { "Keyboard shortcuts and commands (Normal Mode)" };
        let title_paragraph = Paragraph::new(mode_text)
            .block(title_block)
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg))
            .alignment(Alignment::Center);

        frame.render_widget(title_paragraph, chunks[0]);
        let bindings = Self::get_key_bindings(vim_mode);
        let mut content_lines = Vec::new();
        let mut current_category = "";

        for binding in bindings {
            if binding.category != current_category {
                if !current_category.is_empty() {
                    content_lines.push(String::new());
                }
                content_lines.push(format!("{}:", binding.category));
                current_category = binding.category;
            }
            content_lines.push(format!("  {:12} - {}", binding.key, binding.description));
        }

        let content_height = chunks[1].height as usize;
        let total_lines = content_lines.len();
        let visible_lines = content_height.saturating_sub(2);

        let scroll_offset = self.scroll_offset.min(total_lines.saturating_sub(visible_lines));
        let start = scroll_offset;
        let end = (start + visible_lines).min(total_lines);
        let visible_content = content_lines[start..end].join("\n");

        let content = if total_lines > visible_lines {
            format!("{}\n\n[{}/{}] Use ↑/↓ or j/k to scroll",
                visible_content,
                (start + visible_lines).min(total_lines),
                total_lines)
        } else {
            visible_content
        };

        let content_block = Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            .border_style(Style::default().fg(colors.modal_border))
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg));

        let content_paragraph = Paragraph::new(content)
            .block(content_block)
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg))
            .wrap(Wrap { trim: false });

        frame.render_widget(content_paragraph, chunks[1]);
        let footer_text = "Press Esc or ? to close";
        let footer_paragraph = Paragraph::new(footer_text)
            .style(Style::default().fg(colors.muted).bg(colors.modal_bg))
            .alignment(Alignment::Center);

        frame.render_widget(footer_paragraph, chunks[2]);
    }
}
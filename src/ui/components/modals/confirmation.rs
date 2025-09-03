use crate::ui::styling::{ThemeStyles, ThemeColors};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

#[derive(Debug, Clone)]
pub enum ConfirmationAction {
    Complete(String),
    Delete(String),
    DiscardUnsavedChanges,
}

impl ConfirmationAction {
    pub fn title(&self) -> &str {
        match self {
            ConfirmationAction::Complete(_) => "Complete Todo",
            ConfirmationAction::Delete(_) => "Delete Todo",
            ConfirmationAction::DiscardUnsavedChanges => "Discard Changes",
        }
    }

    pub fn message(&self) -> String {
        match self {
            ConfirmationAction::Complete(title) => format!("Complete todo \"{}\"?", title),
            ConfirmationAction::Delete(title) => format!("Delete todo \"{}\"?", title),
            ConfirmationAction::DiscardUnsavedChanges => "You have unsaved changes. Discard them?".to_string(),
        }
    }
}

pub struct ConfirmationModal {
    pub active: bool,
    pub action: Option<ConfirmationAction>,
    pub selected_option: bool,
}

impl ConfirmationModal {
    pub fn new() -> Self {
        Self {
            active: false,
            action: None,
            selected_option: true,
        }
    }

    pub fn open(&mut self, action: ConfirmationAction) {
        self.active = true;
        self.action = Some(action);
        self.selected_option = true;
    }

    pub fn close(&mut self) {
        self.active = false;
        self.action = None;
        self.selected_option = true;
    }

    pub fn toggle_selection(&mut self) {
        self.selected_option = !self.selected_option;
    }

    pub fn is_yes_selected(&self) -> bool {
        self.selected_option
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, styles: &ThemeStyles, colors: &ThemeColors) {
        if !self.active {
            return;
        }

        let Some(action) = &self.action else {
            return;
        };

        let width = (area.width as f32 * 0.5) as u16;
        let height = 7;
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

        let modal_block = Block::default()
            .title(format!(" {} ", action.title()))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors.modal_border))
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg));

        let inner = modal_block.inner(popup_area);
        frame.render_widget(modal_block, popup_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(inner);

        let message_paragraph = Paragraph::new(action.message())
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg))
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center);

        frame.render_widget(message_paragraph, chunks[0]);

        let options_text = if self.selected_option {
            "[Yes]  No"
        } else {
            " Yes  [No]"
        };

        let options_paragraph = Paragraph::new(options_text)
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg))
            .alignment(Alignment::Center);

        frame.render_widget(options_paragraph, chunks[2]);

        let help_text = "← → Navigate | Enter Confirm | Esc Cancel";
        let help_paragraph = Paragraph::new(help_text)
            .style(Style::default().fg(colors.muted).bg(colors.modal_bg))
            .alignment(Alignment::Center);

        frame.render_widget(help_paragraph, chunks[3]);
    }
}
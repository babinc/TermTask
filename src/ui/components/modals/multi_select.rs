use crate::ui::styling::{ThemeStyles, ThemeColors};
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Clear, List, ListItem},
    Frame,
};

pub trait MultiSelectItem: Clone {
    fn name(&self) -> &str;
    fn all() -> Vec<Self>;
}

pub struct MultiSelect<T: MultiSelectItem> {
    pub active: bool,
    pub selected_index: usize,
    pub items: Vec<T>,
    pub current_value: T,
}

impl<T: MultiSelectItem> MultiSelect<T> {
    pub fn new(_title: String, current_value: T) -> Self {
        let items = T::all();
        let selected_index = items
            .iter()
            .position(|item| item.name() == current_value.name())
            .unwrap_or(0);

        Self {
            active: false,
            selected_index,
            items,
            current_value,
        }
    }

    pub fn open(&mut self) {
        self.active = true;
        self.selected_index = self.items
            .iter()
            .position(|item| item.name() == self.current_value.name())
            .unwrap_or(0);
    }

    pub fn close(&mut self) {
        self.active = false;
    }

    pub fn next_item(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.items.len();
        }
    }

    pub fn previous_item(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.items.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn select_current(&mut self) -> T {
        if !self.items.is_empty() && self.selected_index < self.items.len() {
            self.current_value = self.items[self.selected_index].clone();
        }
        self.close();
        self.current_value.clone()
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, styles: &ThemeStyles, colors: &ThemeColors) {
        if !self.active {
            return;
        }

        let width = (area.width as f32 * 0.4) as u16;
        let height = (self.items.len() as u16 + 2).min(area.height - 2);
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

        let list_items: Vec<ListItem> = self.items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let prefix = if i == self.selected_index { "► " } else { "  " };
                let text = format!("{}{}", prefix, item.name());
                ListItem::new(text)
            })
            .collect();

        let items_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors.modal_border))
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg));

        let items_list = List::new(list_items)
            .block(items_block)
            .style(Style::default().fg(colors.foreground).bg(colors.modal_bg));

        frame.render_widget(items_list, popup_area);
    }
}
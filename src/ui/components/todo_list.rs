use crate::models::{TodoItem, TodoList, format_datetime};
use crate::models::config::DateFormat;
use crate::ui::styling::ThemeStyles;
use ratatui::{
    layout::Rect,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

#[derive(Clone)]
pub struct TodoListComponent {
    pub state: ListState,
    pub show_completed: bool,
    pub expand_all: bool,
}

impl TodoListComponent {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            state,
            show_completed: false,
            expand_all: false,
        }
    }

    pub fn toggle_view(&mut self) {
        self.show_completed = !self.show_completed;
        self.state.select(Some(0));
    }

    pub fn select_next(&mut self, todos: &TodoList) {
        let items = if self.show_completed {
            todos.get_completed_todos()
        } else {
            todos.get_active_todos()
        };

        if items.is_empty() {
            self.state.select(None);
            return;
        }

        let selected = self.state.selected().unwrap_or(0);
        let next = if selected >= items.len() - 1 {
            0
        } else {
            selected + 1
        };
        self.state.select(Some(next));
    }

    pub fn select_previous(&mut self, todos: &TodoList) {
        let items = if self.show_completed {
            todos.get_completed_todos()
        } else {
            todos.get_active_todos()
        };

        if items.is_empty() {
            self.state.select(None);
            return;
        }

        let selected = self.state.selected().unwrap_or(0);
        let previous = if selected == 0 {
            items.len() - 1
        } else {
            selected - 1
        };
        self.state.select(Some(previous));
    }

    pub fn get_selected_todo<'a>(&self, todos: &'a TodoList) -> Option<&'a TodoItem> {
        let selected = self.state.selected()?;
        let items = if self.show_completed {
            todos.get_completed_todos()
        } else {
            todos.get_active_todos()
        };
        items.get(selected).copied()
    }

    pub fn expand_all(&mut self) {
        self.expand_all = true;
    }

    pub fn collapse_all(&mut self) {
        self.expand_all = false;
    }

    pub fn toggle_expand_all(&mut self) {
        self.expand_all = !self.expand_all;
    }

    pub fn validate_selection(&mut self, todos: &TodoList) {
        let items = if self.show_completed {
            todos.get_completed_todos()
        } else {
            todos.get_active_todos()
        };

        if items.is_empty() {
            self.state.select(None);
        } else if self.state.selected().is_none() {
            self.state.select(Some(0));
        } else if let Some(selected) = self.state.selected() {
            if selected >= items.len() {
                self.state.select(Some(if items.len() > 0 { items.len() - 1 } else { 0 }));
            }
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, todos: &TodoList, styles: &ThemeStyles, is_active: bool, compact_mode: bool, date_format: &DateFormat) {
        let items = if self.show_completed {
            todos.get_completed_todos()
        } else {
            todos.get_active_todos()
        };

        if items.is_empty() {
            self.state.select(None);
        } else if self.state.selected().is_none() {
            self.state.select(Some(0));
        } else if let Some(selected) = self.state.selected() {
            if selected >= items.len() {
                self.state.select(Some(0));
            }
        }

        let list_items: Vec<ListItem> = items
            .iter()
            .enumerate()
            .map(|(_index, todo)| {
                let mut lines = vec![];

                if !compact_mode {
                    lines.push(Line::from(""));
                }

                let mut spans = vec![];

                if todo.completed {
                    spans.push(Span::styled("✓ ", styles.completed));
                    spans.push(Span::styled(&todo.title, styles.completed));

                    if let Some(completed_at) = todo.completed_at {
                        let formatted_date = format_datetime(&completed_at, date_format);
                        spans.push(Span::styled(
                            format!(" ({})", formatted_date),
                            styles.muted
                        ));
                    }
                } else {
                    spans.push(Span::styled("○ ", styles.normal));
                    spans.push(Span::styled(&todo.title, styles.title));
                }

                if todo.has_description() {
                    spans.push(Span::styled(" [+]", styles.muted));
                }


                lines.push(Line::from(spans));

                if !compact_mode {
                    lines.push(Line::from(""));
                }

                let should_show_description = (todo.expanded || self.expand_all) && todo.has_description();
                if should_show_description {
                    if let Some(desc) = &todo.description {
                        let desc_lines: Vec<Line> = desc
                            .lines()
                            .map(|line| {
                                if line.trim_start().starts_with("- ") {
                                    Line::from(vec![
                                        Span::styled("  ", styles.description),
                                        Span::styled("• ", styles.accent),
                                        Span::styled(
                                            line.trim_start().strip_prefix("- ").unwrap_or(line),
                                            styles.description,
                                        ),
                                    ])
                                } else if line.trim_start().starts_with("* ") {
                                    Line::from(vec![
                                        Span::styled("  ", styles.description),
                                        Span::styled("• ", styles.accent),
                                        Span::styled(
                                            line.trim_start().strip_prefix("* ").unwrap_or(line),
                                            styles.description,
                                        ),
                                    ])
                                } else {
                                    Line::from(Span::styled(
                                        format!("  {}", line),
                                        styles.description,
                                    ))
                                }
                            })
                            .collect();
                        lines.extend(desc_lines);
                        if !compact_mode {
                            lines.push(Line::from(""));
                        }
                    }
                }

                ListItem::new(Text::from(lines))
            })
            .collect();

        let title = if self.show_completed {
            if is_active {
                format!("► Completed Todos ({})", items.len())
            } else {
                format!("Completed Todos ({})", items.len())
            }
        } else {
            if is_active {
                format!("► Active Todos ({})", items.len())
            } else {
                format!("Active Todos ({})", items.len())
            }
        };

        let border_style = if is_active {
            styles.title
        } else {
            styles.border
        };

        let list = List::new(list_items)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .style(border_style),
            )
            .style(styles.normal)
            .highlight_style(styles.selected);

        frame.render_stateful_widget(list, area, &mut self.state);
    }
}
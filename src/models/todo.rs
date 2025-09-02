use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::config::DateFormat;

const CURRENT_VERSION: u32 = 1;

fn default_version() -> u32 {
    CURRENT_VERSION
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TodoItem {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip)]
    pub expanded: bool,
}

impl TodoItem {
    pub fn new(title: String, description: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            completed: false,
            created_at: Utc::now(),
            completed_at: None,
            expanded: false,
        }
    }

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
        if self.completed {
            self.completed_at = Some(Utc::now());
        } else {
            self.completed_at = None;
        }
    }

    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }

    pub fn has_description(&self) -> bool {
        self.description.as_ref().map_or(false, |desc| !desc.trim().is_empty())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TodoList {
    #[serde(default = "default_version")]
    pub version: u32,
    pub items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> Self {
        Self { 
            version: CURRENT_VERSION,
            items: Vec::new() 
        }
    }

    pub fn add_todo(&mut self, title: String, description: Option<String>) {
        let todo = TodoItem::new(title, description);
        self.items.push(todo);
    }

    pub fn remove_todo(&mut self, id: &Uuid) -> bool {
        if let Some(index) = self.items.iter().position(|item| &item.id == id) {
            self.items.remove(index);
            true
        } else {
            false
        }
    }

    pub fn toggle_todo(&mut self, id: &Uuid) -> bool {
        if let Some(item) = self.items.iter_mut().find(|item| &item.id == id) {
            item.toggle_completed();
            true
        } else {
            false
        }
    }

    pub fn toggle_expanded(&mut self, id: &Uuid) -> bool {
        if let Some(item) = self.items.iter_mut().find(|item| &item.id == id) {
            item.toggle_expanded();
            true
        } else {
            false
        }
    }

    pub fn set_expanded(&mut self, id: &Uuid, expanded: bool) -> bool {
        if let Some(item) = self.items.iter_mut().find(|item| &item.id == id) {
            item.expanded = expanded;
            true
        } else {
            false
        }
    }

    pub fn get_active_todos(&self) -> Vec<&TodoItem> {
        self.items.iter().filter(|item| !item.completed).collect()
    }

    pub fn get_completed_todos(&self) -> Vec<&TodoItem> {
        self.items.iter().filter(|item| item.completed).collect()
    }

    pub fn get_todo_by_id(&self, id: &Uuid) -> Option<&TodoItem> {
        self.items.iter().find(|item| &item.id == id)
    }

    pub fn get_todo_by_id_mut(&mut self, id: &Uuid) -> Option<&mut TodoItem> {
        self.items.iter_mut().find(|item| &item.id == id)
    }
}

pub fn format_datetime(dt: &DateTime<Utc>, format: &DateFormat) -> String {
    let local_dt = dt.with_timezone(&Local);
    
    match format {
        DateFormat::AmPm12Hour => local_dt.format("%b %d %-I:%M %p").to_string().to_lowercase(),
        DateFormat::AmPm12HourWithYear => local_dt.format("%b %d %Y %-I:%M %p").to_string().to_lowercase(),
        DateFormat::Hour24 => local_dt.format("%b %d %H:%M").to_string(),
        DateFormat::Hour24WithYear => local_dt.format("%b %d %Y %H:%M").to_string(),
        DateFormat::Iso8601 => local_dt.format("%Y-%m-%d %H:%M").to_string(),
        DateFormat::Relative => {
            let now = Local::now();
            let duration = now.signed_duration_since(local_dt);
            
            if duration.num_seconds() < 60 {
                "just now".to_string()
            } else if duration.num_minutes() < 60 {
                format!("{} minutes ago", duration.num_minutes())
            } else if duration.num_hours() < 24 {
                format!("{} hours ago", duration.num_hours())
            } else if duration.num_days() < 7 {
                format!("{} days ago", duration.num_days())
            } else {
                local_dt.format("%Y-%m-%d").to_string()
            }
        }
    }
}
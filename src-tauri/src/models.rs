use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub urgency: String,
    pub status: String,
    pub tags: Option<String>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub reminder_time: Option<String>,
    pub estimated_duration: Option<i32>,
    pub repeat_pattern: Option<String>,
    pub repeat_until: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInput {
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub urgency: String,
    pub tags: Option<Vec<String>>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub reminder_time: Option<String>,
    pub estimated_duration: Option<i32>,
    pub repeat_pattern: Option<String>,
    pub repeat_until: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskFilter {
    pub search: Option<String>,
    pub tags: Option<Vec<String>>,
    pub status: Option<Vec<String>>,
    pub priority: Option<Vec<String>>,
    pub urgency: Option<Vec<String>>,
    pub date_start: Option<String>,
    pub date_end: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Subtask {
    pub id: i64,
    pub task_id: i64,
    pub title: String,
    pub status: String,
    pub sort_order: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtaskInput {
    pub title: String,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TaskHistory {
    pub id: i64,
    pub task_id: i64,
    pub action: String,
    pub changes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Quadrant {
    First,   // 重要且紧急
    Second,  // 重要但不紧急
    Third,   // 不重要但紧急
    Fourth,  // 不重要且不紧急
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Csv,
    Json,
}

// Password models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Password {
    pub id: i64,
    pub category: String,
    pub subcategory: Option<String>,
    pub account: String,
    pub password: String,
    pub login_url: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordInput {
    pub category: String,
    pub subcategory: Option<String>,
    pub account: String,
    pub password: String,
    pub login_url: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PasswordFilter {
    pub search: Option<String>,
    pub category: Option<String>,
    pub subcategory: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

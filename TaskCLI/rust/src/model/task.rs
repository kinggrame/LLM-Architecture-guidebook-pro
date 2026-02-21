// src/model/task.rs - 任务模型
// 展示 Rust 的结构体、枚举、impl 方法

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 任务优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
            Priority::Urgent => write!(f, "urgent"),
        }
    }
}

impl std::str::FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            "urgent" => Ok(Priority::Urgent),
            _ => Err(format!("Unknown priority: {}", s)),
        }
    }
}

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Todo,
    InProgress,
    Done,
    Cancelled,
}

impl Default for Status {
    fn default() -> Self {
        Status::Todo
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Todo => write!(f, "todo"),
            Status::InProgress => write!(f, "in_progress"),
            Status::Done => write!(f, "done"),
            Status::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl std::str::FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "todo" => Ok(Status::Todo),
            "in_progress" | "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            "cancelled" | "cancel" => Ok(Status::Cancelled),
            _ => Err(format!("Unknown status: {}", s)),
        }
    }
}

/// 任务结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub status: Status,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: u32,
}

impl Task {
    /// 创建新任务（构造函数）
    pub fn new(id: u32, title: String, user_id: u32) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            description: None,
            priority: Priority::default(),
            status: Status::default(),
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            user_id,
        }
    }

    /// 设置描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置优先级
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    /// 设置标签
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// 更新任务状态
    pub fn update_status(&mut self, status: Status) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    /// 更新优先级
    pub fn update_priority(&mut self, priority: Priority) {
        self.priority = priority;
        self.updated_at = Utc::now();
    }

    /// 更新标题
    pub fn update_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now();
    }

    /// 检查是否匹配搜索关键词
    pub fn matches(&self, keyword: &str) -> bool {
        let keyword = keyword.to_lowercase();
        self.title.to_lowercase().contains(&keyword)
            || self
                .description
                .as_ref()
                .map(|d| d.to_lowercase().contains(&keyword))
                .unwrap_or(false)
            || self
                .tags
                .iter()
                .any(|t| t.to_lowercase().contains(&keyword))
    }

    /// 检查是否匹配状态
    pub fn has_status(&self, status: Status) -> bool {
        self.status == status
    }

    /// 检查是否包含标签
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new(1, "Test Task".to_string(), 1);
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.status, Status::Todo);
        assert_eq!(task.priority, Priority::Medium);
    }

    #[test]
    fn test_task_builder() {
        let task = Task::new(1, "Test".to_string(), 1)
            .with_description("Description".to_string())
            .with_priority(Priority::High)
            .with_tags(vec!["work".to_string()]);

        assert_eq!(task.description, Some("Description".to_string()));
        assert_eq!(task.priority, Priority::High);
        assert_eq!(task.tags, vec!["work"]);
    }

    #[test]
    fn test_task_matches() {
        let task = Task::new(1, "Buy milk".to_string(), 1)
            .with_description("Get from store")
            .with_tags(vec!["shopping".to_string()]);

        assert!(task.matches("milk"));
        assert!(task.matches("store"));
        assert!(task.matches("shopping"));
        assert!(!task.matches("xyz"));
    }
}

// src/model/user.rs - 用户模型

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 用户结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    /// 创建新用户
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            created_at: Utc::now(),
        }
    }

    /// 从字符串创建用户
    pub fn from_name(id: u32, name: &str) -> Self {
        Self::new(id, name.to_string())
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User({}, {})", self.id, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(1, "Alice".to_string());
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Alice");
    }
}

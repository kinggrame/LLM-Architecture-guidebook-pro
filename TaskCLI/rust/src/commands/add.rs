// src/commands/add.rs

use crate::model::{Priority, Status, Task};
use crate::storage::Storage;
use anyhow::Result;

/// 添加新任务
pub fn add_task(
    storage: &Storage,
    title: &str,
    description: Option<&str>,
    priority: Priority,
    tags: Option<&str>,
    user_name: &str,
) -> Result<()> {
    // 获取用户 ID
    let user_id = storage
        .get_user_id(user_name)
        .ok_or_else(|| anyhow::anyhow!("User not found: {}", user_name))?;

    // 解析标签
    let tags: Vec<String> = tags
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    // 创建任务
    let mut task = Task::new(0, title.to_string(), user_id);
    task.priority = priority;

    if let Some(desc) = description {
        task.description = Some(desc.to_string());
    }

    if !tags.is_empty() {
        task.tags = tags;
    }

    // 保存
    let task = storage.add_task(task)?;

    println!("✅ Task created: #{} - {}", task.id, task.title);

    Ok(())
}

// src/commands/list.rs

use crate::model::Status;
use crate::storage::Storage;
use anyhow::Result;

/// 列出任务
pub fn list_tasks(
    storage: &Storage,
    status: Option<Status>,
    tag: Option<&str>,
    user: Option<&str>,
) -> Result<()> {
    let mut tasks: Vec<_> = storage.get_tasks().iter().collect();
    
    // 按用户过滤
    if let Some(user_name) = user {
        let user_id = storage.get_user_id(user_name);
        if let Some(id) = user_id {
            tasks.retain(|t| t.user_id == id);
        }
    }
    
    // 按状态过滤
    if let Some(s) = status {
        tasks.retain(|t| t.status == s);
    }
    
    // 按标签过滤
    if let Some(t) = tag {
        tasks.retain(|task| task.has_tag(t));
    }
    
    // 按创建时间排序
    tasks.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    
    if tasks.is_empty() {
        println!("No tasks found");
        return Ok(());
    }
    
    println!("┌─────┬──────────────┬──────────┬─────────┬──────────┐");
    println!("│ ID  │ Title        │ Status   │ Priority│ Tags    │");
    println!("├─────┼──────────────┼──────────┼─────────┼──────────┤");
    
    for task in tasks {
        let tags = if task.tags.is_empty() {
            "-".to_string()
        } else {
            task.tags.join(",")
        };
        
        println!(
            │ {:4} │ {:12} │ {:8} │ {:7} │ {:8} │,
            task.id,
            truncate(&task.title, 12),
            task.status,
            task.priority,
            truncate(&tags, 8)
        );
    }
    
    println!("└─────┴──────────────┴──────────┴─────────┴──────────┘");
    println!("Total: {} tasks", tasks.len());
    
    Ok(())
}

/// 显示单个任务详情
pub fn show_task(storage: &Storage, id: u32) -> Result<()> {
    let task = storage.get_task(id)
        .ok_or_else(|| anyhow::anyhow!("Task not found: {}", id))?;
    
    println!("═══════════════════════════════════════");
    println!("Task #{}", task.id);
    println!("═══════════════════════════════════════");
    println!("Title:       {}", task.title);
    println!("Description: {}", task.description.as_deref().unwrap_or("-"));
    println!("Status:      {}", task.status);
    println!("Priority:    {}", task.priority);
    println!("Tags:        {}", task.tags.join(", "));
    println!("Created:     {}", task.created_at.format("%Y-%m-%d %H:%M"));
    println!("Updated:     {}", task.updated_at.format("%Y-%m-%d %H:%M"));
    println!("═══════════════════════════════════════");
    
    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}..", &s[..max_len - 2])
    } else {
        s.to_string()
    }
}

// src/commands/stats.rs

use crate::storage::{Storage, TaskStats};
use anyhow::Result;

/// 显示统计信息
pub fn show_stats(storage: &Storage, user: Option<&str>) -> Result<()> {
    let user_id = user.and_then(|u| storage.get_user_id(u));

    let stats = storage.get_stats(user_id);

    println!("═══════════════════════════════════════");
    println!("📊 Task Statistics");
    println!("═══════════════════════════════════════");
    println!("Total:      {}", stats.total);
    println!("  📋 Todo:       {}", stats.todo);
    println!("  🔄 In Progress: {}", stats.in_progress);
    println!("  ✅ Done:        {}", stats.done);
    println!("  ❌ Cancelled:   {}", stats.cancelled);
    println!("");
    println!("By Priority:");
    for (priority, count) in &stats.by_priority {
        println!("  {}: {}", priority, count);
    }
    println!("═══════════════════════════════════════");

    Ok(())
}

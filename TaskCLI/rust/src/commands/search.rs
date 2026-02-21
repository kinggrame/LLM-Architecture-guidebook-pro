// src/commands/search.rs

use crate::storage::Storage;
use anyhow::Result;

/// 搜索任务
pub fn search_tasks(storage: &Storage, keyword: &str) -> Result<()> {
    let results = storage.search_tasks(keyword);

    if results.is_empty() {
        println!("No tasks found matching '{}'", keyword);
        return Ok(());
    }

    println!("Found {} tasks:", results.len());
    for task in results {
        println!("  #{} - {} [{}]", task.id, task.title, task.status);
    }

    Ok(())
}

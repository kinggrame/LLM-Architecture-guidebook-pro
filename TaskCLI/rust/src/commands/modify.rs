// src/commands/modify.rs

use crate::model::Priority;
use crate::model::Status;
use crate::storage::Storage;
use anyhow::Result;

/// 修改任务
pub fn modify_task(
    storage: &Storage,
    id: u32,
    status: Option<Status>,
    priority: Option<Priority>,
    title: Option<&str>,
) -> Result<()> {
    let task = storage
        .get_task(id)
        .ok_or_else(|| anyhow::anyhow!("Task not found: {}", id))?
        .clone();

    let mut task = task;

    if let Some(s) = status {
        task.update_status(s);
    }

    if let Some(p) = priority {
        task.update_priority(p);
    }

    if let Some(t) = title {
        task.update_title(t.to_string());
    }

    storage.update_task(task)?;
    println!("✅ Task #{} updated", id);

    Ok(())
}

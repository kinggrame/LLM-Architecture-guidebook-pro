// src/commands/delete.rs

use crate::storage::Storage;
use anyhow::Result;

/// 删除任务
pub fn delete_task(storage: &Storage, id: u32) -> Result<()> {
    // 检查任务是否存在
    if storage.get_task(id).is_none() {
        println!("❌ Task #{} not found", id);
        return Ok(());
    }

    storage.delete_task(id)?;
    println!("✅ Task #{} deleted", id);

    Ok(())
}

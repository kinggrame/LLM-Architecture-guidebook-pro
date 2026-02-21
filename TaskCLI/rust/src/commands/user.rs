// src/commands/user.rs

use crate::storage::Storage;
use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum UserAction {
    /// 添加用户
    Add {
        /// 用户名
        name: String,
    },
    /// 列出用户
    List,
    /// 删除用户
    Delete {
        /// 用户 ID
        id: u32,
    },
}

/// 添加用户
pub fn add_user(storage: &Storage, name: &str) -> Result<()> {
    let user = storage.add_user(name.to_string())?;
    println!("✅ User created: {} (ID: {})", user.name, user.id);
    Ok(())
}

/// 列出用户
pub fn list_users(storage: &Storage) -> Result<()> {
    let users = storage.get_users();

    if users.is_empty() {
        println!("No users found");
        return Ok(());
    }

    println!("Users:");
    for user in users {
        println!("  {} - {}", user.id, user.name);
    }

    Ok(())
}

/// 删除用户
pub fn delete_user(storage: &Storage, id: u32) -> Result<()> {
    storage.delete_user(id)?;
    println!("✅ User #{} deleted", id);
    Ok(())
}

// src/main.rs - 入口文件
// TaskCLI - 命令行任务管理工具

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod error;
mod model;
mod storage;

use commands::{add, delete, list, modify, search, stats, user as user_commands};
use error::AppError;
use model::{Priority, Status, Task, User};
use storage::Storage;

#[derive(Parser)]
#[command(name = "taskcli")]
#[command(about = "A command-line task management tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// 数据存储路径
    #[arg(short, long, default_value = "./data")]
    data_dir: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// 用户管理
    User {
        #[command(subcommand)]
        action: UserAction,
    },
    /// 添加新任务
    Add {
        /// 任务标题
        title: String,
        /// 任务描述
        #[arg(short, long)]
        description: Option<String>,
        /// 优先级 (low/medium/high/urgent)
        #[arg(short, long, default_value = "medium")]
        priority: Priority,
        /// 标签 (逗号分隔)
        #[arg(short, long)]
        tags: Option<String>,
        /// 指定用户
        #[arg(short, long, default_value = "1")]
        user: String,
    },
    /// 列出任务
    List {
        /// 按状态过滤
        #[arg(short, long)]
        status: Option<Status>,
        /// 按标签过滤
        #[arg(short, long)]
        tag: Option<String>,
        /// 按用户过滤
        #[arg(short, long)]
        user: Option<String>,
    },
    /// 显示任务详情
    Show {
        /// 任务 ID
        id: u32,
    },
    /// 修改任务
    Modify {
        /// 任务 ID
        id: u32,
        /// 新状态
        #[arg(short, long)]
        status: Option<Status>,
        /// 新优先级
        #[arg(short, long)]
        priority: Option<Priority>,
        /// 新标题
        #[arg(short, long)]
        title: Option<String>,
    },
    /// 删除任务
    Delete {
        /// 任务 ID
        id: u32,
    },
    /// 搜索任务
    Search {
        /// 搜索关键词
        keyword: String,
    },
    /// 统计信息
    Stats {
        /// 用户过滤
        #[arg(short, long)]
        user: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // 初始化存储
    let storage = Storage::new(cli.data_dir)?;
    
    // 执行命令
    match cli.command {
        Commands::User { action } => match action {
            user_commands::UserAction::Add { name } => {
                user_commands::add_user(&storage, &name)?;
            }
            user_commands::UserAction::List {} => {
                user_commands::list_users(&storage)?;
            }
            user_commands::UserAction::Delete { id } => {
                user_commands::delete_user(&storage, id)?;
            }
        },
        
        Commands::Add { title, description, priority, tags, user } => {
            add::add_task(&storage, &title, description.as_deref(), priority, tags.as_deref(), &user)?;
        }
        
        Commands::List { status, tag, user } => {
            list::list_tasks(&storage, status, tag.as_deref(), user.as_deref())?;
        }
        
        Commands::Show { id } => {
            list::show_task(&storage, id)?;
        }
        
        Commands::Modify { id, status, priority, title } => {
            modify::modify_task(&storage, id, status, priority, title.as_deref())?;
        }
        
        Commands::Delete { id } => {
            delete::delete_task(&storage, id)?;
        }
        
        Commands::Search { keyword } => {
            search::search_tasks(&storage, &keyword)?;
        }
        
        Commands::Stats { user } => {
            stats::show_stats(&storage, user.as_deref())?;
        }
    }
    
    Ok(())
}

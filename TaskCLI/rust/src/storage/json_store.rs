// src/storage/json_store.rs - JSON 文件存储
// 展示 Rust 的文件 I/O、错误处理、Result

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::model::{Task, User};

/// 应用数据容器
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppData {
    pub users: Vec<User>,
    pub tasks: Vec<Task>,
    pub next_user_id: u32,
    pub next_task_id: u32,
}

impl AppData {
    /// 创建新的应用数据
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            tasks: Vec::new(),
            next_user_id: 1,
            next_task_id: 1,
        }
    }

    /// 添加默认用户
    pub fn init_default_user(&mut self) {
        if self.users.is_empty() {
            self.users.push(User::new(1, "default".to_string()));
        }
    }
}

/// 存储管理器
pub struct Storage {
    data_dir: PathBuf,
    data: AppData,
}

impl Storage {
    /// 创建新的存储管理器
    pub fn new(data_dir: PathBuf) -> Result<Self> {
        // 确保目录存在
        fs::create_dir_all(&data_dir).context("Failed to create data directory")?;

        // 加载数据
        let data = Self::load_from_file(&data_dir)?;

        Ok(Self { data_dir, data })
    }

    /// 从文件加载数据
    fn load_from_file(data_dir: &PathBuf) -> Result<AppData> {
        let data_file = data_dir.join("data.json");

        if data_file.exists() {
            let content = fs::read_to_string(&data_file).context("Failed to read data file")?;

            let mut data: AppData =
                serde_json::from_str(&content).context("Failed to parse data file")?;

            // 初始化默认用户
            data.init_default_user();

            Ok(data)
        } else {
            // 创建默认数据
            let mut data = AppData::new();
            data.init_default_user();

            // 保存到文件
            let content = serde_json::to_string_pretty(&data)?;
            fs::write(&data_file, content)?;

            Ok(data)
        }
    }

    /// 保存数据到文件
    pub fn save(&self) -> Result<()> {
        let data_file = self.data_dir.join("data.json");
        let content = serde_json::to_string_pretty(&self.data)?;
        fs::write(&data_file, content)?;
        Ok(())
    }

    // ==================== 用户操作 ====================

    /// 获取所有用户
    pub fn get_users(&self) -> &[User] {
        &self.data.users
    }

    /// 添加用户
    pub fn add_user(&mut self, name: String) -> Result<User> {
        let user = User::new(self.data.next_user_id, name);
        self.data.next_user_id += 1;
        self.data.users.push(user.clone());
        self.save()?;
        Ok(user)
    }

    /// 删除用户
    pub fn delete_user(&mut self, id: u32) -> Result<()> {
        self.data.users.retain(|u| u.id != id);
        // 同时删除用户的任务
        self.data.tasks.retain(|t| t.user_id != id);
        self.save()?;
        Ok(())
    }

    /// 获取用户 ID（按名称）
    pub fn get_user_id(&self, name: &str) -> Option<u32> {
        self.data
            .users
            .iter()
            .find(|u| u.name == name)
            .map(|u| u.id)
    }

    // ==================== 任务操作 ====================

    /// 获取所有任务
    pub fn get_tasks(&self) -> &[Task] {
        &self.data.tasks
    }

    /// 获取任务（按 ID）
    pub fn get_task(&self, id: u32) -> Option<&Task> {
        self.data.tasks.iter().find(|t| t.id == id)
    }

    /// 添加任务
    pub fn add_task(&mut self, task: Task) -> Result<Task> {
        let id = self.data.next_task_id;
        let mut task = task;
        task.id = id;
        self.data.next_task_id += 1;
        self.data.tasks.push(task.clone());
        self.save()?;
        Ok(task)
    }

    /// 更新任务
    pub fn update_task(&mut self, task: Task) -> Result<()> {
        if let Some(existing) = self.data.tasks.iter_mut().find(|t| t.id == task.id) {
            *existing = task;
            self.save()?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Task not found"))
        }
    }

    /// 删除任务
    pub fn delete_task(&mut self, id: u32) -> Result<()> {
        self.data.tasks.retain(|t| t.id != id);
        self.save()?;
        Ok(())
    }

    /// 按用户过滤任务
    pub fn get_tasks_by_user(&self, user_id: u32) -> Vec<&Task> {
        self.data
            .tasks
            .iter()
            .filter(|t| t.user_id == user_id)
            .collect()
    }

    /// 按状态过滤任务
    pub fn get_tasks_by_status(&self, status: crate::model::Status) -> Vec<&Task> {
        self.data
            .tasks
            .iter()
            .filter(|t| t.status == status)
            .collect()
    }

    /// 按标签过滤任务
    pub fn get_tasks_by_tag(&self, tag: &str) -> Vec<&Task> {
        self.data.tasks.iter().filter(|t| t.has_tag(tag)).collect()
    }

    /// 搜索任务
    pub fn search_tasks(&self, keyword: &str) -> Vec<&Task> {
        self.data
            .tasks
            .iter()
            .filter(|t| t.matches(keyword))
            .collect()
    }

    /// 获取任务统计
    pub fn get_stats(&self, user_id: Option<u32>) -> TaskStats {
        let tasks: Vec<&Task> = match user_id {
            Some(id) => self.data.tasks.iter().filter(|t| t.user_id == id).collect(),
            None => self.data.tasks.iter().collect(),
        };

        TaskStats {
            total: tasks.len(),
            todo: tasks
                .iter()
                .filter(|t| t.status == crate::model::Status::Todo)
                .count(),
            in_progress: tasks
                .iter()
                .filter(|t| t.status == crate::model::Status::InProgress)
                .count(),
            done: tasks
                .iter()
                .filter(|t| t.status == crate::model::Status::Done)
                .count(),
            cancelled: tasks
                .iter()
                .filter(|t| t.status == crate::model::Status::Cancelled)
                .count(),
            by_priority: self::count_by_priority(&tasks),
        }
    }
}

fn count_by_priority(tasks: &[&Task]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for task in tasks {
        *counts.entry(task.priority.to_string()).or_insert(0) += 1;
    }
    counts
}

/// 任务统计
#[derive(Debug)]
pub struct TaskStats {
    pub total: usize,
    pub todo: usize,
    pub in_progress: usize,
    pub done: usize,
    pub cancelled: usize,
    pub by_priority: HashMap<String, usize>,
}

# TaskCLI 项目学习指南

> 🎯 通过实际项目代码学习 Rust 和 C++

---

## 项目结构

```
TaskCLI/
├── rust/                    # Rust 版本
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs          # 入口
│       ├── model/           # 数据模型
│       │   ├── task.rs      # 任务
│       │   └── user.rs      # 用户
│       ├── storage/         # 存储层
│       │   └── json_store.rs
│       └── commands/        # 命令实现
│
└── cpp/                     # C++ 版本
    ├── CMakeLists.txt
    ├── include/
    │   ├── model/
    │   │   ├── task.h
    │   │   └── user.h
    │   └── storage/
    │       └── json_store.h
    └── src/
        ├── main.cpp
        ├── model/
        │   ├── task.cpp
        │   └── user.cpp
        └── storage/
            └── json_store.cpp
```

---

# 第一课：结构体定义

## Rust 版本

```rust
// rust/src/model/task.rs

/// 任务优先级枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

/// 任务结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub status: Status,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: u32,
}
```

**学习要点：**
- `pub` - 公开字段
- `Option<String>` - 可选值，类似 nullable
- `Vec<String>` - 动态数组
- `derive(...)` - 自动实现 trait

---

## C++ 版本

```cpp
// cpp/include/model/task.h

enum class Priority {
    Low,
    Medium,
    High,
    Urgent
};

enum class Status {
    Todo,
    InProgress,
    Done,
    Cancelled
};

class Task {
public:
    uint32 std::string title_t id;
   ;
    std::optional<std::string> description;
    Priority priority;
    Status status;
    std::vector<std::string> tags;
    std::chrono::system_clock::time_point created_at;
    std::chrono::system_clock::time_point updated_at;
    uint32_t user_id;
    
    Task(uint32_t id, const std::string& title, uint32_t user_id);
    // ... 方法声明
};
```

**学习要点：**
- `class` vs `struct`（默认访问权限不同）
- `std::string` - 字符串类型
- `std::optional` - 可选值（C++17）
- `std::vector` - 动态数组

---

## 对比总结

| Rust | C++ | 说明 |
|------|-----|------|
| `struct Task { }` | `class Task { public: ... };` | 结构体定义 |
| `u32` | `uint32_t` | 无符号整数 |
| `String` | `std::string` | 可变字符串 |
| `&str` | `const std::string&` | 字符串引用 |
| `Option<T>` | `std::optional<T>` | 可选值 |
| `Vec<T>` | `std::vector<T>` | 动态数组 |
| `enum` | `enum class` | 枚举（更安全） |

---

# 第二课：构造函数和方法

## Rust 版本（impl 块）

```rust
// rust/src/model/task.rs

impl Task {
    /// 构造函数
    pub fn new(id: u32, title: String, user_id: u32) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            description: None,
            priority: Priority::default(),
            status: Status::default(),
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            user_id,
        }
    }
    
    /// 设置描述（建造者模式）
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    
    /// 更新状态
    pub fn update_status(&mut self, status: Status) {
        self.status = status;
        self.updated_at = Utc::now();
    }
    
    /// 检查匹配
    pub fn matches(&self, keyword: &str) -> bool {
        let keyword = keyword.to_lowercase();
        self.title.to_lowercase().contains(&keyword)
            || self.description.as_ref()
                .map(|d| d.to_lowercase().contains(&keyword))
                .unwrap_or(false)
    }
}
```

**学习要点：**
- `impl Type` - 为类型添加方法
- `Self` - 返回当前类型
- `&self` - 不可变借用
- `&mut self` - 可变借用

---

## C++ 版本

```cpp
// cpp/src/model/task.cpp

Task::Task(uint32_t id, const std::string& title, uint32_t user_id)
    : id(id), title(title), user_id(user_id) {
    priority = Priority::Medium;
    status = Status::Todo;
    created_at = system_clock::now();
    updated_at = system_clock::now();
}

void Task::set_description(const std::string& desc) {
    description = desc;
    updated_at = system_clock::now();
}

bool Task::matches(const std::string& keyword) const {
    std::string lower_keyword = keyword;
    std::transform(lower_keyword.begin(), lower_keyword.end(), 
                   lower_keyword.begin(), ::tolower);
    
    std::string lower_title = title;
    std::transform(lower_title.begin(), lower_title.end(),
                   lower_title.begin(), ::tolower);
    
    return lower_title.find(lower_keyword) != std::string::npos;
}
```

**学习要点：**
- 构造函数初始化列表 `: field(value)`
- `const` 方法 - 不修改成员
- `std::string::transform` - 字符串转换

---

# 第三课：智能指针和所有权

## Rust 版本

```rust
// rust/src/storage/json_store.rs

pub struct Storage {
    data_dir: PathBuf,
    data: AppData,
}

impl Storage {
    pub fn new(data_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&data_dir)?;
        
        let data = Self::load_from_file(&data_dir)?;
        
        Ok(Self { data_dir, data })
    }
    
    pub fn add_task(&mut self, task: Task) -> Result<Task> {
        let id = self.data.next_task_id;
        let mut task = task;
        task.id = id;
        
        self.data.next_task_id += 1;
        self.data.tasks.push(task.clone());  // 克隆
        
        self.save()?;
        Ok(task)
    }
}
```

**学习要点：**
- `Result<T>` - 错误处理
- `?` 操作符 - 错误传播
- `.clone()` - 显式克隆

---

## C++ 版本（智能指针）

```cpp
// cpp/include/storage/json_store.h

class Storage {
public:
    Storage(const std::string& data_dir);
    
    std::shared_ptr<Task> add_task(std::shared_ptr<Task> task);
    std::vector<std::shared_ptr<Task>> get_tasks();
    
private:
    void save();
    void load();
    
    std::vector<std::shared_ptr<User>> users_;
    std::vector<std::shared_ptr<Task>> tasks_;
};
```

```cpp
// cpp/src/storage/json_store.cpp

std::shared_ptr<Task> Storage::add_task(std::shared_ptr<Task> task) {
    task->id = next_task_id_++;
    tasks_.push_back(task);
    save();
    return task;
}
```

**学习要点：**
- `std::shared_ptr<T>` - 共享所有权
- `std::make_shared<T>()` - 创建智能指针
- 引用计数自动管理内存

---

# 第四课：泛型和容器

## Rust 版本

```rust
// 使用 HashMap 统计
use std::collections::HashMap;

pub fn get_stats(&self, user_id: Option<u32>) -> TaskStats {
    let tasks: Vec<&Task> = match user_id {
        Some(id) => self.data.tasks.iter().filter(|t| t.user_id == id).collect(),
        None => self.data.tasks.iter().collect(),
    };
    
    TaskStats {
        total: tasks.len(),
        todo: tasks.iter().filter(|t| t.status == Status::Todo).count(),
        // ...
    }
}
```

---

## C++ 版本

```cpp
// 使用 unordered_map
#include <unordered_map>

std::unordered_map<std::string, size_t> count_by_priority;
for (const auto& t : tasks) {
    count_by_priority[t->priority_to_string()]++;
}
```

---

# 第五课：错误处理

## Rust 版本

```rust
// Result 和 ? 操作符
pub fn new(data_dir: PathBuf) -> Result<Self> {
    fs::create_dir_all(&data_dir)
        .context("Failed to create data directory")?;
    
    let data = Self::load_from_file(&data_dir)?;
    Ok(Self { data_dir, data })
}

pub fn add_user(&mut self, name: String) -> Result<User> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("Name cannot be empty"));
    }
    // ...
}
```

---

## C++ 版本（异常）

```cpp
class StorageException : public std::exception {
public:
    StorageException(const std::string& msg) : msg_(msg) {}
    const char* what() const noexcept override { return msg_.c_str(); }
private:
    std::string msg_;
};

void Storage::save() {
    if (!file.is_open()) {
        throw StorageException("Failed to open file");
    }
}
```

---

# 第六课：迭代器和闭包

## Rust 版本

```rust
// 搜索任务
pub fn search_tasks(&self, keyword: &str) -> Vec<&Task> {
    self.data.tasks.iter()
        .filter(|t| t.matches(keyword))
        .collect()
}

// 过滤和映射
let high_priority: Vec<_> = tasks.iter()
    .filter(|t| t.priority == Priority::High)
    .map(|t| t.title.clone())
    .collect();
```

---

## C++ 版本

```cpp
// 使用算法
#include <algorithm>
#include <iterator>

std::vector<std::shared_ptr<Task>> Storage::search_tasks(
    const std::string& keyword) {
    
    std::vector<std::shared_ptr<Task>> result;
    std::copy_if(tasks_.begin(), tasks_.end(),
                 std::back_inserter(result),
                 [&keyword](const auto& t) {
                     return t->matches(keyword);
                 });
    return result;
}
```

---

# 第七课：CLI 参数解析

## Rust 版本（clap）

```rust
// rust/src/main.rs

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "taskcli")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long, default_value = "./data")]
    data_dir: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        title: String,
        #[arg(short, long)]
        description: Option<String>,
    },
    List {
        #[arg(short, long)]
        status: Option<Status>,
    },
    // ...
}
```

---

## C++ 版本（手动解析）

```cpp
// cpp/src/main.cpp

int main(int argc, char* argv[]) {
    if (argc < 2) {
        print_usage();
        return 1;
    }
    
    std::string cmd = argv[1];
    
    if (cmd == "add") {
        if (argc < 3) {
            std::cerr << "Error: missing title\n";
            return 1;
        }
        std::string title = argv[2];
        // ...
    } else if (cmd == "list") {
        // ...
    }
}
```

---

# 练习建议

1. **运行项目**：在 `TaskCLI/rust` 目录运行 `cargo run -- add "Buy milk"`
2. **添加功能**：为 Task 添加截止日期
3. **对比学习**：尝试用 C++ 实现相同功能
4. **扩展项目**：添加任务提醒功能

---

*持续更新中...*

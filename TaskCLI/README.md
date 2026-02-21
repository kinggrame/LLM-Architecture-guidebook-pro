# TaskCLI 项目 - 30天实战学习计划

> 🎯 **目标**：通过构建一个完整的命令行任务管理系统，系统掌握 Rust 和 C++

---

## 项目概述

**TaskCLI** - 一个支持多用户的命令行任务管理工具

### 核心功能
- 添加、删除、修改任务
- 任务分类（标签）
- 任务优先级
- 任务状态管理（待办/进行中/完成）
- 数据持久化（JSON 文件存储）
- 多用户支持

---

## 学习路径（30天）

### 第一阶段：项目基础（Day 1-10）

| 天数 | 主题 | 项目任务 | 语言特性 |
|:----:|------|----------|----------|
| Day 1 | 项目初始化 | 创建项目结构、命令行参数解析 | 基础 I/O、函数 |
| Day 2 | 任务模型 | 定义任务结构体 | Struct、impl、枚举 |
| Day 3 | 数据存储 | JSON 文件读写 | 序列化、错误处理 |
| Day 4 | 命令实现 | 添加/删除任务 | 所有权、借用 |
| Day 5 | 任务管理 | 修改任务状态 | Option、Result |
| Day 6 | 标签系统 | 任务分类 | HashMap、泛型 |
| Day 7 | 搜索功能 | 按标签/状态搜索 | 迭代器、闭包 |
| Day 8 | 用户系统 | 多用户支持 | 生命周期、引用 |
| Day 9 | 配置文件 | 用户配置管理 | 文件 I/O、RAII |
| Day 10 | 单元测试 | 测试驱动开发 | 测试框架 |

### 第二阶段：C++ 实现（Day 11-20）

| 天数 | 主题 | 项目任务 | C++ 特性 |
|:----:|------|----------|----------|
| Day 11 | 项目迁移 | Rust → C++ 核心结构 | 类、指针 |
| Day 12 | 内存管理 | 智能指针实现 | unique_ptr、shared_ptr |
| Day 13 | 模板泛型 | 重构任务存储 | 模板、容器 |
| Day 14 | STL 应用 | 使用标准库容器 | vector、map |
| Day 15 | 错误处理 | 异常机制 | try-catch |
| Day 16 | 继承多态 | 任务类型扩展 | 虚函数 |
| Day 17 | 线程安全 | 并发任务处理 | thread、mutex |
| Day 18 | 文件流 | JSON 序列化 | fstream |
| Day 19 | 性能优化 | 内存布局优化 | move语义 |
| Day 20 | 构建系统 | CMake 配置 | CMake |

### 第三阶段：高级特性（Day 21-30）

| 天数 | 主题 | 项目任务 | 高级特性 |
|:----:|------|----------|----------|
| Day 21 | 异步 | 后台数据同步 | async/await |
| Day 22 | 网络 | REST API 服务器 | HTTP 服务器 |
| Day 23 | 数据库 | SQLite 集成 | 数据库操作 |
| Day 24 | CLI 增强 | 交互式界面 | 终端 UI |
| Day 25 | 插件系统 | 命令扩展 | 动态加载 |
| Day 26 | 国际化 | 多语言支持 | UTF-8 |
| Day 27 | 单元测试 | 完整测试覆盖 | Mock |
| Day 28 | 性能分析 | 性能调优 | Profiling |
| Day 29 | 文档 | 自动生成文档 | Documentation |
| Day 30 | 发布 | 打包发布 | Release |

---

## 项目结构

```
TaskCLI/
├── rust/                      # Rust 版本
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs           # 入口
│   │   ├── cli.rs            # CLI 参数解析
│   │   ├── model/
│   │   │   ├── mod.rs
│   │   │   ├── task.rs       # 任务模型
│   │   │   └── user.rs       # 用户模型
│   │   ├── storage/
│   │   │   ├── mod.rs
│   │   │   └── json_store.rs  # JSON 存储
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── add.rs        # 添加任务
│   │   │   ├── list.rs       # 列表任务
│   │   │   └── modify.rs     # 修改任务
│   │   └── error.rs          # 错误类型
│   └── tests/
│
├── cpp/                       # C++ 版本
│   ├── CMakeLists.txt
│   ├── include/
│   │   ├── model/
│   │   │   ├── task.h
│   │   │   └── user.h
│   │   ├── storage/
│   │   │   └── json_store.h
│   │   └── cli.h
│   ├── src/
│   │   ├── main.cpp
│   │   ├── cli.cpp
│   │   ├── model/
│   │   │   ├── task.cpp
│   │   │   └── user.cpp
│   │   └── storage/
│   │       └── json_store.cpp
│   └── tests/
│
└── docs/                     # 学习文档
    └── snippets/             # 代码片段分析
```

---

## 数据模型

### 任务 (Task)

```rust
struct Task {
    id: u32,
    title: String,
    description: Option<String>,
    priority: Priority,
    status: Status,
    tags: Vec<String>,
    created_at: DateTime,
    updated_at: DateTime,
    user_id: u32,
}

enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

enum Status {
    Todo,
    InProgress,
    Done,
    Cancelled,
}
```

### 用户 (User)

```rust
struct User {
    id: u32,
    name: String,
    created_at: DateTime,
}
```

---

## 命令行接口

```bash
# 用户管理
taskcli user add <name>
taskcli user list
taskcli user delete <id>

# 任务管理
taskcli add <title> [-d <desc>] [-p high] [-t tag1,tag2]
taskcli list [-s status] [-t tag] [-u user]
taskcli show <id>
taskcli modify <id> [-s status] [-p priority]
taskcli delete <id>

# 搜索
taskcli search <keyword>

# 统计
taskcli stats
taskcli stats -u <user>
```

---

## 开始学习

请从 **Day 1** 开始，我们首先创建 Rust 项目的基础结构。

👉 [Day 1: 项目初始化与命令行参数解析](./Day1_项目初始化/README.md)

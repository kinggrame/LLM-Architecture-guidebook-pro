# 第6天：模块系统（Mod）与代码组织

> 🗂️ **目标**：学会如何组织代码，把相关功能分组，让项目结构清晰。

---

## 6.1 什么是模块？

**模块**是把相关代码组织在一起的方式。就像文件夹管理文件一样，模块管理代码。

**比喻**：
```
项目/
├── main.rs          # 主程序
├── core/            # 核心模块
│   ├── mod.rs       # 模块入口
│   ├── engine.rs    # 引擎
│   └── config.rs   # 配置
└── scene/           # 场景模块
    ├── mod.rs
    └── scene.rs
```

---

## 6.2 声明模块（mod）

### 6.2.1 从文件加载模块

假设有目录结构：
```
src/
├── main.rs
├── lib.rs
├── core/
│   ├── mod.rs
│   ├── engine.rs
│   └── config.rs
```

**main.rs**:
```rust
// 使用 mod 声明模块
mod core;     // 引入 core 目录
mod scene;    // 引入 scene 目录
mod script;
mod resources;
mod rendering;

fn main() {
    // 使用模块中的内容
    let engine = core::engine::Engine::new();
}
```

**core/mod.rs**:
```rust
// 导出子模块
pub mod engine;
pub mod config;
```

### 6.2.2 模块的入口：mod.rs

每个子目录需要一个 `mod.rs`（或在 Rust 2018+ 风格中可以直接用 `目录名.rs`）来声明模块内容。

```rust
// rust/src/core/mod.rs

pub mod engine;
pub mod config;
```

---

## 6.3 可见性：pub 关键字

默认情况下，模块内的内容是**私有的**。使用 `pub` 让内容**公开**。

### 6.3.1 pub 的不同级别

```rust
mod game {
    // 默认私有
    struct Secret {
        name: String,
    }
    
    // 公开结构体
    pub struct Player {
        pub name: String,    // 公开字段
        pub age: i32,
        health: f32,         // 仍然私有
    }
    
    // 公开函数
    pub fn create_player(name: &str) -> Player {
        Player {
            name: String::from(name),
            age: 20,
            health: 100.0,
        }
    }
    
    // 私有函数（仅在模块内使用）
    fn calculate_health() -> f32 {
        100.0
    }
}

fn main() {
    // 可以创建 Player
    let player = game::create_player("小林");
    println!("{}", player.name);  // 可以访问
    
    // println!("{}", player.health);  // ❌ 错误！health 是私有的
}
```

### 6.3.2 pub use - 重新导出

```rust
mod modules {
    pub mod inner {
        pub fn hello() {
            println!("Hello!");
        }
    }
}

// 把 inner 导出到顶层
use modules::inner;

fn main() {
    hello();  // 直接使用，不用 modules::inner::hello()
}
```

---

## 6.4 use 语句 - 引入模块

### 6.4.1 基本用法

```rust
mod audio {
    pub fn play() {
        println!("播放音乐");
    }
}

fn main() {
    // 方式1：完整路径
    audio::play();
    
    // 方式2：使用 use 引入
    use audio::play;
    play();
}
```

### 6.4.2 引入时重命名

```rust
use audio::play as play_music;

fn main() {
    play_music();
}
```

### 6.4.3 引入多个项目

```rust
use std::io::{Read, Write};
use std::collections::{HashMap, VecDeque};
```

---

## 6.5 项目中的模块组织

### 6.5.1 主入口：main.rs

```rust
// rust/src/main.rs

// 声明模块
mod core;
mod rendering;
mod resources;
mod scene;
mod script;

// 引入需要的类型
use crate::core::config::GameConfig;
use crate::core::engine::Engine;
use crate::resources::ResourceManager;
use crate::scene::scene::VisualNovelScene;
use crate::scene::SceneManager;
use crate::script::command::CommandType;
use crate::script::ScriptEngine;

fn main() -> Result<()> {
    // 创建引擎
    let mut engine = Engine::new();
    
    // 配置
    let config = GameConfig {
        title: "Visual Novel Engine".to_string(),
        width: 1280,
        height: 720,
        vsync: true,
        ..Default::default()
    };
    
    // 初始化
    engine.initialize(config)?;
    
    println!("引擎启动成功！");
    Ok(())
}
```

### 6.5.2 core/mod.rs

```rust
// rust/src/core/mod.rs

pub mod engine;
pub mod config;
```

### 6.5.3 层级引用

在子模块中引用父模块或其他模块：

```rust
// 假设在 rust/src/script/mod.rs 中

// 引用同级的 command 模块
pub mod command;

// 从顶级引用
use crate::resources::ResourceManager;
```

---

## 6.6 Cargo 项目管理

### 6.6.1 Cargo.toml 详解

```toml
[package]
name = "visual-novel-engine"   # 项目名
version = "0.1.0"             # 版本号
edition = "2021"              # Rust 版本

[dependencies]                 # 依赖库
wgpu = "0.17"
rodio = "0.17"
image = "0.24"

[dev-dependencies]            # 开发依赖（测试用）
```

### 6.6.2 常用 Cargo 命令

```bash
# 创建新项目
cargo new my_project

# 构建项目
cargo build

# 运行项目
cargo run

# 检查代码（不生成可执行文件）
cargo check

# 发布构建
cargo build --release

# 添加依赖
cargo add wgpu
```

---

## 6.7 今日总结

今天我们学习了：
- ✅ **模块（mod）** - 组织代码的方式
- ✅ **pub 关键字** - 控制可见性
- ✅ **use 语句** - 引入模块
- ✅ **Cargo** - Rust 的包管理器
- ✅ 项目中的模块组织方式

---

## 6.8 练习题

1. 查看项目的 `rust/src/main.rs`，找出它声明了哪些模块
2. 在 `core/mod.rs` 中添加一个新模块 `logger`
3. 解释为什么有些字段用 `pub`，有些不用

---

## 6.9 明日预告

明天我们将学习：
- **脚本系统** - 如何让游戏剧情用文本编写
- **脚本解析器原理** - 文本如何变成游戏指令
- **项目中的 ScriptEngine 实现**

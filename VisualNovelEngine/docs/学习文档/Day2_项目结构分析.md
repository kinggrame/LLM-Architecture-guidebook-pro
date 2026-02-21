# 第2天：项目结构分析

## 2.1 查看项目整体结构

让我们先看看 Visual Novel Engine 项目的目录结构：

```
VisualNovelEngine/
├── cpp/                    # C++ 版本（底层，性能好）
│   ├── CMakeLists.txt      # 构建配置文件
│   ├── include/            # 头文件（.h）
│   │   ├── core/           # 核心引擎
│   │   ├── rendering/      # 渲染系统
│   │   ├── resources/      # 资源管理
│   │   ├── scene/          # 场景管理
│   │   └── script/         # 脚本系统
│   └── src/                # 源文件（.cpp）
│       ├── main.cpp
│       ├── core/
│       ├── rendering/
│       ├── resources/
│       ├── scene/
│       └── script/
│
├── rust/                   # Rust 版本（安全，易用）
│   ├── Cargo.toml          # 项目配置文件
│   └── src/
│       ├── main.rs         # 程序入口
│       ├── core/           # 核心引擎
│       ├── rendering/      # 渲染系统
│       ├── resources/      # 资源管理
│       ├── scene/          # 场景管理
│       └── script/         # 脚本系统
│
├── assets/                 # 游戏资源
│   └── scripts/
│       └── demo_script.txt # 示例剧情脚本
│
└── docs/                   # 文档
```

---

## 2.2 为什么有两种语言版本？

### C++ 版本
**优点：**
- 性能极佳（运行速度快）
- 游戏行业广泛使用
- 可以直接操作硬件

**缺点：**
- 语法复杂
- 内存管理需要小心（容易出 bug）
- 编译时间长

**适用场景：** 对性能要求极高的大型游戏

### Rust 版本
**优点：**
- 内存安全（不用担心内存泄漏）
- 现代语法，易于学习
- 编译检查严格，减少 bug

**缺点：**
- 学习曲线陡
- 生态没有 C++ 成熟

**适用场景：** 快速开发、需要安全性的项目

> 💡 **对于学习来说**：建议先学习 **Rust 版本**，因为：
> 1. 代码更简洁易懂
> 2. Rust 的编译器会给出很好的错误提示
> 3. 现代编程语言，语法更友好

---

## 2.3 Rust 版本核心文件

### 主入口：main.rs
```rust
// rust/src/main.rs

mod core;       // 引入核心模块
mod rendering;  // 引入渲染模块
mod resources;  // 引入资源模块
mod scene;      // 引入场景模块
mod script;     // 引入脚本模块

fn main() {
    // 程序从这里开始执行
    println!("游戏引擎启动！");
}
```

---

## 2.4 各模块职责（Rust 版本）

### 目录结构对应关系：

| 目录 | 模块名 | 职责 |
|------|--------|------|
| `rust/src/core/` | core | 核心引擎 Engine 类 |
| `rust/src/rendering/` | rendering | 图形渲染 |
| `rust/src/resources/` | resources | 加载图片、音频 |
| `rust/src/scene/` | scene | 场景管理 |
| `rust/src/script/` | script | 脚本解析 |

---

## 2.5 关键源文件

### 2.5.1 核心引擎 (core/engine.rs)

这是整个引擎的心脏，负责协调所有子系统：

```rust
// 简化版代码
pub struct Engine {
    config: GameConfig,                      // 游戏配置
    resource_manager: ...,                   // 资源管理器
    render_engine: ...,                      // 渲染引擎
    scene_manager: ...,                      // 场景管理器
    script_engine: ...,                      // 脚本引擎
    running: bool,                           // 是否运行中
}

impl Engine {
    // 创建新引擎
    pub fn new() -> Self { ... }
    
    // 初始化引擎
    pub fn initialize(&mut self, config: GameConfig) -> Result<()> { ... }
    
    // 运行主循环
    pub fn run(&mut self) -> Result<()> { ... }
    
    // 关闭引擎
    pub fn shutdown(&mut self) { ... }
}
```

### 2.5.2 脚本引擎 (script/)

脚本引擎负责解析和执行剧情脚本：

```rust
// 命令类型
pub enum CommandType {
    Dialogue,     // 对话
    Choice,       // 选择
    Background,   // 背景
    Character,    // 角色
    Music,        // 音乐
    // ... 其他命令
}

// 脚本命令
pub struct Command {
    pub command_type: CommandType,
    pub params: Vec<String>,  // 参数列表
}
```

### 2.5.3 场景管理 (scene/)

管理游戏中的不同场景：

```rust
pub struct SceneManager {
    scenes: HashMap<String, Box<dyn Scene>>,
    current_scene: Option<String>,
}

pub trait Scene {
    fn on_enter(&mut self);
    fn on_exit(&mut self);
    fn update(&mut self, delta_time: f32);
    fn render(&mut self);
}
```

---

## 2.6 Cargo.toml - 项目配置文件

这是 Rust 项目的"名片"，告诉编译器如何构建项目：

```toml
[package]
name = "visual-novel-engine"
version = "0.1.0"
edition = "2021"

[dependencies]
# 图形渲染
wgpu = "0.17"          # 现代图形 API
winit = "0.29"         # 窗口管理

# 音频
rodio = "0.17"         # 音频播放

# 图片
image = "0.24"         # 图片加载

# 序列化
serde = "1.0"          # 数据序列化
serde_json = "1.0"    # JSON 处理

# 线程安全
parking_lot = "0.12"   # 更快的 Mutex

# 日志
log = "0.4"           # 日志接口
env_logger = "0.10"   # 日志实现

# 错误处理
anyhow = "1.0"        # 灵活的错误类型
```

---

## 2.7 今日总结

今天我们学习了：
- ✅ 项目整体目录结构
- ✅ C++ 和 Rust 两种语言版本的特点
- ✅ 各模块（core, rendering, resources, scene, script）的职责
- ✅ 核心文件的作用
- ✅ Cargo.toml 配置文件

---

## 2.8 思考题

1. 为什么游戏引擎需要分成这么多模块？不能把所有代码写在一个文件里吗？
2. 如果你要添加一个新功能（比如视频播放），应该新增还是放到现有模块里？

---

## 2.9 明日预告

明天我们将学习：**Rust 基础语法 1** - 变量、数据类型、函数等基础内容。即使你没有编程经验也能学会！

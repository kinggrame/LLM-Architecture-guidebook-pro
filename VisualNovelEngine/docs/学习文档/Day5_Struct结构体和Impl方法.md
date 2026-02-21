# 第5天：结构体（Struct）与方法（Impl）

> 📦 **目标**：学会创建自定义数据类型，把相关的数据组织在一起。

---

## 5.1 什么是结构体？

**结构体**是一种自定义数据类型，可以把多个相关的值组合在一起。

**比喻**：就像一张"名片"，包含姓名、电话、邮箱等多个字段。

---

## 5.2 定义结构体

### 5.2.1 基本结构体

```rust
// 定义一个玩家结构体
struct Player {
    name: String,      // 名字
    level: i32,        // 等级
    health: f32,       // 生命值
    is_alive: bool,    // 是否存活
}

fn main() {
    // 创建结构体实例
    let player = Player {
        name: String::from("小林"),
        level: 1,
        health: 100.0,
        is_alive: true,
    };
    
    // 访问字段
    println!("玩家: {}, 等级: {}, 生命值: {}", 
        player.name, 
        player.level, 
        player.health
    );
}
```

### 5.2.2 简化创建（字段初始化简写）

当变量名和字段名相同时，可以简化：

```rust
fn main() {
    let name = String::from("小美");
    let level = 5;
    let health = 80.0;
    
    // 简写（字段名 = 变量名）
    let player = Player {
        name,
        level,
        health,
        is_alive: true,
    };
}
```

### 5.2.3 可变结构体

```rust
fn main() {
    let mut player = Player {
        name: String::from("小林"),
        level: 1,
        health: 100.0,
        is_alive: true,
    };
    
    // 修改字段
    player.level = 2;
    player.health -= 10.0;
    
    println!("等级: {}, 生命值: {}", player.level, player.health);
}
```

---

## 5.3 为结构体定义方法（impl）

使用 `impl` 关键字可以为结构体定义**方法**（类似其他语言的成员函数）。

### 5.3.1 基本方法

```rust
struct Player {
    name: String,
    level: i32,
    health: f32,
}

impl Player {
    // 这是一个方法，第一个参数是 self（指向实例本身）
    
    // 创建新玩家（关联函数，类似构造函数）
    fn new(name: &str) -> Self {
        Player {
            name: String::from(name),
            level: 1,
            health: 100.0,
        }
    }
    
    // 获取玩家信息
    fn get_info(&self) -> String {
        format!("{} - Lv.{} HP:{}", self.name, self.level, self.health)
    }
    
    // 升级
    fn level_up(&mut self) {
        self.level += 1;
        self.health = 100.0;  // 升级回满血
    }
    
    // 受伤
    fn take_damage(&mut self, damage: f32) {
        self.health -= damage;
        if self.health <= 0.0 {
            self.health = 0.0;
        }
    }
}

fn main() {
    let mut player = Player::new("小林");
    
    println!("{}", player.get_info());  // 小林 - Lv.1 HP:100
    
    player.take_damage(30.0);
    println!("受伤后: {}", player.get_info());  // 小林 - Lv.1 HP:70
    
    player.level_up();
    println!("升级后: {}", player.get_info());  // 小林 - Lv.2 HP:100
}
```

### 5.3.2 self、&self、&mut self 的区别

| 形式 | 说明 | 能否修改字段 |
|------|------|-------------|
| `self` | 获取所有权 | ✅ 可以 |
| `&self` | 只读借用 | ❌ 不能 |
| `&mut self` | 可变借用 | ✅ 可以 |

> 💡 **最佳实践**：大多数方法使用 `&self`，只读访问即可，需要修改时用 `&mut self`。

---

## 5.4 枚举（Enum）

枚举用于表示**有限的可能情况**。

### 5.4.1 基本枚举

```rust
// 定义命令类型枚举
enum CommandType {
    None,
    Dialogue,     // 对话
    Choice,       // 选择
    Background,   // 背景
    Character,    // 角色
    Music,        // 音乐
    Jump,         // 跳转
    End,          // 结束
}

fn main() {
    let cmd = CommandType::Dialogue;
    
    match cmd {
        CommandType::Dialogue => println!("处理对话"),
        CommandType::Choice => println!("处理选择"),
        CommandType::Background => println!("切换背景"),
        _ => println!("其他命令"),
    }
}
```

### 5.4.2 带数据的枚举

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },      // 匿名结构体
    Write(String),                 // 字符串
    ChangeColor(u8, u8, u8),      // RGB 颜色
}

fn main() {
    let msg1 = Message::Move { x: 10, y: 20 };
    let msg2 = Message::Write(String::from("你好"));
    let msg3 = Message::ChangeColor(255, 0, 0);
    
    match msg1 {
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        Message::Write(text) => println!("消息: {}", text),
        Message::ChangeColor(r, g, b) => println!("颜色: RGB({},{},{})", r, g, b),
        Message::Quit => println!("退出"),
    }
}
```

---

## 5.5 项目中的实际例子

### 5.5.1 引擎配置结构体

```rust
// rust/src/core/config.rs

#[derive(Debug, Clone)]
pub struct GameConfig {
    pub title: String,      // 游戏标题
    pub width: u32,         // 窗口宽度
    pub height: u32,        // 窗口高度
    pub vsync: bool,        // 垂直同步
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            title: "Visual Novel Engine".to_string(),
            width: 1280,
            height: 720,
            vsync: true,
        }
    }
}
```

### 5.5.2 命令结构体

```rust
// rust/src/script/command.rs

#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    None,
    Dialogue,
    Choice,
    Background,
    Character,
    Music,
    Sound,
    Transition,
    Wait,
    Label,
    Jump,
    End,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub command_type: CommandType,
    pub params: Vec<String>,
}

impl Command {
    pub fn new(command_type: CommandType, params: Vec<String>) -> Self {
        Self {
            command_type,
            params,
        }
    }
}
```

### 5.5.3 引擎主结构体

```rust
// rust/src/core/engine.rs

pub struct Engine {
    config: GameConfig,
    resource_manager: Arc<Mutex<ResourceManager>>,
    render_engine: Arc<Mutex<RenderEngine>>,
    scene_manager: Arc<Mutex<SceneManager>>,
    script_engine: Arc<Mutex<ScriptEngine>>,
    running: bool,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            config: GameConfig::default(),
            resource_manager: Arc::new(Mutex::new(ResourceManager::new())),
            render_engine: Arc::new(Mutex::new(RenderEngine::new())),
            scene_manager: Arc::new(Mutex::new(SceneManager::new())),
            script_engine: Arc::new(Mutex::new(ScriptEngine::new())),
            running: false,
        }
    }
    
    pub fn initialize(&mut self, config: GameConfig) -> Result<()> {
        self.config = config;
        // ... 初始化代码
        Ok(())
    }
}
```

---

## 5.6 Option 类型（安全的空值）

Rust 没有 `null`，而是用 `Option<T>` 表示"可能有值，也可能是空"：

```rust
fn main() {
    // Some 表示有值
    let name: Option<String> = Some(String::from("小林"));
    
    // None 表示空
    let empty: Option<String> = None;
    
    // 使用 match 处理
    match name {
        Some(n) => println!("名字: {}", n),
        None => println!("没有名字"),
    }
    
    // 或者用 if let 简化
    if let Some(n) = empty {
        println!("{}", n);
    } else {
        println!("是空的");
    }
}
```

这在项目中经常用到，比如查找场景：

```rust
// 场景可能不存在
pub fn get_scene(&self, name: &str) -> Option<&Box<dyn Scene>> {
    self.scenes.get(name)
}
```

---

## 5.7 今日总结

今天我们学习了：
- ✅ **结构体（Struct）** - 自定义数据类型
- ✅ **impl** - 为结构体定义方法
- ✅ **self、&self、&mut self** - 方法的参数
- ✅ **枚举（Enum）** - 有限的可能性
- ✅ **Option<T>** - 安全的空值处理

---

## 5.8 练习题

1. 创建一个 `Character` 结构体，包含：名字、表情图片路径、位置
2. 为 `Character` 添加方法：`show()` 显示角色，`hide()` 隐藏角色
3. 创建一个 `SceneState` 枚举，表示：标题画面游戏中对话框 暂停画面

---

## 5.9 明日预告

明天我们将学习：
- **模块系统（mod）** - 如何组织代码
- **crate 和包管理** - 使用 Cargo
- **可见性（pub）** - 控制哪些可以公开访问

# 第3天：Rust 基础语法 - 变量、数据类型、函数

> 🏁 **目标**：今天结束后，你将能够读懂简单的 Rust 代码，理解变量声明、基础数据类型和函数定义。

---

## 3.1 第一个 Rust 程序

让我们从最简单的程序开始：

```rust
fn main() {
    println!("Hello, Visual Novel Engine!");
}
```

**代码解释：**
- `fn main()` - 定义一个名为 `main` 的函数（程序的入口点）
- `println!()` - 打印文本到屏幕（注意有个 `!`，这是宏，不是函数）
- `;` - 每条语句以分号结束

> 📝 **小知识**：`main` 函数是每个 Rust 程序的入口，就像一本书的第一章。

---

## 3.2 变量和可变性

### 3.2.1 变量声明

```rust
fn main() {
    // 声明一个变量，默认不可变
    let age = 20;
    let name = "小林";
    let pi = 3.14;
    
    println!("名字: {}, 年龄: {}, π: {}", name, age, pi);
}
```

### 3.2.2 可变变量

如果需要修改变量，需要加上 `mut`：

```rust
fn main() {
    let mut score = 0;
    
    println!("初始分数: {}", score);
    
    score = 100;  // 修改值
    
    println!("最终分数: {}", score);
}
```

**关键点：**
- `let` - 声明变量
- `mut` - 让变量可变（mutable）
- 变量名使用 snake_case（如 `game_score`）

---

## 3.3 基础数据类型

### 3.3.1 整数

```rust
fn main() {
    let a: i32 = 42;      // 32位有符号整数（最常用）
    let b: i64 = 1000;    // 64位有符号整数
    let c: u32 = 10;      // 32位无符号整数（不能是负数）
    
    println!("a={}, b={}, c={}", a, b, c);
}
```

### 3.3.2 浮点数

```rust
fn main() {
    let x: f32 = 3.14;    // 32位浮点数
    let y: f64 = 2.71828; // 64位浮点数（更精确，默认）
    
    println!("x={}, y={}", x, y);
}
```

### 3.3.3 布尔值

```rust
fn main() {
    let is_running: bool = true;
    let is_paused: bool = false;
    
    println!("运行中: {}, 暂停: {}", is_running, is_paused);
}
```

### 3.3.4 字符和字符串

```rust
fn main() {
    let c: char = 'A';              // 单个字符
    let s1: &str = "Hello";         // 字符串切片（不可变）
    let s2: String = String::from("World");  // 可变字符串
    
    println!("字符: {}, 字符串1: {}, 字符串2: {}", c, s1, s2);
}
```

---

## 3.4 数据类型速查表

| 类型 | 说明 | 示例 |
|------|------|------|
| `i32` | 整数（-21亿 ~ 21亿） | `let a = 42;` |
| `i64` | 大整数 | `let b = 1000i64;` |
| `f32` | 32位浮点数 | `let c = 3.14f32;` |
| `f64` | 64位浮点数（默认） | `let d = 3.14;` |
| `bool` | 布尔值 | `let e = true;` |
| `char` | 单个字符 | `let f = '中';` |
| `String` | 可变字符串 | `let g = String::new();` |
| `&str` | 字符串切片 | `"hello"` |

---

## 3.5 函数

### 3.5.1 定义函数

```rust
// 定义一个打招呼的函数
fn greet(name: &str) -> String {
    // -> String 表示返回 String 类型
    format!("你好，{}！", name)
}

fn main() {
    let message = greet("小林");
    println!("{}", message);  // 输出: 你好，小林！
}
```

### 3.5.2 无返回值的函数

```rust
fn print_game_title() {
    println!("========================");
    println!("  Visual Novel Engine  ");
    println!("========================");
}

fn main() {
    print_game_title();
}
```

### 3.5.3 多参数函数

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // Rust 中最后的表达式就是返回值（不用加 return）
}

fn main() {
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);  // 输出: 5 + 3 = 8
}
```

---

## 3.6 项目中的实际例子

让我们看看引擎代码中的变量和函数：

### 3.6.1 游戏配置 (rust/src/core/config.rs)

```rust
// 这是一个结构体定义，我们明天会详细讲
pub struct GameConfig {
    pub title: String,      // 游戏标题
    pub width: u32,         // 窗口宽度
    pub height: u32,        // 窗口高度
    pub vsync: bool,        // 是否垂直同步
}

// 使用配置创建引擎
fn main() {
    let config = GameConfig {
        title: "我的视觉小说".to_string(),
        width: 1280,
        height: 720,
        vsync: true,
    };
    
    println!("游戏: {}, 分辨率: {}x{}", 
        config.title, 
        config.width, 
        config.height
    );
}
```

### 3.6.2 命令参数 (rust/src/script/command.rs)

```rust
// 命令结构体
pub struct Command {
    pub command_type: CommandType,  // 命令类型
    pub params: Vec<String>,         // 参数列表（Vec 是动态数组）
}

fn main() {
    // 创建一个显示背景的命令
    let cmd = Command {
        command_type: CommandType::Background,
        params: vec!["classroom.jpg".to_string()],
    };
    
    println!("命令类型: {:?}", cmd.command_type);
    println!("参数: {:?}", cmd.params);
}
```

---

## 3.7 运算符

### 3.7.1 算术运算符

```rust
fn main() {
    let a = 10;
    let b = 3;
    
    println!("加: {} + {} = {}", a, b, a + b);  // 13
    println!("减: {} - {} = {}", a, b, a - b);  // 7
    println!("乘: {} * {} = {}", a, b, a * b);  // 30
    println!("除: {} / {} = {}", a, b, a / b);  // 3
    println!("余: {} % {} = {}", a, b, a % b);  // 1
}
```

### 3.7.2 比较运算符

```rust
fn main() {
    let a = 5;
    let b = 10;
    
    println!("a == b: {}", a == b);  // false
    println!("a != b: {}", a != b);  // true
    println!("a < b: {}", a < b);    // true
    println!("a > b: {}", a > b);    // false
}
```

---

## 3.8 今日总结

今天我们学习了：
- ✅ `fn main()` - 程序入口
- ✅ `let` - 声明变量
- ✅ `mut` - 可变变量
- ✅ 基础数据类型：整数、浮点、布尔、字符、字符串
- ✅ 函数定义和返回值
- ✅ 运算符

---

## 3.9 练习题

1. 写一个函数，接收两个整数，返回较大的那个
2. 创建一个表示"玩家"的结构，包含：名字（String）、等级（i32）、生命值（f32）

---

## 3.10 明日预告

明天我们将学习：
- **条件语句**（if/else）
- **循环**（for、while）
- **Rust 独有的概念：所有权**（这是 Rust 最核心的特性！）

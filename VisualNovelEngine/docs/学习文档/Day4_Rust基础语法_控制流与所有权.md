# 第4天：Rust 基础语法 - 控制流与所有权

> ⚠️ **重要**：今天的"所有权"部分是 Rust 最核心的概念，理解它能帮助你理解为什么 Rust 既安全又高效！

---

## 4.1 条件语句：if / else

### 4.1.1 基本的 if / else

```rust
fn main() {
    let score = 85;
    
    if score >= 90 {
        println!("优秀！");
    } else if score >= 60 {
        println!("及格");
    } else {
        println!("需要努力！");
    }
}
```

### 4.1.2 作为表达式的 if

在 Rust 中，`if` 可以返回值（类似三元运算符 `? :`）：

```rust
fn main() {
    let score = 75;
    
    // if 作为表达式，返回值必须类型一致
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else {
        "D"
    };
    
    println!("得分 {} 对应等级 {}", score, grade);
}
```

---

## 4.2 循环：for 和 while

### 4.2.1 for 循环

```rust
fn main() {
    // 遍历数组
    let characters = ["小林", "小美", "老师"];
    
    for name in characters.iter() {
        println!("角色: {}", name);
    }
    
    // 遍历范围
    for i in 0..5 {
        println!("i = {}", i);  // 输出 0, 1, 2, 3, 4
    }
}
```

### 4.2.2 while 循环

```rust
fn main() {
    let mut health = 100;
    
    while health > 0 {
        println!("生命值: {}", health);
        health -= 20;
    }
    
    println!("游戏结束！");
}
```

### 4.2.3 循环中的 break 和 continue

```rust
fn main() {
    for i in 0..10 {
        if i == 3 {
            continue;  // 跳过 i == 3 的情况
        }
        
        if i == 7 {
            break;     // 跳出循环
        }
        
        println!("i = {}", i);
    }
    
    println!("循环结束");
}
```

---

## 4.3 match 模式匹配

`match` 类似于其他语言的 `switch`，但更强大：

```rust
fn main() {
    let command = "jump";
    
    match command {
        "jump" => println!("跳跃！"),
        "attack" => println!("攻击！"),
        "defend" => println!("防御！"),
        _ => println!("未知命令"),  // _ 类似 default
    }
}
```

```rust
// match 也可以返回值
fn main() {
    let num = 3;
    
    let result = match num {
        1 => "一",
        2 => "二",
        3 => "三",
        _ => "其他",
    };
    
    println!("数字: {}", result);
}
```

---

## 4.4 🌟 核心概念：所有权（Ownership）

> 这是 Rust 最重要的特性！它让 Rust 不需要垃圾回收器也能自动管理内存。

### 4.4.1 什么是所有权？

**简单的比喻：**
想象你有一本书（数据），你把书借给别人（赋值给另一个变量）：
- 在其他语言中，你们俩可能同时拥有这本书（两份拷贝）
- 在 Rust 中，你把书给了别人，你就没有这本书了（所有权转移）

```rust
fn main() {
    let s1 = String::from("hello");  // s1 拥有这块内存
    let s2 = s1;                      // 所有权从 s1 转移到 s2
    
    // println!("{}", s1);  // ❌ 错误！s1 已经不再拥有这个数据
    println!("{}", s2);              // ✅ 正确
}
```

### 4.4.2 为什么这样设计？

**好处：**
- 不需要垃圾回收器（GC），性能更好
- 编译时就能确保没有内存错误
- 不会出现"悬挂指针"（指向已释放的内存）

### 4.4.3 所有权规则

Rust 的所有权有三条规则：
1. 每个值都有一个**所有者**（owner）
2. 同一时刻只能有一个所有者
3. 当所有者离开作用域时，值会被**自动释放**

```rust
fn main() {
    {  // s 在这里创建
        let s = String::from("hello");
        println!("{}", s);
    }  // s 在这里离开作用域，内存被自动释放
}
```

### 4.4.4 引用（ Borrowing ）

如果你想"借用"数据而不获得所有权，可以使用**引用** `&`：

```rust
fn main() {
    let s1 = String::from("hello");
    
    // & 表示借用，不获得所有权
    let len = calculate_length(&s1);
    
    println!("'{}' 的长度是 {}", s1, len);  // s1 仍然可用
}

// 使用引用作为参数
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### 4.4.5 可变引用

如果需要修改借用的数据，使用 `&mut`：

```rust
fn main() {
    let mut s = String::from("hello");
    
    change(&mut s);
    
    println!("{}", s);  // 输出: hello, world
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

> ⚠️ **注意**：在 Rust 中，**同时只能有一个可变引用**，这可以避免数据竞争。

---

## 4.5 生命周期（Lifetimes）

### 4.5.1 什么是生命周期？

生命周期是 Rust 用来确保**引用始终有效**的机制。简单来说，它告诉编译器引用"活"多长时间。

### 4.5.2 生命周期的基本语法

```rust
// 生命周期标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("hello");
    let result;
    {
        let string2 = String::from("world!");
        result = longest(&string1, &string2);
        println!("最长的字符串: {}", result);
    }
}
```

### 4.5.3 生命周期的规则

1. **每个引用参数都有自己的生命周期**
2. **如果有多个引用，返回值需要标注生命周期**
3. **返回的引用必须来自输入的引用之一**

### 4.5.4 结构体中的生命周期

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("{}", excerpt.part);
}
```

---

## 4.6 Copy 和 Clone

### 4.6.1 Copy trait

对于**简单类型**（如整数、浮点数、布尔值），Rust 会自动实现 `Copy`，这意味着这些值会被**复制**而不是**移动**：

```rust
fn main() {
    let x = 5;
    let y = x;  // 复制，而不是移动
    
    println!("x = {}, y = {}", x, y);  // 两个都可以使用
}
```

### 4.6.2 可复制和不可复制的类型

```rust
fn main() {
    // 可以 Copy 的类型
    let a: i32 = 42;
    let b = a;  // 复制
    println!("a = {}, b = {}", a, b);  // OK
    
    // 不能 Copy 的类型（使用 move）
    let s1 = String::from("hello");
    let s2 = s1;  // 移动
    // println!("{}", s1);  // 错误！s1 已经失效
    println!("{}", s2);  // OK
}
```

### 4.6.3 Clone trait

如果你需要显式复制一个值，可以使用 `clone()`：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // 显式克隆
    
    println!("s1 = {}, s2 = {}", s1, s2);  // 两个都可以使用
}
```

### 4.6.4 自定义类型实现 Copy 和 Clone

```rust
#[derive(Clone, Copy)]  // 编译器自动实现
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1;  // 复制（因为实现了 Copy）
    let p3 = p1.clone();  // 克隆
    
    println!("p1: ({}, {})", p1.x, p1.y);  // 仍然有效
}
```

---

## 4.7 项目中的实际例子

### 4.7.1 引擎中的所有权使用

```rust
// rust/src/core/engine.rs

pub struct Engine {
    // 这些字段使用 Arc<Mutex<...>> 来共享所有权
    // 详细解释见后文
    resource_manager: Arc<Mutex<ResourceManager>>,
    render_engine: Arc<Mutex<RenderEngine>>,
    scene_manager: Arc<Mutex<SceneManager>>,
    script_engine: Arc<Mutex<ScriptEngine>>,
}
```

### 4.7.2 脚本命令中的所有权

```rust
// rust/src/script/command.rs

impl Command {
    // 创建新命令，params 使用 Vec<String>（拥有所有权的数组）
    pub fn new(command_type: CommandType, params: Vec<String>) -> Self {
        Self {
            command_type,
            params,  // params 的所有权转移到这里
        }
    }
}
```

### 4.7.3 借用检查在引擎中的应用

```rust
// 从场景管理器获取资源
fn load_background(path: &str, resource_manager: &ResourceManager) {
    // 使用借用，不获取所有权
    let texture = resource_manager.load_texture(path);
}
```

---

## 4.8 今日总结

今天我们学习了：
- ✅ `if / else` 条件语句
- ✅ `for` 循环和 `while` 循环
- ✅ `match` 模式匹配
- ✅ **所有权（Ownership）** - Rust 的核心特性
- ✅ **引用（&）** 和 **可变引用（&mut）**
- ✅ **生命周期（'a）** - 确保引用有效
- ✅ **Copy 和 Clone** - 值的复制机制

---

## 4.9 练习题

1. 写一个程序，使用 `match` 判断分数等级（A/B/C/D）
2. 解释下面代码为什么报错：
   ```rust
   fn main() {
       let s = String::from("hello");
       let s2 = s;
       println!("{}", s);
   }
   ```
3. 修改上题，使用借用（&）让代码正确运行
4. 解释 `Copy` 和 `Clone` 的区别，什么时候应该用哪个？

---

## 4.10 明日预告

明天我们将学习：
- **结构体（Struct）** - 如何自定义数据类型
- **impl 方法** - 为结构体添加函数
- **枚举（Enum）** - 定义有限的选择

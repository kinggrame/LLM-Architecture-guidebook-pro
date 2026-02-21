# 附录 A：Rust vs C++ 对比参考

> 📊 本文档帮助你理解 Rust 和 C++ 在实现相同功能时的差异

---

## A.1 基础语法对比

### 变量声明

**Rust:**
```rust
fn main() {
    let x = 5;              // 不可变
    let mut y = 10;         // 可变
    const MAX: i32 = 100;   // 常量
}
```

**C++:**
```cpp
int main() {
    const int x = 5;        // 常量（必须初始化）
    int y = 10;             // 可变（默认可变）
    constexpr int MAX = 100;// 编译时常量
}
```

---

## A.2 字符串

**Rust:**
```rust
fn main() {
    let s1: &str = "hello";         // 字符串切片（引用）
    let s2: String = String::from("world");  // 堆分配字符串
    let s3 = s2.clone();            // 深拷贝
}
```

**C++:**
```cpp
#include <string>

int main() {
    const char* s1 = "hello";        // C 风格字符串
    std::string s2 = "world";        // std::string
    std::string s3 = s2;             // 拷贝
}
```

---

## A.3 所有权对比

### Rust（自动内存管理）

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // 所有权转移
    
    // println!("{}", s1);  // 错误！s1 已失效
    println!("{}", s2);    // 正常
}
```

### C++（手动或智能指针）

```cpp
#include <string>
#include <memory>

int main() {
    // 方式1: 手动管理（危险）
    std::string* s1 = new std::string("hello");
    std::string* s2 = s1;  // 两个指针指向同一内存
    delete s1;              // 释放 s1
    // delete s2;            // 错误！已释放
    
    // 方式2: 智能指针（推荐）
    auto s1 = std::make_unique<std::string>("hello");
    auto s2 = std::move(s1);  // 转移所有权
}
```

---

## A.4 引用和指针

### Rust 引用

```rust
fn main() {
    let mut x = 10;
    
    let r1 = &x;        // 不可变引用
    let r2 = &mut x;    // 可变引用（同时只能有一个）
    
    println!("{}", r1);
}
```

### C++ 指针和引用

```cpp
int main() {
    int x = 10;
    
    // 引用
    int& r1 = x;
    r1 = 20;  // 修改 x
    
    // 指针
    int* p = &x;
    *p = 30;  // 修改 x
    
    // 空指针
    int* null_ptr = nullptr;
}
```

---

## A.5 结构体

### Rust

```rust
struct Player {
    name: String,
    level: i32,
    health: f32,
}

impl Player {
    fn new(name: &str) -> Self {
        Player {
            name: String::from(name),
            level: 1,
            health: 100.0,
        }
    }
    
    fn level_up(&mut self) {
        self.level += 1;
    }
}
```

### C++

```cpp
class Player {
public:
    std::string name;
    int level;
    float health;
    
    Player(const std::string& name) : name(name), level(1), health(100.0) {}
    
    void levelUp() {
        level++;
    }
};
```

---

## A.6 智能指针

### Rust（所有权）

```rust
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

fn main() {
    // 单一所有者
    let a = Box::new(5);
    
    // 引用计数（单线程）
    let rc = Rc::new(5);
    let rc2 = Rc::clone(&rc);
    
    // 线程安全引用计数
    let arc = Arc::new(5);
    
    // 可变借用（内部可变性）
    let cell = RefCell::new(5);
    *cell.borrow_mut() = 10;
}
```

### C++（智能指针）

```cpp
#include <memory>

int main() {
    // 独占所有权（类似 Box）
    auto a = std::make_unique<int>(5);
    
    // 共享所有权（类似 Rc/Arc）
    auto shared = std::make_shared<int>(5);
    auto shared2 = shared;
    
    // 弱引用
    std::weak_ptr<int> weak = shared;
}
```

---

## A.7 并发

### Rust

```rust
use std::thread;
use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);
    
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    
    handle.join().unwrap();
}
```

### C++

```cpp
#include <thread>
#include <mutex>

int main() {
    std::mutex mtx;
    int counter = 0;
    
    std::thread t([&]() {
        std::lock_guard<std::mutex> lock(mtx);
        counter++;
    });
    
    t.join();
}
```

---

## A.8 项目架构对比

### Rust Engine 结构

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
```

### C++ Engine 结构

```cpp
// cpp/include/core/Engine.h
class Engine {
    std::unique_ptr<SceneManager> m_sceneManager;
    std::unique_ptr<RenderEngine> m_renderEngine;
    std::unique_ptr<ResourceManager> m_resourceManager;
    std::unique_ptr<ScriptEngine> m_scriptEngine;
    
    bool m_running = false;
};
```

---

## A.9 总结表

| 特性 | Rust | C++ |
|------|------|-----|
| 内存安全 | 编译时保证 | 手动/智能指针 |
| 所有权 | 唯一owner | 拷贝/引用计数 |
| 空指针 | Option<T> | nullptr |
| 线程安全 | Rust默认安全 | 需要mutex等 |
| 泛型 | 强大( trait bound) | 模板 |
| 错误处理 | Result/panic | 异常 |

---

## A.10 学习建议

1. **先学 Rust**：语法更安全，编译器会帮助你学习
2. **理解概念**：所有权、生命周期是核心
3. **对比学习**：用已学的 Rust 概念理解 C++
4. **实践为王**：多写代码，多调试

---

## A.11 明日预告

从明天开始，我们将进入 **延展学习阶段**，每天提供：
- 精选代码 Demo
- 详细的学习内容
- 核心关键词

目标是让你掌握 Rust 系统级编程和 C++ 编程能力。

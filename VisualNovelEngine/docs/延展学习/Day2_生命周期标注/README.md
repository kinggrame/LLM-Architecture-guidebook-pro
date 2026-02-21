# Day 2: 生命周期标注

> 🎯 **目标**：理解 Rust 的生命周期系统，学会在复杂场景中使用生命周期标注

---

## 学习目标

1. 理解为什么需要生命周期
2. 掌握生命周期标注语法
3. 在结构体和方法中使用生命周期
4. 理解生命周期省略规则

---

## 代码 Demo

### Demo 1: 生命周期的基本概念

```rust
// src/lifetime_basic.rs

/// 生命周期示例
/// 
/// 生命周期是 Rust 用来确保引用始终有效的机制
/// 它告诉编译器引用的有效范围

fn main() {
    println!("=== 演示1: 悬空引用问题 ===");
    
    // 下面的代码无法编译，因为返回的引用会指向已释放的局部变量
    // let result = dangle();
    
    println!("\n=== 演示2: 使用生命周期 ===");
    
    let string1 = String::from("hello");
    let string2 = String::from("world!");
    
    let result = longest(&string1, &string2);
    println!("最长的字符串是: {}", result);
    
    println!("\n=== 演示3: 生命周期与作用域 ===");
    
    let string1 = String::from("hello");
    {
        let string2 = String::from("world!");
        let result = longest(&string1, &string2);
        println!("最长的字符串是: {}", result);
    } // string2 在这里被释放
    
    // println!("{}", result); // 错误！result 引用的数据可能已失效
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

---

### Demo 2: 结构体中的生命周期

```rust
// src/lifetime_struct.rs

/// 结构体中使用生命周期
/// 
/// 当结构体包含引用时，需要标注生命周期

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("注意: {}", announcement);
        self.part
    }
}

fn main() {
    println!("=== 演示1: 结构体中的生命周期 ===");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("引用内容: {}", excerpt.part);
    println!("等级: {}", excerpt.level());
    
    println!("\n=== 演示2: 方法中的生命周期 ===");
    
    let result = excerpt.announce_and_return("这是一段重要的引用");
    println!("返回: {}", result);
    
    println!("\n=== 演示3: 多个生命周期参数 ===");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first = novel.split('.').next().unwrap();
    
    let excerpt = ImportantExcerpt { part: first };
    
    // 调用返回生命周期的方法
    let s = "announcement";
    let returned = excerpt.announce_and_return(s);
    println!("返回的引用: {}", returned);
}
```

---

### Demo 3: 生命周期标注的省略

```rust
// src/lifetime_elision.rs

/// Rust 的生命周期省略规则
/// 
/// 在某些情况下，编译器可以自动推断生命周期

// 省略前
fn first_word_original(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 省略后（Rust 2018+）
// 编译器会自动推断：
// 1. 每个省略的输入生命周期成为不同的生命周期参数
// 2. 如果只有一个输入生命周期，它被赋给所有省略的输出生命周期
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    println!("=== 演示: 生命周期省略 ===");
    
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("第一个单词: {}", word);
    
    // 省略规则：
    // 1. 每个引用参数获得自己的生命周期
    // 2. 如果只有一个输入生命周期，所有输出生命周期都获得它
    // 3. 如果有 &self 或 &mut self 参数，它获得所有输出生命周期
}
```

---

### Demo 4: 静态生命周期

```rust
// src/static_lifetime.rs

/// 静态生命周期 'static
/// 
/// 静态生命周期表示程序整个运行期间都有效的引用

fn main() {
    println!("=== 演示1: 静态字符串 ===");
    
    // 字符串字面量具有 'static 生命周期
    let s: &'static str = "我是一个静态字符串";
    println!("{}", s);
    
    println!("\n=== 演示2: 静态生命周期在函数中 ===");
    
    // 某些情况下需要显式声明 'static
    print_static_string();
}

fn print_static_string() {
    let s: &'static str = "这个字符串将一直有效";
    println!("{}", s);
}

fn print_string(s: &str) {
    println!("{}", s);
}
```

---

### Demo 5: 与 C++ 对比

```rust
// src/rust_vs_cpp_lifetime.rs

/// Rust 生命周期 vs C++ 智能指针

// ===== Rust 版本 =====

struct RustWrapper<'a> {
    data: &'a str,
}

impl<'a> RustWrapper<'a> {
    fn new(data: &'a str) -> Self {
        RustWrapper { data }
    }
    
    fn get(&self) -> &str {
        self.data
    }
}

fn rust_lifetime_demo() {
    println!("=== Rust 生命周期 ===");
    
    let data = String::from("hello");
    let wrapper = RustWrapper::new(&data);
    println!("数据: {}", wrapper.get());
    // data 和 wrapper 同时有效
}

// ===== C++ 版本 =====

// C++ 没有编译器的生命周期检查
// 需要开发者手动确保引用有效性

class CppWrapper {
private:
    std::string* m_data;
    
public:
    CppWrapper(std::string* data) : m_data(data) {}
    
    std::string* get() { return m_data; }
    
    // 警告：调用者必须确保指针有效！
};

void cpp_lifetime_demo() {
    std::cout << "=== C++ 手动管理 ===" << std::endl;
    
    auto data = std::make_unique<std::string>("hello");
    auto wrapper = CppWrapper(data.get());
    std::cout << "数据: " << *wrapper.get() << std::endl;
    
    // data 超出作用域被释放，wrapper.get() 变成悬空指针！
    // 这就是典型的 use-after-free 错误
}
```

---

## 核心关键词

### 生命周期基础
- **Lifetime (生命周期)**: 引用有效的作用域范围
- **Lifetime Annotation ('a)**: 生命周期的标注语法
- **Lifetime Parameter (生命周期参数)**: 泛型生命周期参数
- **Lifetime Elision (生命周期省略)**: 编译器自动推断生命周期

### 生命周期规则
- **Input Lifetime (输入生命周期)**: 函数参数的引用生命周期
- **Output Lifetime (输出生命周期)**: 函数返回值的引用生命周期
- **Static Lifetime ('static)**: 程序整个运行期间有效的生命周期
- **Lifetime Bounds (生命周期约束)**: 指定泛型类型必须满足的生命周期

### 编译器概念
- **Borrow Checker (借用检查器)**: 确保引用不悬空的编译器组件
- **NLL (Non-Lexical Lifetimes)**: 非词法生命周期，Rust 2018+ 特性

---

## 练习题

### 基础练习

1. **标注生命周期**：
   修复以下函数，使其能够编译：
   ```rust
   fn longest(x: &str, y: &str) -> &str {
       if x.len() > y.len() { x } else { y }
   }
   ```

2. **理解输出**：
   ```rust
   fn main() {
       let string1 = String::from("hello");
       let result;
       {
           let string2 = String::from("world!");
           result = longest(&string1, &string2);
       }
       println!("{}", result); // 这行会出错吗？
   }
   ```

### 进阶练习

3. **实现一个带生命周期的结构体**：
   创建 `Config` 结构体，包含一个对配置字符串的引用。

4. **生命周期推断**：
   研究为什么某些函数不需要显式标注生命周期。

---

## 延伸阅读

- [The Rust Book - 生命周期](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Rust by Example - 生命周期](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)
- [Nomicon - 生命周期](https://doc.rust-lang.org/nomicon/lifetimes.html)

---

## 下一天预告

**Day 3: Trait 系统**

- trait 定义和实现
- 默认实现和 trait 继承
- trait object 和动态分发
- 常用标准库 trait

---

*💡 提示：运行代码时注意观察借用检查器如何阻止悬空引用！*

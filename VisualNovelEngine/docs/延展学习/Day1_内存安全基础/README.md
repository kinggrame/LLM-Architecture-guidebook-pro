# Day 1: Rust 内存安全基础

> 🏁 **目标**：深入理解 Rust 的所有权系统，掌握内存安全的底层原理

---

## 学习目标

1. 理解所有权规则
2. 掌握借用检查机制
3. 理解 Rust 如何在编译时防止内存错误
4. 对比 C++ 的内存管理方式

---

## 代码 Demo

### Demo 1: 基本所有权演示

```rust
// src/ownership_basics.rs

/// 演示 Rust 的基本所有权规则
/// 
/// 规则1: 每个值都有一个所有者
/// 规则2: 同一时刻只能有一个所有者
/// 规则3: 当所有者离开作用域时，值会被丢弃

fn main() {
    println!("=== 演示1: 所有权转移 ===");
    
    // s1 拥有 String 的所有权
    let s1 = String::from("hello");
    println!("s1 = {}", s1);
    
    // 所有权从 s1 转移到 s2
    let s2 = s1;
    // println!("s1 = {}", s1); // 编译错误！s1 不再有效
    println!("s2 = {}", s2);
    
    println!("\n=== 演示2: 作用域结束自动释放 ===");
    
    {
        let s3 = String::from("world");
        println!("s3 = {}", s3);
    } // s3 在这里被自动释放（drop）
    
    println!("s3 已经不存在了");
    
    println!("\n=== 演示3: 函数参数的所有权 ===");
    
    let s4 = String::from("ownership");
    takes_ownership(s4);
    // println!("s4 = {}", s4); // 编译错误！s4 的所有权已转移
    
    println!("\n=== 演示4: 返回值的所有权 ===");
    
    let s5 = String::from("返回值");
    let s6 = gives_ownership();
    println!("s5 = {}", s5);
    println!("s6 = {}", s6);
}

/// 这个函数获取传入字符串的所有权
fn takes_ownership(s: String) {
    println!("收到所有权: {}", s);
} // 函数结束时 s 被释放

/// 这个函数返回一个新的 String，所有权转移给调用者
fn gives_ownership() -> String {
    String::from("新创建的值")
}
```

**运行结果：**
```
=== 演示1: 所有权转移 ===
s1 = hello
s2 = hello

=== 演示2: 作用域结束自动释放 ===
s3 = world
s3 已经不存在了

=== 演示3: 函数参数的所有权 ===
收到所有权: ownership

=== 演示4: 返回值的所有权 ===
s5 = 返回值
s6 = 新创建的值
```

---

### Demo 2: 借用（Borrowing）

```rust
// src/borrowing.rs

/// 演示 Rust 的借用机制
/// 
/// 借用允许你使用值而不获得所有权

fn main() {
    println!("=== 演示1: 不可变借用 ===");
    
    let s1 = String::from("hello");
    
    // & 表示不可变借用
    let len = calculate_length(&s1);
    
    println!("s1 = {}, 长度 = {}", s1, len);
    // s1 仍然有效
    
    println!("\n=== 演示2: 可变借用 ===");
    
    let mut s2 = String::from("hello");
    modify_string(&mut s2);
    println!("修改后: {}", s2);
    
    println!("\n=== 演示3: 借用规则 ===");
    
    // 规则1: 不可变借用和可变借用不能同时存在
    let mut s3 = String::from("hello");
    
    let r1 = &s3;
    // let r2 = &mut s3; // 编译错误！不能同时有不可变借用和可变借用
    println!("r1 = {}", r1);
    
    // r1 的作用域结束后，才可以创建可变借用
    let r2 = &mut s3;
    println!("r2 = {}", r2);
    
    println!("\n=== 演示4: 悬空引用 ===");
    
    // 编译器会阻止创建悬空引用
    // 下面的代码无法编译：
    // let reference_to_nothing = dangle();
}

// 计算字符串长度（借用，不获取所有权）
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 修改字符串（可变借用）
fn modify_string(s: &mut String) {
    s.push_str(", world");
}

// 这个函数会返回悬空引用，编译不通过
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // s 在函数结束时被释放，返回的引用无效
// }
```

---

### Demo 3: 借用检查器实战

```rust
// src/borrow_checker_examples.rs

/// 展示常见的借用检查错误及修正方法

fn main() {
    // 错误1: 可变借用和不可变借用同时存在
    problem1();
    solution1();
    
    // 错误2: 在借用期间修改数据
    problem2();
    solution2();
    
    // 错误3: 返回局部变量的引用
    problem3();
}

/// 问题1: 同时存在可变和不可变借用
fn problem1() {
    println!("\n--- 问题1: 同时借用 ---");
    let mut s = String::from("hello");
    
    // 第一个不可变借用
    let r1 = &s;
    // let r2 = &mut s; // 错误！
    
    println!("r1 = {}", r1);
}

/// 解决方案1: 确保不可变借用使用完再创建可变借用
fn solution1() {
    println!("\n--- 解决方案1 ---");
    let mut s = String::from("hello");
    
    let r1 = &s;
    println!("r1 = {}", r1);
    // r1 不再使用
    
    let r2 = &mut s;
    r2.push_str(" world");
    println!("r2 = {}", r2);
}

/// 问题2: 在借用期间修改数据
fn problem2() {
    println!("\n--- 问题2: 借用期间修改 ---");
    let mut s = String::from("hello");
    
    let r1 = &s;
    // s.push_str("world"); // 错误！s 在被借用状态
    
    println!("r1 = {}", r1);
}

/// 解决方案2: 使用可变借用代替不可变借用
fn solution2() {
    println!("\n--- 解决方案2 ---");
    let mut s = String::from("hello");
    
    let r1 = &mut s;
    r1.push_str(" world");
    // println!("{}", s); // 注意：这里不能再使用 s
    println!("r1 = {}", r1);
}

/// 问题3: 返回局部变量的引用
fn problem3() {
    println!("\n--- 问题3: 悬空引用 ---");
    // 编译器会在编译时检测并阻止这个问题
    // let reference = dangle();
}

// 正确的做法：返回所有权而不是引用
fn no_dangle() -> String {
    let s = String::from("hello");
    s // 移动返回值，所有权转移
}
```

---

### Demo 4: 与 C++ 对比

```rust
// src/rust_vs_cpp.rs

/// 对比 Rust 和 C++ 的内存管理方式

// ===== Rust 版本 =====

fn rust_ownership_demo() {
    println!("=== Rust 内存安全 ===");
    
    // 1. 栈分配
    let x = 5;
    let y = x; // 复制到栈上，两个独立的值
    println!("x = {}, y = {}", x, y); // 两者都有效
    
    // 2. 堆分配（String）
    let s1 = String::from("hello");
    let s2 = s1; // 移动，所有权转移
    // println!("{}", s1); // 错误！s1 已失效
    println!("{}", s2); // 正常
    
    // 3. 借用
    let s3 = String::from("borrowed");
    let len = calculate_len(&s3);
    println!("s3 = {}, len = {}", s3, len); // s3 仍然有效
}

fn calculate_len(s: &String) -> usize {
    s.len()
}
```

**对应的 C++ 版本：**

```cpp
// rust_vs_cpp.cpp

#include <iostream>
#include <string>
#include <memory>

void cpp_memory_demo() {
    std::cout << "=== C++ 内存管理 ===" << std::endl;
    
    // 1. 栈分配
    int x = 5;
    int y = x; // 复制到栈上
    std::cout << "x = " << x << ", y = " << y << std::endl;
    
    // 2. 堆分配（危险的手动管理）
    std::string* s1 = new std::string("hello");
    std::string* s2 = s1; // 两个指针指向同一内存
    // 危险！忘记释放会导致内存泄漏
    // 危险！重复释放会导致未定义行为
    delete s1;
    // delete s2; // 错误！s1 已释放
    
    // 3. 智能指针（现代 C++ 推荐）
    auto s3 = std::make_unique<std::string>("modern");
    auto s4 = std::move(s3); // 转移所有权
    // std::cout << *s3 << std::endl; // 错误！s3 已失效
    std::cout << *s4 << std::endl; // 正常
    
    // 4. 借用（使用引用）
    std::string s5 = "borrowed";
    std::string& len = s5; // 引用，不获取所有权
    std::cout << s5 << " len = " << len.length() << std::endl;
}

int main() {
    rust_ownership_demo();
    cpp_memory_demo();
    return 0;
}
```

---

### Demo 5: 实战：安全的资源管理

```rust
// src/resource_management.rs

/// 展示 Rust 如何确保资源安全管理

use std::fs::File;
use std::io::{self, Read};

/// 使用 RAII 模式的文件读取
fn read_file_contents(path: &str) -> Result<String, io::Error> {
    // File 在作用域结束时自动关闭
    let mut file = File::open(path)?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(contents)
}

/// 对比：错误处理版本
fn read_file_safe(path: &str) -> String {
    match File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Ok(_) = file.read_to_string(&mut contents) {
                contents
            } else {
                String::from("读取失败")
            }
        }
        Err(e) => {
            eprintln!("打开文件失败: {}", e);
            String::new()
        }
    }
}

fn main() {
    println!("=== Rust RAII 资源管理 ===");
    
    // 创建一个测试文件
    std::fs::write("test.txt", "Hello, Rust!").unwrap();
    
    // 读取文件
    match read_file_contents("test.txt") {
        Ok(contents) => println!("文件内容: {}", contents),
        Err(e) => println!("错误: {}", e),
    }
    
    // 清理
    std::fs::remove_file("test.txt").ok();
    
    println!("\n=== 对比 C++ ===");
    println!("C++ 需要手动调用 close() 或使用 RAII 包装器");
    println!("Rust 自动调用 drop()，无需手动管理");
}
```

---

## 核心关键词

### 所有权相关
- **Owner (所有者)**: 负责管理内存的生命周期
- **Move (移动)**: 所有权从一个变量转移到另一个
- **Copy (复制)**: 栈上的简单类型自动复制
- **Drop (释放)**: 作用域结束时自动释放资源
- **RAII**: 资源获取即初始化，作用域结束自动释放

### 借用相关
- **Borrow (借用)**: 使用值但不获得所有权
- **Reference (引用)**: 借用创建的别名
- **Mutable Reference (&mut)**: 可变借用
- **Immutable Reference (&)**: 不可变借用
- **Dangling Reference (悬空引用)**: 指向已释放内存的引用
- **Borrow Checker (借用检查器)**: Rust 编译器的一部分，确保引用有效

### 内存相关
- **Stack (栈)**: 固定大小值存储，分配/释放快
- **Heap (堆)**: 动态大小数据存储
- **Memory Safety (内存安全)**: 防止内存泄漏、双重释放、悬空指针
- **Thread Safety (线程安全)**: 多线程环境下的内存安全

---

## 练习题

### 基础练习

1. **移动还是复制？** 预测以下代码的输出：
   ```rust
   fn main() {
       let x = 5;
       let y = x;
       println!("x = {}, y = {}", x, y);
       
       let s1 = String::from("hello");
       let s2 = s1;
       // println!("{}", s1); // 取消注释会发生什么？
       println!("{}", s2);
   }
   ```

2. **修复借用错误**：
   ```rust
   fn main() {
       let mut s = String::from("hello");
       let r1 = &s;
       let r2 = &s;
       let r3 = &mut s; // 如何修复这个错误？
       println!("{}, {}, and {}", r1, r2, r3);
   }
   ```

### 进阶练习

3. **实现一个安全的可变借出函数**：
   编写一个函数，接收一个 `&mut Vec<i32>`，向其中添加元素。

4. **对比实验**：用 C++ 和 Rust 各实现一个"资源获取后必须释放"的场景，比较两者的实现方式。

---

## 延伸阅读

- [The Rust Book - 所有权](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust by Example - 所有权](https://doc.rust-lang.org/rust-by-example/scope/move.html)
- [Rust 借用检查器详解](https://doc.rust-lang.org/nomicon/borrows.html)

---

## 下一天预告

**Day 2: 生命周期标注**

- 理解为什么需要生命周期
- 掌握生命周期标注语法
- 学会在实际代码中使用生命周期

---

*💡 提示：运行 `cargo run --bin ownership_basics` 看看所有权是如何工作的！*

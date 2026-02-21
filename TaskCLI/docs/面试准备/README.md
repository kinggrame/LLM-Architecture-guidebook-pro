# Rust/C++ 工程师面试准备指南

> 🎯 目标：系统掌握 Rust 和 C++，成功通过面试

---

## 目录

1. [Rust 核心面试题](#1-rust-核心面试题)
2. [C++ 核心面试题](#2-c-核心面试题)
3. [系统设计题](#3-系统设计题)
4. [算法与数据结构](#4-算法与数据结构)
5. [项目经验问题](#5-项目经验问题)
6. [高频问题汇总](#6-高频问题汇总)

---

## 1. Rust 核心面试题

### Q1: 解释 Rust 的所有权系统

```rust
// 核心概念：
// 1. 每个值有唯一所有者
// 2. 所有者离开作用域时值被丢弃
// 3. 默认移动语义
// 4. 借用检查器在编译时防止悬挂引用

fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // 移动，s1 不再有效
    
    // println!("{}", s1); // 编译错误！
    println!("{}", s2); // OK
}
```

### Q2: &T vs &mut T vs T

```rust
// &T - 不可变引用
// - 多个 &T 可以同时存在
// - 不能修改数据

// &mut T - 可变引用
// - 同时只能有一个
// - 可以修改数据

// T - 所有权
// - 可以移动或克隆
// - 拥有资源

fn main() {
    let mut s = String::from("hello");
    
    // 不可变借用
    let r1 = &s;
    let r2 = &s; // OK
    
    // 可变借用（需要等不可变借用结束）
    let r3 = &mut s;
}
```

### Q3: Box vs Rc vs Arc

```rust
// Box<T> - 堆分配，独占所有权
let b = Box::new(42);

// Rc<T> - 引用计数，单线程
let rc = Rc::new(42);
let rc2 = Rc::clone(&rc);

// Arc<T> - 原子引用计数，多线程
use std::sync::Arc;
let arc = Arc::new(42);
```

### Q4: 生命周期是什么？

```rust
// 生命周期是引用有效的范围
// 编译器用它确保引用不会指向已释放的内存

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### Q5: trait object vs 泛型

```rust
// 泛型 - 静态分发，无运行时开销
fn process<T: Display>(item: T) {
    println!("{}", item);
}

// trait object - 动态分发，有运行时开销
fn process_dyn(item: &dyn Display) {
    println!("{}", item);
}
```

### Q6: Result vs Option

```rust
// Option<T> - 有值或无值
fn find_user(id: u32) -> Option<User> {
    if id == 1 { Some(User) } else { None }
}

// Result<T, E> - 成功或错误
fn read_file(path: &str) -> Result<String, io::Error> {
    // ...
}
```

### Q7: unsafe Rust 何时使用？

```rust
// 1. 调用 C 库
// 2. 性能关键代码（绕过安全检查）
// 3. 实现 unsafe trait
// 4. 访问裸指针
```

### Q8: async/await 原理

```rust
// async 函数返回 Future
// await 等待 Future 完成
// 需要运行时（tokio, async-std）

async fn fetch() -> String {
    reqwest::get("url").await?.text().await
}
```

### Q9: Send 和 Sync 是什么？

```rust
// Send - 可以在线程间传递所有权
// Sync - 可以在线程间共享引用

// 基础类型默认实现
// 自定义类型需要手动实现（危险！）
```

### Q10: Rust 的错误处理

```rust
// 1. Result<T, E> - 可恢复错误
fn read() -> Result<String, Error> { ... }

// 2. panic! - 不可恢复错误
panic!("something wrong");

// 3. ? 操作符 - 传播错误
fn read() -> Result<String, Error> {
    let content = std::fs::read_to_string("file")?;
    Ok(content)
}
```

---

## 2. C++ 核心面试题

### Q1: vector 扩容机制

```cpp
// 1. 容量满时，扩容约 1.5-2 倍
// 2. 分配新内存
// 3. 移动/拷贝元素到新内存
// 4. 释放旧内存

// 优化：使用 reserve() 预分配
vector<int> v;
v.reserve(1000); // 避免多次重新分配
```

### Q2: new vs malloc

```cpp
// new:
// - 调用构造函数
// - 失败时抛出 bad_alloc
// - 返回正确类型指针

// malloc:
// - 只分配原始内存
// - 失败返回 NULL
// - 返回 void*
```

### Q3: shared_ptr 线程安全

```cpp
// shared_ptr 引用计数线程安全
// 指向的对象默认不线程安全

// 多个线程读写需要加锁：
std::mutex mtx;
std::shared_ptr<T> ptr;

void read() {
    std::lock_guard<std::mutex> lock(mtx);
    // 读操作
}
```

### Q4: RAII 是什么？

```cpp
// RAII - 资源获取即初始化
// 构造函数获取资源
// 析构函数释放资源

class File {
    FILE* f;
public:
    File(const char* path) {
        f = fopen(path, "r");
    }
    ~File() {
        if (f) fclose(f);
    }
};
```

### Q5: 虚函数和纯虚函数

```cpp
// 虚函数 - 可被重写
virtual void draw() { }

// 纯虚函数 - 必须重写
virtual void draw() = 0;

// 抽象类 - 含有纯虚函数的类
class Shape {
public:
    virtual void draw() = 0;
};
```

### Q6: 移动语义

```cpp
// 移动构造函数
Class(Class&& other) noexcept {
    data = other.data;
    other.data = nullptr; // 重要！
}

// 移动赋值
Class& operator=(Class&& other) noexcept {
    if (this != &other) {
        delete data;
        data = other.data;
        other.data = nullptr;
    }
    return *this;
}
```

### Q7: 智能指针对比

```cpp
// unique_ptr - 独占所有权
auto p = std::make_unique<int>(42);

// shared_ptr - 共享所有权
auto p = std::make_shared<int>(42);

// weak_ptr - 弱引用，不增加计数
std::weak_ptr<int> wp = p;
```

### Q8: const 和 constexpr

```cpp
// const - 运行时常量
const int MAX = 100;

// constexpr - 编译时常量
constexpr int MAX = 100;
constexpr int factorial(int n) {
    return n <= 1 ? 1 : n * factorial(n-1);
}
```

### Q9: 模板元编程

```cpp
// 模板函数
template<typename T>
T max(T a, T b) { return a > b ? a : b; }

// 模板类
template<typename T>
class Stack {
    std::vector<T> data;
public:
    void push(const T& t) { data.push_back(t); }
};
```

### Q10: C++11 新特性

```cpp
// auto - 类型推断
auto x = 42;

// lambda - 匿名函数
auto f = [](int x) { return x * 2; };

// range for
for (auto& elem : v) { }

// nullptr
int* p = nullptr;

// enum class
enum class Color { Red, Green, Blue };
```

---

## 3. 系统设计题

### Q1: 设计一个线程池

```rust
// Rust 版本
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::VecDeque;

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    tasks: Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send>>>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let tasks = Arc::new(Mutex::new(VecDeque::new()));
        
        let workers = (0..size).map(|_| {
            let tasks = Arc::clone(&tasks);
            thread::spawn(move || {
                loop {
                    let task = tasks.lock().unwrap().pop_front();
                    match task {
                        Some(t) => t(),
                        None => break,
                    }
                }
            })
        }).collect();
        
        Self { workers, tasks }
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.tasks.lock().unwrap().push_back(Box::new(f));
    }
}
```

### Q2: 设计一个缓存系统

```cpp
// LRU Cache 实现
template<typename K, typename V>
class LRUCache {
    size_t capacity_;
    std::list<std::pair<K, V>> items_;
    std::unordered_map<K, std::list<std::pair<K, V>>::iterator> cache_;
    
public:
    LRUCache(size_t capacity) : capacity_(capacity) {}
    
    V get(const K& key) {
        auto it = cache_.find(key);
        if (it == cache_.end()) return V{};
        
        items_.splice(items_.begin(), items_, it->second);
        return it->second->second;
    }
    
    void put(const K& key, const V& value) {
        if (cache_.find(key) != cache_.end()) {
            items_.splice(items_.begin(), items_, cache_[key]);
            items_.front().second = value;
            return;
        }
        
        if (items_.size() >= capacity_) {
            auto last = items_.back();
            cache_.erase(last.first);
            items_.pop_back();
        }
        
        items_.push_front({key, value});
        cache_[key] = items_.begin();
    }
};
```

### Q3: 设计一个消息队列

```rust
// 简单的消息队列
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    
    // 生产者
    thread::spawn(move || {
        tx.send("message 1".to_string()).unwrap();
        tx.send("message 2".to_string()).unwrap();
    });
    
    // 消费者
    for msg in rx {
        println!("Received: {}", msg);
    }
}
```

---

## 4. 算法与数据结构

### 常见考点

| 题型 | 难度 | 频率 |
|------|------|------|
| 数组/字符串 | ⭐ | 极高 |
| 链表 | ⭐ | 高 |
| 二叉树 | ⭐⭐ | 高 |
| 动态规划 | ⭐⭐⭐ | 中 |
| 图算法 | ⭐⭐⭐ | 低 |
| 位运算 | ⭐⭐ | 中 |

### 必刷算法

1. **两数之和** - 哈希表
2. **反转链表** - 指针操作
3. **合并两个有序链表** - 归并
4. **最大子序和** - 动态规划
5. **反转二叉树** - 递归
6. **LRU 缓存** - 设计题
7. **生产者消费者** - 并发

---

## 5. 项目经验问题

### 如何描述项目

**STAR 法则：**
- **S**ituation - 背景
- **T**ask - 任务
- **A**ction - 行动
- **R**esult - 结果

### 项目描述模板

```
这个项目是一个[项目类型]，
主要解决了[问题/需求]。

我负责[具体模块]，
使用了[Rust/C++ 技术栈]。

技术挑战包括：
1. [挑战1] - 解决方案
2. [挑战2] - 解决方案

最终达到了[性能/效果]的提升。
```

### 常见追问

1. **为什么选择这个技术栈？**
   - 性能要求
   - 内存安全
   - 生态系统

2. **遇到过最困难的问题？**
   - 内存泄漏
   - 竞态条件
   - 性能瓶颈

3. **如何优化性能？**
   - Profiling 工具
   - 算法优化
   - 内存布局
   - 并发优化

---

## 6. 高频问题汇总

### Rust

| 问题 | 答案要点 |
|------|----------|
| 所有权是什么？ | 唯一所有者、移动语义、自动释放 |
| 借用规则？ | 不可变/可变不能共存 |
| 生命周期作用？ | 防止悬挂引用 |
| trait object？ | 动态分发、有运行时开销 |
| 异步编程？ | async/await、Future、tokio |
| unsafe 何时用？ | FFI、性能优化 |

### C++

| 问题 | 答案要点 |
|------|----------|
| vector 扩容？ | 1.5-2倍、重新分配、移动 |
| 智能指针？ | unique/shared/weak |
| RAII？ | 构造获取、析构释放 |
| 移动语义？ | 转移所有权、避免拷贝 |
| 虚函数？ | 动态绑定、vtable |
| 线程安全？ | mutex/atomic |

---

## 面试技巧

### 代码题

1. **先思考** - 2-3 分钟理解题意
2. **先说思路** - 再开始写代码
3. **边写边说** - 解释你的代码
4. **测试用例** - 考虑边界情况

### 系统设计

1. **澄清需求** - 问清楚约束
2. **高层设计** - 架构思路
3. **深入细节** - 关键技术点
4. **优化扩展** - 可扩展性

### 项目介绍

1. **简洁明了** - 1-2 分钟
2. **突出重点** - 难点和解决方案
3. **数据支撑** - 性能提升具体数字
4. **展示深度** - 底层原理理解

---

## 推荐资源

### 刷题
- LeetCode（算法）
- 剑指 Offer（国内面试）

### Rust
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)

### C++
- [cppreference](https://en.cppreference.com/)
- [learncpp](https://www.learncpp.com/)

---

## 学习路线建议

### 第一阶段（1-2月）
- Rust/C++ 基础语法
- 智能指针、并发基础
- 完成 TaskCLI 项目

### 第二阶段（2-3月）
- 深入特性（生命周期、模板）
- 算法与数据结构
- 刷题 100+

### 第三阶段（1-2月）
- 系统设计
- 项目完善
- 面试准备

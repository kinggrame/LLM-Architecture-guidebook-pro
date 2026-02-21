# Rust 高级特性与系统编程 - 完整指南

> 🎯 目标：从初学者到能够进行系统级编程和开源贡献

---

## 目录

1. [unsafe Rust - 与底层交互](#1-unsafe-rust)
2. [宏编程 - 声明式和过程宏](#2-宏编程)
3. [生命周期高级应用](#3-生命周期高级应用)
4. [trait 高级特性](#4-trait-高级特性)
5. [异步编程深入](#5-异步编程深入)
6. [并发与内存模型](#6-并发与内存模型)
7. [FFI 与 C 互操作](#7-ffi-与-c-互操作)
8. [内存管理和性能优化](#8-内存管理和性能优化)

---

## 1. unsafe Rust

### 1.1 什么时候使用 unsafe

`unsafe` 关键字允许你绕过 Rust 的安全检查：

```rust
// 允许的操作：
// 1. 解引用裸指针
// 2. 调用 unsafe 函数
// 3. 访问或修改可变静态变量
// 4. 实现 unsafe trait
// 5. 访问 union 的字段

fn main() {
    // 裸指针：*const T 和 *mut T
    let mut num = 42;
    
    // 创建裸指针（不需要安全借用）
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    unsafe {
        // 解引用裸指针
        println!("r1 points to: {}", *r1);
        *r2 = 100;
        println!("num is now: {}", num);
    }
}
```

### 1.2 裸指针 vs 引用

```rust
fn main() {
    let num = 42;
    
    // 安全引用
    let safe_ref = &num;
    
    // 裸指针（可以在不同作用域间传递）
    let raw_ptr = &num as *const i32;
    
    // 引用在离开作用域后失效
    // println!("{}", safe_ref); // 编译错误
    
    // 裸指针仍然有效
    unsafe {
        println!("Raw ptr points to: {}", *raw_ptr);
    }
}
```

### 1.3 读取/修改静态变量

```rust
// 静态变量
static mut COUNTER: i32 = 0;
static READONLY: i32 = 42;

fn main() {
    // 读取不可变静态变量（安全）
    println!("READONLY: {}", READONLY);
    
    // 修改可变静态变量（需要 unsafe）
    unsafe {
        COUNTER += 1;
        println!("COUNTER: {}", COUNTER);
    }
}
```

### 1.4 unsafe trait

```rust
unsafe trait Dangerous {
    fn do_something(&self);
}

struct MyStruct;

unsafe impl Dangerous for MyStruct {
    fn do_something(&self) {
        println!("Doing something dangerous!");
    }
}

fn main() {
    let s = MyStruct;
    // 调用 unsafe trait 方法需要 unsafe 块
    unsafe {
        s.do_something();
    }
}
```

### 1.5 实战：实现链表

```rust
use std::boxed::Box;

// 链表节点
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }
    
    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.len += 1;
    }
    
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.value
        })
    }
    
    fn len(&self) -> usize {
        self.len
    }
}

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    
    list.push(1);
    list.push(2);
    list.push(3);
    
    println!("Length: {}", list.len()); // 3
    
    while let Some(v) = list.pop() {
        println!("Popped: {}", v);
    }
}
```

---

## 2. 宏编程

### 2.1 声明式宏（macro_rules!）

```rust
// 简单的 println! 简化版
macro_rules! my_println {
    ($($arg:tt)*) => {
        println!($($arg)*)
    };
}

// 更复杂的例子：创建向量
macro_rules! vec {
    ($($elem:expr),*) => {
        {
            let mut v = Vec::new();
            $(v.push($elem);)*
            v
        }
    };
    
    ($elem:expr; $n:expr) => {
        vec![0; $n].iter().map(|_| $elem).collect()
    };
}

fn main() {
    my_println!("Hello, {}", "World!");
    
    let v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
    
    let v2 = vec![42; 5];
    println!("{:?}", v2);
}
```

### 2.2 过程宏

#### 2.2.1 自定义派生（derive）

```rust
use serde::{Serialize, Deserialize};

// 使用 serde 的 derive
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    
    // 序列化
    let json = serde_json::to_string(&p).unwrap();
    println!("JSON: {}", json);
    
    // 反序列化
    let p2: Point = serde_json::from_str(&json).unwrap();
    println!("{:?}", p2);
}
```

#### 2.2.2 自定义属性宏

```rust
// 类似 actix-web 的 route 宏
#[route("/hello", method = "GET")]
fn hello() -> String {
    "Hello, World!".to_string()
}

// 宏的简单实现示例
#[rustfmt::skip]
macro_rules! route {
    ($path:expr, method = $method:ident) => {
        #[allow(non_camel_case_types)]
        struct $method;
        
        impl Method for $method {
            fn as_str(&self) -> &str {
                stringify!($method)
            }
        }
        
        fn __route() -> Route {
            Route::new($path, $method)
        }
    };
}
```

### 2.3 宏的卫生性

```rust
macro_rules! foo {
    ($i:ident) => {
        let $i = 42;
    };
}

macro_rules! bar {
    () => {
        // 这里创建的 x 和外部的 x 是不同的
        let x = 42;
    };
}

fn main() {
    foo!(x);
    // x 在这里可用
    println!("{}", x);
    
    // 使用 $crate 引用当前 crate
}
```

---

## 3. 生命周期高级应用

### 3.1 多个生命周期参数

```rust
fn longest_with_announcement<'a, 'b>(
    x: &'a str,
    y: &'b str,
    ann: &str,
) -> &'a str {
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = "hello";
    let result;
    {
        let s2 = "world!";
        result = longest_with_announcement(s1, s2, "comparing");
    }
    println!("Longest: {}", result);
}
```

### 3.2 生命周期在结构体中

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 返回的生命周期与 self 相同
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("Please attention: {}", announcement);
        self.part
    }
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

### 3.3 静态生命周期

```rust
// 字符串字面量有 'static 生命周期
let s: &'static str = "I live forever!";

// 常量也是 'static
const GREETING: &str = "Hello!";

fn print_greeting(greeting: &'static str) {
    println!("{}", greeting);
}

fn main() {
    print_greeting("Hello, World!");
}
```

### 3.4 生命周期省略规则

```rust
// 编译器自动推断生命周期
fn first_word(s: &str) -> &str {
    // 省略后等价于：
    // fn first_word<'a>(s: &'a str) -> &'a str
    s.split_whitespace().next().unwrap_or("")
}

struct Context<'a> {
    s: &'a str,
}

impl<'a> Context<'a> {
    fn process(&self) -> &str {
        // 省略后等价于：
        // fn process<'b>(&'b self) -> &'b str
        self.s
    }
}
```

---

## 4. trait 高级特性

### 4.1 trait object 和动态分发

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle {}x{}", self.width, self.height);
    }
}

// 动态分发的函数
fn draw_all(items: &[&dyn Drawable]) {
    for item in items {
        item.draw();
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rect = Rectangle { width: 10.0, height: 20.0 };
    
    // 静态分发（泛型）
    fn draw_static<T: Drawable>(item: &T) {
        item.draw();
    }
    
    draw_static(&circle);
    draw_static(&rect);
    
    // 动态分发
    draw_all(&[&circle, &rect]);
}
```

### 4.2 Trait Object 的内存布局

```rust
// trait object = 胖指针
// 包含：数据指针 + vtable 指针

trait Printable {
    fn format(&self) -> String;
}

struct Point { x: i32, y: i32 }

impl Printable for Point {
    fn format(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    
    // &Point 是 8 字节
    // &dyn Printable 是 16 字节（胖指针）
    
    let print: &dyn Printable = &p;
    println!("{}", print.format());
}
```

### 4.3 关联类型

```rust
trait Container {
    type Item;
    
    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn add(&mut self, item: Self::Item);
    fn len(&self) -> usize;
}

struct VecContainer {
    items: Vec<String>,
}

impl Container for VecContainer {
    type Item = String;
    
    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.items.get(index)
    }
    
    fn add(&mut self, item: Self::Item) {
        self.items.push(item);
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let mut c = VecContainer { items: Vec::new() };
    c.add("hello".to_string());
    c.add("world".to_string());
    println!("Len: {}", c.len());
}
```

### 4.4 默认泛型参数和trait bound

```rust
use std::fmt::{self, Display};

// 默认泛型参数
struct Pair<T, U = f64> {
    x: T,
    y: U,
}

impl<T, U> Pair<T, U> {
    fn new(x: T, y: U) -> Self {
        Pair { x, y }
    }
}

// 使用 Display trait bound
impl<T: Display, U: Display> Display for Pair<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p1 = Pair::new(1, 2.5);
    let p2 = Pair::new("hello", "world");
    
    println!("{}", p1);
    // println!("{}", p2); // 需要 Display
}
```

### 4.5 Supertrait

```rust
trait Outline: Display {
    fn outline(&self) {
        println!("{}", "*".repeat(50));
        println!("{}", self);
        println!("{}", "*".repeat(50));
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Outline for Point {}

fn main() {
    let p = Point { x: 10, y: 20 };
    p.outline();
}
```

---

## 5. 异步编程深入

### 5.1 async/await 基础

```rust
async fn fetch_data() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://httpbin.org/get").await?;
    let body = response.text().await?;
    Ok(body)
}

async fn example() {
    println!("Starting fetch...");
    
    let result = fetch_data().await;
    
    match result {
        Ok(data) => println!("Got data: {} chars", data.len()),
        Err(e) => println!("Error: {}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    example().await;
    Ok(())
}
```

### 5.2 并发执行

```rust
async fn fetch_url(url: &str) -> String {
    println!("Fetching: {}", url);
    // 模拟网络请求
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    format!("Data from {}", url)
}

#[tokio::main]
async fn main() {
    // join! - 并发执行多个 future
    let (result1, result2) = tokio::join!(
        fetch_url("https://example.com"),
        fetch_url("https://rust-lang.org")
    );
    
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
}
```

### 5.3 select! - 多路复用

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(10);
    let (tx2, mut rx2) = mpsc::channel(10);
    
    // 发送数据
    tokio::spawn(async move {
        tx1.send("from channel 1").await.ok();
    });
    
    tokio::spawn(async move {
        tx2.send("from channel 2").await.ok();
    });
    
    // 等待任何一个 channel 收到消息
    tokio::select! {
        Some(msg) = rx1.recv() => {
            println!("Received from channel 1: {}", msg);
        }
        Some(msg) = rx2.recv() => {
            println!("Received from channel 2: {}", msg);
        }
    }
}
```

### 5.4 Stream 处理

```rust
use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
    let stream = stream::iter(1..=5);
    
    tokio::pin!(stream);
    
    while let Some(value) = stream.next().await {
        println!("Received: {}", value);
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
```

---

## 6. 并发与内存模型

### 6.1 线程安全原语

```rust
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() {
    // Mutex - 互斥锁
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..3 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Counter: {}", *counter.lock().unwrap());
    
    // RwLock - 读写锁
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    
    // 读操作（多个并发）
    {
        let reader = data.read().unwrap();
        println!("Read: {:?}", *reader);
    }
    
    // 写操作（独占）
    {
        let mut writer = data.write().unwrap();
        writer.push(4);
    }
}
```

### 6.2 原子操作

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    
    let handles: Vec<_> = (0..3)
        .map(|_| {
            thread::spawn(|| {
                for _ in 0..1000 {
                    COUNTER.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Counter: {}", COUNTER.load(Ordering::Relaxed));
}
```

### 6.3 Send 和 Sync

```rust
// 线程安全的类型自动实现这些 trait

// Send: 可以在线程间传递所有权
// Sync: 可以在线程间共享引用

// 手动实现（不推荐，除非你知道自己在做什么）
use std::marker::PhantomData;

struct MyContainer<T> {
    data: T,
    _marker: PhantomData<*const ()>, // 不允许多线程共享
}

// impl<T: Send> Send for MyContainer<T> {}
// impl<T: Sync> Sync for MyContainer<T> {}

fn main() {
    // 基本类型都是 Send + Sync
    println!("i32 is Send: {}", std::mem::needs_drop::<i32>());
    
    // Rc 不是线程安全的
    // let rc = std::rc::Rc::new(42);
    // rc.clone(); // 不是 Send
}
```

---

## 7. FFI 与 C 互操作

### 7.1 从 Rust 调用 C

```rust
// build.rs
// fn main() {
//     cc::Build::new()
//         .file("src/c_code.c")
//         .compile("c_code");
// }

#[link(name = "c_code")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
    fn greet(name: *const c_char) -> *mut c_char;
}

use std::ffi::{CStr, CString};

fn main() {
    unsafe {
        let result = add(1, 2);
        println!("1 + 2 = {}", result);
        
        let name = CString::new("Rust").unwrap();
        let greeting = greet(name.as_ptr());
        println!("C said: {:?}", CStr::from_ptr(greeting));
    }
}
```

### 7.2 从 C 调用 Rust

```rust
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[no_mangle]
pub extern "C" fn distance(p1: *const Point, p2: *const Point) -> f64 {
    unsafe {
        let p1 = &*p1;
        let p2 = &*p2;
        ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
    }
}
```

### 7.3 包装 C 库

```rust
// 绑定生成的例子
use std::ffi::CStr;

#[link(name = "sqlite3")]
extern "C" {
    fn sqlite3_open(filename: *const c_char, ppDb: *mut *mut c_void) -> i32;
    fn sqlite3_close(db: *mut c_void) -> i32;
    fn sqlite3_errmsg(db: *mut c_void) -> *const c_char;
}

struct Connection {
    db: *mut c_void,
}

impl Connection {
    fn open(path: &str) -> Result<Connection, String> {
        let db = std::ptr::null_mut();
        let c_path = std::ffi::CString::new(path).unwrap();
        
        unsafe {
            let result = sqlite3_open(c_path.as_ptr(), &db);
            if result == 0 {
                Ok(Connection { db })
            } else {
                Err("Failed to open".to_string())
            }
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            sqlite3_close(self.db);
        }
    }
}
```

---

## 8. 内存管理和性能优化

### 8.1 栈 vs 堆分配

```rust
fn main() {
    // 栈分配（固定大小，在编译时确定）
    let x = 42;           // i32 是 Copy
    let y = x;            // 复制到栈上
    
    // 堆分配（动态大小）
    let s = String::from("hello"); // 堆分配
    let s2 = s;            // 移动，而不是复制
    
    // Vec<T> 栈上只存指针和长度
    let v = vec![1, 2, 3, 4, 5];
    
    println!("Stack: x={}, y={}", x, y);
    println!("Heap: {}", s2);
    println!("Vec: {:?}", v);
}
```

### 8.2 避免不必要的克隆

```rust
// 不好：多次克隆
fn process(data: String) {
    let s = data.clone(); // 克隆1
    let s2 = data.clone(); // 克隆2
}

// 好：使用引用
fn process_ref(data: &str) {
    let s = data;
}

// 好：明确所有权
fn process_owned(data: String) -> String {
    // 处理 data
    data // 返回所有权
}
```

### 8.3 预分配容量

```rust
fn main() {
    // 不预分配
    let mut v = Vec::new();
    for i in 0..1000 {
        v.push(i); // 可能多次重新分配
    }
    
    // 预分配
    let mut v = Vec::with_capacity(1000);
    for i in 0..1000 {
        v.push(i); // 只分配一次
    }
    
    println!("Capacity: {}", v.capacity());
}
```

### 8.4 使用 entry API

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    
    // 不好：多次查询
    if map.contains_key("key") {
        *map.get_mut("key").unwrap() += 1;
    } else {
        map.insert("key", 1);
    }
    
    // 好：使用 entry API
    *map.entry("key").or_insert(0) += 1;
    
    // entry API 还可以做更多复杂操作
    map.entry("key").and_modify(|v| *v += 1).or_insert(42);
}
```

---

## 面试题精选

### Q1: 解释 Rust 的所有权系统

```rust
// 关键点：
// 1. 每个值有唯一所有者
// 2. 所有者离开作用域时值被丢弃
// 3. 移动语义是默认行为
// 4. 借用检查器在编译时防止悬挂引用
```

### Q2: &T vs &mut T vs T

```rust
// &T: 不可变借用，多个可以并存
// &mut T: 可变借用，同时只能有一个
// T: 所有权，可以转移或克隆
```

### Q3: Box<T> vs Rc<T> vs Arc<T>

```rust
// Box<T>: 堆分配，独占所有权
// Rc<T>: 引用计数，单线程
// Arc<T>: 原子引用计数，多线程
```

### Q4: 解释 async/await

```rust
// async 函数返回 Future
// await 等待 Future 完成
// tokio 提供了运行时
```

---

## 练习项目

1. **实现 FFI 绑定**：为某个 C 库生成 Rust 绑定
2. **异步 HTTP 服务器**：使用 tokio 实现
3. **并发文件处理**：处理大量文件
4. **实现自己的容器**：如 BTreeMap

---

## 参考资源

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [Async Book](https://rust-lang.github.io/async-book/)

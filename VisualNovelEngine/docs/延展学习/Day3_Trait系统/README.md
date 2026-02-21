# Day 3: Trait 系统

> 🎯 **目标**：深入理解 Rust 的 trait 系统，掌握面向接口编程的核心技能

---

## 学习目标

1. 理解 trait 的概念和作用
2. 掌握 trait 定义和实现
3. 理解 trait object 和动态分发
4. 学会使用标准库常用 trait

---

## 代码 Demo

### Demo 1: 基本 Trait 定义

```rust
// src/trait_basic.rs

/// trait 定义行为规范
/// 类似其他语言的"接口"

trait Printable {
    fn print(&self);
}

struct Point {
    x: i32,
    y: i32,
}

impl Printable for Point {
    fn print(&self) {
        println!("Point({}, {})", self.x, self.y);
    }
}

struct Person {
    name: String,
    age: u32,
}

impl Printable for Person {
    fn print(&self) {
        println!("Person(name: {}, age: {})", self.name, self.age);
    }
}

fn main() {
    println!("=== Demo 1: 基本 Trait ===");
    
    let point = Point { x: 10, y: 20 };
    let person = Person { name: String::from("Alice"), age: 30 };
    
    point.print();
    person.print();
    
    println!("\n=== Demo 2: Trait 作为函数参数 ===");
    
    print_something(&point);
    print_something(&person);
    
    println!("\n=== Demo 3: Trait 返回类型 ===");
    
    let p = create_point(5, 6);
    p.print();
}

fn print_something(p: &impl Printable) {
    p.print();
}
```

---

### Demo 2: Trait 默认实现

```rust
// src/trait_default.rs

/// trait 可以有默认实现

trait Describable {
    fn describe(&self) -> String {
        String::from("这是一个对象")
    }
    
    fn describe_detailed(&self) -> String {
        format!("{}", self.describe())
    }
}

struct Animal {
    name: String,
    species: String,
}

impl Describable for Animal {
    fn describe(&self) -> String {
        format!("{} 是 {}", self.name, self.species)
    }
}

struct Thing {
    name: String,
}

impl Describable for Thing {}

fn main() {
    println!("=== Demo: Trait 默认实现 ===");
    
    let dog = Animal { 
        name: String::from("旺财"), 
        species: String::from("狗") 
    };
    
    let box_ = Thing { 
        name: String::from("盒子") 
    };
    
    println!("{}", dog.describe());
    println!("{}", dog.describe_detailed());
    
    println!("\n使用默认实现:");
    println!("{}", box_.describe());
    println!("{}", box_.describe_detailed());
}
```

---

### Demo 3: Trait Object

```rust
// src/trait_object.rs

/// trait object 实现动态分发

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
        println!("绘制圆形，半径: {}", self.radius);
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("绘制矩形: {} x {}", self.width, self.height);
    }
}

// 使用 trait object
fn draw_all(items: &[&dyn Drawable]) {
    for item in items {
        item.draw();
    }
}

// 使用泛型（静态分发）
fn draw_all_generic<T: Drawable>(items: &[T]) {
    for item in items {
        item.draw();
    }
}

fn main() {
    println!("=== Demo: Trait Object ===");
    
    let circle = Circle { radius: 5.0 };
    let rect = Rectangle { width: 10.0, height: 20.0 };
    
    // 动态分发
    println!("动态分发:");
    draw_all(&[&circle, &rect]);
    
    // 静态分发
    println!("\n静态分发:");
    draw_all_generic(&[&circle, &rect]);
    
    println!("\n=== 性能对比 ===");
    println!("dyn Trait: 动态分派，有运行时开销");
    println!("泛型 T: 静态分派，无运行时开销");
}
```

---

### Demo 4: 标准库常用 Trait

```rust
// src/standard_traits.rs

/// Rust 标准库常用 trait

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 自定义类型实现常用 trait
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

use std::ops::Add;
use std::fmt;

fn main() {
    println!("=== Demo: 标准库常用 Trait ===");
    
    // Debug
    let p1 = Point::new(1, 2);
    println!("Debug: {:?}", p1);
    
    // Display
    println!("Display: {}", p1);
    
    // Default
    let p2 = Point::default();
    println!("Default: {}", p2);
    
    // Clone 和 Copy
    let p3 = p1;
    println!("Copy: p1 = {}, p3 = {}", p1.x, p3.x);
    let mut p4 = p1.clone();
    p4.x = 10;
    println!("Clone: p4 = {}", p4);
    
    // PartialEq
    let p5 = Point::new(1, 2);
    println!("PartialEq: {} == {} = {}", p1, p5, p1 == p5);
    
    // Add
    let p6 = p1 + p5;
    println!("Add: {} + {} = {}", p1, p5, p6);
}
```

---

### Demo 5: 关联类型

```rust
// src/associated_types.rs

/// 关联类型在 trait 定义中使用

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
    println!("=== Demo: 关联类型 ===");
    
    let mut container = VecContainer { items: Vec::new() };
    
    container.add(String::from("hello"));
    container.add(String::from("world"));
    
    println!("长度: {}", container.len());
    println!("索引 0: {:?}", container.get(0));
    println!("索引 1: {:?}", container.get(1));
    println!("索引 2: {:?}", container.get(2));
}
```

---

## 核心关键词

### Trait 基础
- **Trait**: 定义行为规范的接口
- **Implement**: 为类型实现 trait
- **Trait Bound**: 泛型类型必须满足的约束

### Trait 进阶
- **Default Implementation**: 默认实现，可以被覆盖
- **Trait Object (dyn Trait)**: 动态分发的 trait 引用
- **Static Dispatch**: 静态分发，编译时确定
- **Dynamic Dispatch**: 动态分发，运行时确定

### 标准库 Trait
- **Debug**: 调试格式化输出
- **Display**: 用户格式化输出
- **Clone**: 深拷贝
- **Copy**: 浅拷贝（位复制）
- **Default**: 默认值
- **PartialEq/ Eq**: 相等性比较
- **PartialOrd/ Ord**: 顺序比较
- **Add/ Sub/ Mul/ Div**: 算术运算

---

## 练习题

1. 为 `Point` 实现 `Sub`、`Mul` trait
2. 使用 trait object 实现一个动物"叫声"系统
3. 理解 `&dyn Trait` 和 `impl Trait` 的区别

---

## 下一天预告

**Day 4: 泛型编程**

- 泛型函数和泛型结构体
- trait bound 和 where 子句
- 泛型代码的编译模型

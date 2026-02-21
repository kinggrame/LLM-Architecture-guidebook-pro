# Day 4: 泛型编程

> 🎯 **目标**：掌握 Rust 的泛型编程，实现类型无关的通用代码

---

## 代码 Demo

### Demo 1: 泛型函数

```rust
// 泛型函数
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("最大数: {}", result);
    
    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("最大字符: {}", result);
}
```

### Demo 2: 泛型结构体

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

### Demo 3: Trait Bound

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("最大的值是 x = {}", self.x);
        } else {
            println!("最大的值是 y = {}", self.y);
        }
    }
}

fn main() {
    let pair = Pair::new(5, 10);
    pair.cmp_display();
}
```

---

## 关键词

- **Generic (泛型)**: 类型参数化
- **Type Parameter (类型参数)**: 泛型函数/结构体的参数
- **Trait Bound**: 泛型的约束条件
- **Where Clause**: 更清晰的约束语法

---

## 下一天预告

**Day 5: 智能指针**

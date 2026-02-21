# Day 5: 智能指针

> 🎯 **目标**：掌握 Rust 的智能指针，实现灵活的内存管理

---

## 代码 Demo

### Demo 1: Box<T> - 堆分配

```rust
fn main() {
    // Box 在堆上分配数据
    let b = Box::new(5);
    println!("Box value: {}", b);
    
    // Box 用于递归类型
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
}
```

### Demo 2: Rc<T> - 引用计数

```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(vec![1, 2, 3]);
    println!("初始引用计数: {}", Rc::strong_count(&data));
    
    let a = Rc::clone(&data);
    println!("克隆后计数: {}", Rc::strong_count(&data));
    
    {
        let b = Rc::clone(&data);
        println!("在作用域内计数: {}", Rc::strong_count(&data));
    }
    
    println!("离开作用域后计数: {}", Rc::strong_count(&data));
}
```

### Demo 3: RefCell<T> - 内部可变性

```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(42);
    
    // 不可变借用
    let r1 = x.borrow();
    println!("不可变借用: {}", r1);
    
    // 可变借用
    *x.borrow_mut() = 100;
    println!("可变借用后: {}", x.borrow());
}
```

### Demo 4: Arc<T> + Mutex<T> - 线程安全

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut num = data.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终结果: {}", *data.lock().unwrap());
}
```

---

## 关键词

- **Box<T>**: 堆分配指针
- **Rc<T>**: 引用计数（单线程）
- **Arc<T>**: 原子引用计数（多线程）
- **RefCell<T>**: 内部可变性
- **Mutex<T>**: 互斥锁

---

## 下一天预告

**Day 6: 错误处理**

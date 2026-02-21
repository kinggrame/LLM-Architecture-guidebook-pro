# Day 7: 并发编程

> 🎯 **目标**：掌握 Rust 的多线程编程

---

## 代码 Demo

### Demo 1: 基础线程

```rust
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Demo 1: 创建线程 ===");
    
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("线程: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for i in 1..5 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_millis(100));
    }
    
    handle.join().unwrap();
    println!("线程完成");
}
```

### Demo 2: 线程间共享数据

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("=== Demo 2: Mutex 共享数据 ===");
    
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
    
    println!("最终计数: {}", *counter.lock().unwrap());
}
```

### Demo 3: Channels

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    println!("=== Demo 3: 消息传递 ===");
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        tx.send("Hello from thread!").unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("收到: {}", received);
}
```

---

## 关键词

- **thread::spawn**: 创建新线程
- **Mutex<T>**: 互斥锁
- **Arc<T>**: 原子引用计数
- **Channel**: 消息传递

---

## 下一天预告

**Day 8: unsafe Rust**

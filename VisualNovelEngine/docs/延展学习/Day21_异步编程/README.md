# Day 21: Rust 异步编程

> 🎯 **目标**：掌握 Rust 的 async/await 异步编程模型

---

## 代码 Demo

### Demo 1: async/await 基础

```rust
async fn fetch_data() -> String {
    // 模拟异步操作
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    "数据获取完成".to_string()
}

async fn example() {
    println!("开始异步操作...");
    
    let result = fetch_data().await;
    println!("结果: {}", result);
}

#[tokio::main]
async fn main() {
    example().await;
}
```

### Demo 2: 并发执行

```rust
async fn fetch_url(url: &str) -> String {
    println!("正在获取: {}", url);
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    format!("{} 的内容", url)
}

#[tokio::main]
async fn main() {
    println!("=== Demo: 并发执行 ===");
    
    // 使用 join! 并发执行多个异步任务
    let (result1, result2) = tokio::join!(
        fetch_url("https://example.com"),
        fetch_url("https://rust-lang.org")
    );
    
    println!("结果1: {}", result1);
    println!("结果2: {}", result2);
}
```

### Demo 3: Streams

```rust
async fn process_stream() {
    let stream = tokio_stream::iter(1..=5);
    
    tokio::pin!(stream);
    
    while let Some(value) = stream.next().await {
        println!("处理: {}", value);
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
```

---

## 关键词

- **async/await**: 异步编程语法
- **Future**: 代表未来某个时刻的值
- **tokio**: 异步运行时
- **join!**: 并发执行多个 Future
- **select!**: 多路复用多个 Future

---

## 下一天预告

**Day 22: 嵌入式 Rust 开发**

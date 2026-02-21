# Day 6: 错误处理

> 🎯 **目标**：掌握 Rust 的错误处理模式

---

## 代码 Demo

### Demo 1: Result 类型

```rust
use std::fs::File;
use std::io::{self, Read};

fn main() {
    // 使用 Result
    let result = read_file("test.txt");
    
    match result {
        Ok(contents) => println!("文件内容: {}", contents),
        Err(e) => println!("错误: {}", e),
    }
    
    // 使用 ? 操作符
    let content = read_file_simple("test.txt").unwrap_or_default();
    println!("内容: {}", content);
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_file_simple(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

### Demo 2: 自定义错误类型

```rust
use std::fmt;

#[derive(Debug)]
enum MyError {
    IoError(std::io::Error),
    ParseError(String),
    Custom(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::IoError(e) => write!(f, "IO错误: {}", e),
            MyError::ParseError(s) => write!(f, "解析错误: {}", s),
            MyError::Custom(s) => write!(f, "错误: {}", s),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::IoError(err)
    }
}

fn main() {
    let result: Result<i32, MyError> = Ok(42);
    println!("{:?}", result);
}
```

---

## 关键词

- **Result<T, E>**: 可恢复错误的类型
- **? 操作符**: 错误传播语法糖
- **panic!**: 不可恢复的错误
- **unwrap()**: 获取值或 panic

---

## 下一天预告

**Day 7: 并发编程**

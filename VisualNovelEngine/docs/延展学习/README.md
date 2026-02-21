# 🚀 Rust & C++ 系统编程 - 30天延展学习计划

> 📚 本计划专为想要掌握 Rust 系统级编程和 C++ 的开发者设计
> 
> 🎯 **目标**：从基础到高级，系统掌握两门语言的核心技能

---

## 📖 学习路径总览

```
┌─────────────────────────────────────────────────────────────────┐
│                     30天学习路线图                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Day 1-10:  Rust 核心技能                                      │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐          │
│  │ 所有权   │  │ 生命周期 │  │ Trait   │  │ 泛型    │  ...    │
│  │ 深入理解 │  │ 标注    │  │ 系统    │  │ 编程    │          │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘          │
│       │             │             │             │               │
│       ▼             ▼             ▼             ▼               │
│  Day 11-20:  C++ 核心技能                                      │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐          │
│  │ 指针    │  │ 内存    │  │ 模板    │  │ RAII    │  ...    │
│  │ 引用    │  │ 管理    │  │ 元编程  │  │ 惯用法  │          │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘          │
│       │             │             │             │               │
│       ▼             ▼             ▼             ▼               │
│  Day 21-30:  高级主题与实战                                     │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐          │
│  │ 并发    │  │ 异步    │  │ 嵌入式  │  │ 项目    │          │
│  │ 编程    │  │ 编程    │  │ 开发    │  │ 实战    │          │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘          │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 📅 详细学习计划

### 第一阶段：Rust 核心技能（Day 1-10）

| 天数 | 主题 | 核心技能 | 项目类型 |
|:----:|------|----------|----------|
| Day 1 | 内存安全基础 | 所有权、借用、生命周期 | 内存管理演示 |
| Day 2 | 生命周期进阶 | 'a 标注、结构体生命周期 | 引用有效性检查器 |
| Day 3 | Trait 系统 |  trait 定义、默认实现、trait object | 插件系统框架 |
| Day 4 | 泛型编程 | 泛型函数、泛型结构体、trait bound | 通用数据结构 |
| Day 5 | 智能指针 | Box、Rc、Arc、Cell、RefCell | 引用计数容器 |
| Day 6 | 错误处理 | Result、?操作符、panic、自定义错误 | 错误处理框架 |
| Day 7 | 并发基础 | thread、Mutex、Channels | 多线程下载器 |
| Day 8 | unsafe Rust | unsafe 块、裸指针、FFI | 绑定 C 库 |
| Day 9 | 宏系统 | 声明宏、过程宏 | 代码生成器 |
| Day 10 | 异步编程 | async/await、Future、Pin | 异步 HTTP 服务器 |

### 第二阶段：C++ 核心技能（Day 11-20）

| 天数 | 主题 | 核心技能 | 项目类型 |
|:----:|------|----------|----------|
| Day 11 | 指针与引用 | 指针运算、引用、this 指针 | 链表实现 |
| Day 12 | 内存管理 | new/delete、RAII、智能指针 | 文件资源管理 |
| Day 13 | 类的进阶 | 构造/析构、拷贝语义、移动语义 | 动态数组类 |
| Day 14 | 模板编程 | 函数模板、类模板、SFINAE | 通用排序器 |
| Day 15 | STL 容器 | vector、map、set、algorithm | 文本搜索引擎 |
| Day 16 | 继承与多态 | 虚函数、抽象类、接口 | 图形渲染器 |
| Day 17 | C++11/14/17 | auto、lambda、optional、variant | 现代 C++ 重构 |
| Day 18 | 并发编程 | thread、mutex、future、atomic | 线程池实现 |
| Day 19 | 模板元编程 | type traits、constexpr、概念 | 类型推导库 |
| Day 20 | 内存模型 | 原子操作、内存顺序、锁-free | 无锁数据结构 |

### 第三阶段：高级主题与实战（Day 21-30）

| 天数 | 主题 | 核心技能 | 项目类型 |
|:----:|------|----------|----------|
| Day 21 | Rust 异步运行时 | tokio、select!、Stream | 异步 Web 服务器 |
| Day 22 | 嵌入式 Rust | no_std、embedded-hal、ESP32 | LED 闪烁控制 |
| Day 23 | 网络编程 | TCP/UDP、HTTP、WebSocket | 简单聊天服务器 |
| Day 24 | 数据库 | SQLite、连接池、ORM | 简易 KV 数据库 |
| Day 25 | FFI 互操作 | Rust 调用 C、C 调用 Rust | 性能关键模块 |
| Day 26 | 游戏引擎架构 | ECS、组件系统 | 迷你游戏引擎 |
| Day 27 | 编译器基础 | 词法分析、语法分析 | 表达式计算器 |
| Day 28 | 操作系统原理 | 系统调用、进程、调度器 | 简易 Shell |
| Day 29 | 网络协议栈 | TCP/IP、UDP、HTTP | HTTP 服务器 |
| Day 30 | 综合项目 | 综合运用 | 完整项目实现 |

---

## 🎯 每日学习结构

每个 Demo 包含以下内容：

```
Day_X/
├── README.md           # 学习目标和大纲
├── src/
│   ├── main.rs         # 主代码
│   └── lib.rs          # 库代码（如需要）
├── tests/              # 测试代码
├── examples/          # 示例代码
└── docs/
    ├── 笔记.md         # 学习笔记
    └── 关键词.md       # 核心关键词
```

---

## 🛠️ 环境准备

### Rust 环境

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 验证安装
rustc --version
cargo --version

# 安装常用工具
rustup component add rustfmt clippy rust-src
cargo install cargo-watch cargo-bloat
```

### C++ 环境

```bash
# Linux (Ubuntu)
sudo apt install build-essential g++ cmake gdb

# macOS
xcode-select --install

# Windows
# 下载 Visual Studio Build Tools 或 MinGW-w64
```

---

## 📚 学习建议

### 学习方法

1. **先读后写**：先理解概念，再动手编码
2. **对比学习**：对照 Rust 和 C++ 的实现方式
3. **动手实践**：每个 Demo 都要亲自运行和修改
4. **举一反三**：尝试用另一种语言实现相同功能
5. **总结输出**：每学完一个主题，写学习笔记

### 调试技巧

- **Rust**: 使用 `cargo check`、`cargo clippy`、`rust-analyzer`
- **C++**: 使用 `clang-tidy`、`cmake --build . --target analyze`

---

## 🔗 相关资源

### Rust 资源

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)

### C++ 资源

- [cppreference](https://en.cppreference.com/)
- [learncpp](https://www.learncpp.com/)
- [Effective Modern C++](https://www.oreilly.com/library/view/effective-modern-c/9781491908419/)
- [C++ Core Guidelines](https://isocpp.github.io/CppCoreGuidelines/)

---

## 📝 考核标准

完成本计划后，你应该能够：

- [ ] 熟练使用 Rust 和 C++ 进行系统级编程
- [ ] 理解内存管理和并发编程的核心概念
- [ ] 能够阅读和理解大型开源项目源码
- [ ] 具备独立实现复杂系统的能力
- [ ] 能够进行 Rust 和 C++ 的 FFI 互操作

---

## 🚀 开始学习

选择你感兴趣的章节开始学习：

- [Day 1: Rust 所有权深入理解](./Day1_内存安全基础/README.md)
- [Day 11: C++ 指针与引用](./Day11_指针与引用/README.md)

---

*祝学习愉快！ 🎉*

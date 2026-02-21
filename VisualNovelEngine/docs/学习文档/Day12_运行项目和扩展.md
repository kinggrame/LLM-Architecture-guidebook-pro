# 第12天：运行项目与扩展指南

> 🚀 **目标**：学会如何编译运行项目，以及如何扩展功能。

---

## 12.1 环境准备

### 12.1.1 安装 Rust

**Windows:**
```powershell
# 使用 winget
winget install Rustlang.Rust.MSVC

# 或者下载安装包
# https://rustup.rs
```

**验证安装：**
```bash
rustc --version
cargo --version
```

### 12.1.2 项目依赖（Rust 版本）

根据 Cargo.toml，需要安装：
- **wgpu** - 图形渲染（需要显卡驱动）
- **winit** - 窗口管理
- **rodio** - 音频播放
- **image** - 图片加载
- **parking_lot** - 并发控制

> 💡 首次运行 `cargo build` 时会自动下载所有依赖

---

## 12.2 运行项目

### 12.2.1 构建 Rust 版本

```bash
cd VisualNovelEngine/rust

# 开发模式构建（快）
cargo build

# 发布模式构建（更小、更快）
cargo build --release

# 运行
cargo run
```

### 12.2.2 运行输出示例

```
INFO  visual_novel_engine::core::engine > ============================================
INFO  visual_novel_engine::core::engine >   Visual Novel Engine - Rust Version
INFO  visual_novel_engine::core::engine > ============================================
INFO  visual_novel_engine::core::engine > Initializing Visual Novel Engine...
INFO  visual_novel_engine::core::engine > ResourceManager initialized
INFO  visual_novel_engine::core::engine > Engine initialized successfully
INFO  visual_novel_engine::core::engine > Script loaded successfully
```

---

## 12.3 C++ 版本构建

### 12.3.1 安装依赖

**Windows (使用 vcpkg):**
```bash
# 安装 SFML
vcpkg install sfml:x64-windows
```

### 12.3.2 构建

```bash
cd VisualNovelEngine/cpp

# 创建构建目录
mkdir build
cd build

# 配置
cmake .. -DCMAKE_TOOLCHAIN_FILE=...

# 编译
cmake --build .

# 运行
./VisualNovelEngine
```

---

## 12.4 扩展功能指南

### 12.4.1 添加新脚本命令

如果你想添加一个新命令 `@fade`（淡入淡出效果）：

**步骤1：在 CommandType 中添加枚举**

```rust
// rust/src/script/command.rs

pub enum CommandType {
    // ... 现有命令
    Fade,  // 新增
}
```

**步骤2：解析命令**

```rust
// 在解析器中添加
"fade" => Command::new(CommandType::Fade, vec![param.to_string()]),
```

**步骤3：注册处理器**

```rust
// main.rs

se.set_command_handler(CommandType::Fade, |cmd| {
    // 实现淡入淡出效果
    let duration = cmd.params.first().unwrap_or(&"1.0".to_string());
    println!("淡入淡出效果: {}秒", duration);
});
```

### 12.4.2 添加新场景

添加一个"设置菜单"场景：

```rust
// rust/src/scene/settings.rs

pub struct SettingsScene {
    // 设置项
    volume: f32,
    // ...
}

impl Scene for SettingsScene {
    fn on_enter(&mut self) {
        println!("进入设置画面");
    }
    
    fn update(&mut self, delta_time: f32) {
        // 处理按键
    }
    
    fn render(&mut self) {
        // 绘制设置界面
    }
    
    fn on_exit(&mut self) {
        println!("退出设置画面");
    }
}
```

**注册场景：**
```rust
let settings = SettingsScene::new();
sm.register_scene("settings", settings);
```

### 12.4.3 添加新资源类型

添加字体支持：

```rust
// rust/src/resources/font.rs

pub struct FontManager {
    fonts: HashMap<String, Font>,
}

impl FontManager {
    pub fn load_font(&mut self, path: &str) -> Option<&Font> {
        // 加载字体文件
    }
}
```

---

## 12.5 调试技巧

### 12.5.1 日志输出

```rust
// 添加日志
log::info!("玩家选择了选项: {}", choice);
log::warn!("资源加载失败: {}", path);
log::error!("脚本解析错误: {}", error);
```

### 12.5.2 使用 cargo check

```bash
# 快速检查代码错误（不编译）
cargo check

# 检查并显示所有警告
cargo check --all-targets -- -W warnings
```

### 12.5.3 使用 clippy（代码风格检查）

```bash
cargo clippy
```

---

## 12.6 学习建议

### 12.6.1 后续深入方向

1. **图形编程**
   - 学习 wgpu 或 OpenGL
   - 理解渲染管线

2. **音频系统**
   - 学习音频混合
   - 实现淡入淡出

3. **UI 系统**
   - 实现按钮、滑动条
   - 响应式布局

4. **存档系统**
   - JSON 序列化
   - 加密存档

5. **动画系统**
   - 补间动画
   - 粒子效果

### 12.6.2 推荐学习资源

**Rust 语言：**
- 《The Rust Book》（官方教程）
- Rust By Example

**游戏引擎：**
- Learn OpenGL（learnopengl.com）
- wgpu 官方文档

**视觉小说：**
- Ren'Py（参考实现）
- Ink（脚本语言）

---

## 12.7 项目改进建议

### 12.7.1 当前待实现功能

根据 README.md：
- [ ] 完整图形渲染
- [ ] UI系统（文本框、按钮）
- [ ] 完整的选择支系统
- [ ] 存档/读档功能
- [ ] 动画和过渡效果
- [ ] 字体渲染
- [ ] 多语言支持

### 12.7.2 可以尝试的改进

1. **完善渲染系统**
   - 实现真正的窗口显示
   - 添加图片绘制

2. **完善音频系统**
   - 实现背景音乐循环
   - 添加音效

3. **完善脚本系统**
   - 实现选择支功能
   - 实现变量系统

---

## 12.8 恭喜完成学习！

### 你已经学到了：

```
✅ 视觉小说引擎概念
✅ 项目结构分析
✅ Rust 基础语法
✅ 所有权和借用
✅ 结构体和方法
✅ 模块系统
✅ 脚本系统原理
✅ 场景管理系统
✅ 资源管理系统
✅ 渲染系统基础
✅ 引擎整合理解
✅ 运行和扩展
```

### 下一步：

1. 尝试运行项目
2. 修改示例脚本
3. 添加新功能
4. 深入学习某个子系统

---

## 12.9 学习导航

如果你想继续深入，可以按以下顺序学习：

1. **巩固 Rust** - 继续学习生命周期、trait、并发
2. **图形渲染** - 学习 wgpu 库
3. **游戏开发** - 学习 ECS 架构
4. **视觉小说** - 研究 Ren'Py 的实现

祝你在游戏开发的道路上学习愉快！ 🎮

# 第10天：渲染系统 - 在屏幕上绘制画面

> 🎨 **目标**：理解游戏如何在屏幕上显示图片、角色和UI。

---

## 10.1 什么是渲染？

**渲染（Rendering）** 就是把数据转换为可见图像的过程。

对于视觉小说引擎：
- 输入：图片数据 + 位置 + 效果
- 输出：屏幕上的像素

---

## 10.2 渲染的基本流程

```
┌─────────────────┐
│  1. 清空屏幕   │  ← 擦除上一帧
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  2. 绘制背景    │  ← 最底层
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  3. 绘制角色    │  ← 在背景之上
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  4. 绘制UI      │  ← 最顶层（对话框等）
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  5. 显示帧缓冲  │  ← 呈现到屏幕
└─────────────────┘
```

### 层次顺序（Z-order）：
```
最顶层：UI（对话框、按钮、选择菜单）
中间：角色立绘
底层：背景图片
```

---

## 10.3 图形 API 简介

### 常见的图形 API：

| API | 说明 | 特点 |
|-----|------|------|
| **OpenGL** | 老牌跨平台 | 兼容性好 |
| **Vulkan** | OpenGL 继任者 | 性能强，复杂 |
| **DirectX** | Windows 专用 | 游戏主流 |
| **wgpu** | Rust 图形库 | 安全、现代 |

项目中 Rust 版本使用 **wgpu**，C++ 版本使用 **SFML**（基于 OpenGL）。

---

## 10.4 渲染引擎结构

```rust
// rust/src/rendering/mod.rs

pub struct RenderEngine {
    width: u32,
    height: u32,
    window: Option<Window>,
    // 渲染相关状态
}

impl RenderEngine {
    pub fn new() -> Self {
        Self {
            width: 1280,
            height: 720,
            window: None,
        }
    }
    
    // 初始化
    pub fn initialize(&mut self, width: u32, height: u32, title: &str, vsync: bool) -> Result<()> {
        // 创建窗口
        self.window = Some(Window::new(...));
        Ok(())
    }
    
    // 清空屏幕
    pub fn clear(&mut self) {
        // 设置背景色
    }
    
    // 绘制图片
    pub fn draw_texture(&mut self, texture: &Texture, x: f32, y: f32) {
        // 在指定位置绘制纹理
    }
    
    // 呈现帧
    pub fn present(&mut self) -> Result<()> {
        // 将渲染结果显示到屏幕
        Ok(())
    }
    
    // 关闭
    pub fn shutdown(&mut self) {
        self.window = None;
    }
}
```

---

## 10.5 绘制流程示例

### 10.5.1 绘制背景

```rust
fn draw_background(render_engine: &mut RenderEngine, bg: &Texture) {
    // 绘制到整个屏幕
    render_engine.draw_texture(bg, 0.0, 0.0);
}
```

### 10.5.2 绘制角色

```rust
fn draw_character(render_engine: &mut RenderEngine, char: &Character) {
    // 绘制到指定位置
    let position = match char.position {
        Position::Left => (100.0, 200.0),
        Position::Center => (500.0, 200.0),
        Position::Right => (900.0, 200.0),
    };
    
    render_engine.draw_texture(&char.texture, position.0, position.1);
}
```

### 10.5.3 绘制对话框

```rust
fn draw_dialog_box(render_engine: &mut RenderEngine, dialog: &DialogBox) {
    // 绘制对话框背景
    render_engine.draw_texture(&dialog.background, 50.0, 500.0);
    
    // 绘制说话者名字
    render_engine.draw_text(&dialog.speaker_name, 70.0, 510.0);
    
    // 绘制对话内容
    render_engine.draw_text(&dialog.content, 70.0, 550.0);
}
```

---

## 10.6 游戏循环（Game Loop）

游戏的核心是一个**无限循环**，每秒钟运行60次（60 FPS）：

```rust
fn main() {
    let mut engine = Engine::new();
    engine.initialize(config);
    
    // 游戏主循环
    while engine.is_running() {
        // 1. 计算两帧之间的时间（delta time）
        let delta_time = get_delta_time();
        
        // 2. 处理输入（键盘、鼠标）
        handle_input();
        
        // 3. 更新游戏状态
        engine.update(delta_time);
        
        // 4. 渲染画面
        engine.render();
        
        // 5. 等待垂直同步（可选）
        wait_for_vsync();
    }
    
    engine.shutdown();
}
```

### 每帧执行顺序：

```
┌────────────────────────────────────────────────────┐
│                    帧 #1 开始                       │
├────────────────────────────────────────────────────┤
│ 1. 处理输入   │  检查玩家按下了什么                │
├───────────────┼───────────────────────────────────┤
│ 2. 更新逻辑   │  移动角色、检测碰撞、执行脚本     │
├───────────────┼───────────────────────────────────┤
│ 3. 渲染画面   │  绘制背景、角色、UI               │
├───────────────┼───────────────────────────────────┤
│ 4. 呈现帧     │  显示到屏幕（等待 v-sync）        │
└───────────────┴───────────────────────────────────┘
                      │
                      ▼
                 帧 #2 开始
```

---

## 10.7 项目中的渲染实现

### 10.7.1 渲染引擎初始化

```rust
// rust/src/core/engine.rs

pub fn initialize(&mut self, config: GameConfig) -> Result<()> {
    // 初始化渲染引擎
    {
        let mut re = self.render_engine.lock();
        re.initialize(
            self.config.width,
            self.config.height,
            &self.config.title,
            self.config.vsync,
        )?;
    }
    Ok(())
}
```

### 10.7.2 渲染流程

```rust
// 渲染一帧
fn render(&mut self) -> Result<()> {
    // 1. 清空屏幕
    self.render_engine.lock().clear();
    
    // 2. 让当前场景渲染
    self.scene_manager.lock().render();
    
    // 3. 呈现到屏幕
    self.render_engine.lock().present()?;
    
    Ok(())
}
```

---

## 10.8 坐标系

在 2D 游戏中，通常使用**笛卡尔坐标系**：

```
屏幕坐标系（左上角为原点）：
┌─────────────────────────────► X (增加向右)
│ (0,0)     640     1280
│  ┌────────┼────────┐
│  │        │        │
│  │   左   │   右   │
│  │        │        │
720┼────────┼────────┤
│  │        │        │
│  │        │        │
│  └────────┼────────┘
│
▼ Y (增加向下)
```

---

## 10.9 今日总结

今天我们学习了：
- ✅ **渲染的概念** - 数据如何变成图像
- ✅ **渲染层次** - 背景→角色→UI
- ✅ **图形 API** - wgpu、OpenGL
- ✅ **RenderEngine** - 渲染引擎结构
- ✅ **游戏循环** - 60 FPS 的更新机制
- ✅ **坐标系** - 屏幕坐标

---

## 10.10 练习题

1. 如果要在角色背后添加阴影，应该在哪一层绘制？
2. 为什么游戏通常需要等待垂直同步（vsync）？
3. 如果游戏卡顿（掉帧），应该优先优化哪个环节？

---

## 10.11 明日预告

明天我们将学习：
- **引擎整合** - 所有子系统如何协同工作
- **完整流程** - 从启动到退出的全过程
- **数据流向** - 脚本→场景→渲染

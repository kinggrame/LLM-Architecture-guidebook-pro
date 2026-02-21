# 第11天：引擎整合 - 理解整体架构

> 🔗 **目标**：理解所有子系统如何协同工作，形成完整的游戏引擎。

---

## 11.1 系统架构总览

让我们从宏观角度看看 Visual Novel Engine 的架构：

```
┌─────────────────────────────────────────────────────────────┐
│                     Visual Novel Engine                      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                    Engine (主引擎)                   │   │
│  │         负责初始化、运行、关闭所有子系统              │   │
│  └─────────────────────────────────────────────────────┘   │
│                              │                              │
│         ┌────────────────────┼────────────────────┐        │
│         │                    │                    │        │
│         ▼                    ▼                    ▼        │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐   │
│  │  Script     │    │   Scene     │    │  Resource   │   │
│  │  Engine     │    │   Manager   │    │  Manager    │   │
│  │             │    │             │    │             │   │
│  │ 解析脚本    │    │ 管理场景    │    │ 加载资源    │   │
│  │ 执行命令    │    │ 切换画面    │    │ 缓存图片    │   │
│  └─────────────┘    └──────┬──────┘    └─────────────┘   │
│                            │                               │
│                            ▼                               │
│                   ┌─────────────────┐                      │
│                   │  Render Engine  │                      │
│                   │     (渲染)       │                      │
│                   │                 │                      │
│                   │ 绘制背景/角色/UI │                      │
│                   └─────────────────┘                      │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 11.2 数据流向

```
游戏启动
   │
   ▼
┌──────────────────────────────────────────┐
│ 1. Engine.initialize()                   │
│    - 初始化所有子系统                    │
└──────────────────────────────────────────┘
   │
   ▼
┌──────────────────────────────────────────┐
│ 2. 加载脚本 (ScriptEngine)               │
│    demo_script.txt ──► 命令列表          │
└──────────────────────────────────────────┘
   │
   ▼
┌──────────────────────────────────────────┐
│ 3. 游戏主循环 (60 FPS)                   │
│                                          │
│    ┌─────────────────────────────────┐   │
│    │  更新 Update                    │   │
│    │  - ScriptEngine: 执行命令       │   │
│    │  - SceneManager: 更新场景        │   │
│    └─────────────────────────────────┘   │
│    │                                      │
│    │  渲染 Render                        │
│    │  - RenderEngine: 绘制画面          │   │
│    └─────────────────────────────────┘   │
└──────────────────────────────────────────┘
   │
   ▼
游戏结束
```

---

## 11.3 引擎初始化流程

### 11.3.1 main.rs 中的初始化

```rust
fn main() -> Result<()> {
    // 1. 创建引擎实例
    let mut engine = Engine::new();
    
    // 2. 配置游戏参数
    let config = GameConfig {
        title: "Visual Novel Engine".to_string(),
        width: 1280,
        height: 720,
        vsync: true,
        ..Default::default()
    };
    
    // 3. 初始化引擎（会初始化所有子系统）
    engine.initialize(config)?;
    
    // 4. 获取子系统
    let resource_manager = engine.resource_manager();
    let script_engine = engine.script_engine();
    let scene_manager = engine.scene_manager();
    
    // 5. 继续初始化...
}
```

### 11.3.2 Engine::initialize() 内部

```rust
pub fn initialize(&mut self, config: GameConfig) -> Result<()> {
    self.config = config;
    
    // 按顺序初始化各子系统
    // 1. 资源管理器（基础服务）
    self.resource_manager.lock().initialize()?;
    
    // 2. 渲染引擎（窗口和图形）
    self.render_engine.lock().initialize(...)?;
    
    // 3. 场景管理器
    self.scene_manager.lock().initialize()?;
    
    // 4. 脚本引擎
    self.script_engine.lock().initialize()?;
    
    Ok(())
}
```

---

## 11.4 游戏主循环

### 11.4.1 循环结构

```rust
pub fn run(&mut self) -> Result<()> {
    self.running = true;
    
    while self.running {
        // 1. 计算 delta time
        let delta_time = 1.0 / 60.0;
        
        // 2. 更新所有系统
        self.update(delta_time)?;
        
        // 3. 渲染画面
        self.render()?;
    }
    
    Ok(())
}
```

### 11.4.2 Update 流程

```rust
fn update(&mut self, delta_time: f32) -> Result<()> {
    // 更新场景管理器
    // 这会调用当前场景的 update
    self.scene_manager.lock().update(delta_time);
    
    Ok(())
}
```

### 11.4.3 Render 流程

```rust
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

## 11.5 完整的游戏流程示例

### 场景：玩家启动游戏，看到背景和角色对话

```
1. 引擎启动
   │
2. 脚本引擎加载 demo_script.txt
   │
3. 脚本引擎执行 @bg classroom.jpg
   │   └──► ResourceManager 加载图片
   │   └──► RenderEngine 绘制背景
   │
4. 脚本引擎执行 @music bgm_main.ogg
   │   └──► ResourceManager 加载音频
   │   └──► AudioPlayer 播放音乐
   │
5. 脚本引擎执行 小林: 早上好！
   │   └──► 设置对话框文本
   │   └──► 等待玩家点击
   │
6. 玩家点击鼠标/按空格
   │
7. 脚本引擎执行下一条命令
   │
8. ... 重复步骤 5-7
   │
9. 脚本引擎执行 @end
   │
10. 游戏结束
```

---

## 11.6 各组件交互图

```
                    ┌─────────────┐
                    │   脚本文件   │
                    │ (txt/json)  │
                    └──────┬──────┘
                           │
                           ▼
              ┌────────────────────────┐
              │     ScriptEngine        │
              │   (解析并执行命令)       │
              └───────────┬────────────┘
                          │
          ┌───────────────┼───────────────┐
          │               │               │
          ▼               ▼               ▼
   ┌────────────┐  ┌────────────┐  ┌────────────┐
   │  Resource  │  │    Scene   │  │    事件    │
   │  Manager   │  │   Manager  │  │  玩家输入  │
   └─────┬──────┘  └─────┬──────┘  └─────┬──────┘
         │               │               │
         │      ┌────────┴────────┐       │
         │      │                 │       │
         ▼      ▼                 ▼       ▼
   ┌─────────────────────────────────────────┐
   │           RenderEngine                  │
   │         (绘制所有内容到屏幕)            │
   └─────────────────────────────────────────┘
```

---

## 11.7 线程安全：Arc 和 Mutex

在 Rust 中，使用 `Arc<Mutex<T>>` 让多个组件可以安全地共享数据：

```rust
pub struct Engine {
    // Arc: 允许多个引用计数
    // Mutex: 同一时刻只有一个线程可以访问
    resource_manager: Arc<Mutex<ResourceManager>>,
    render_engine: Arc<Mutex<RenderEngine>>,
    scene_manager: Arc<Mutex<SceneManager>>,
    script_engine: Arc<Mutex<ScriptEngine>>,
}
```

**为什么需要这样？**
- 游戏有多个系统需要访问同一个资源管理器
- `Mutex` 防止同时修改造成数据错误
- `Arc` 允许多个地方持有引用

---

## 11.8 今日总结

今天我们学习了：
- ✅ **系统架构总览** - 六大子系统的关系
- ✅ **数据流向** - 从脚本到渲染的完整流程
- ✅ **初始化流程** - Engine::initialize() 的步骤
- ✅ **游戏主循环** - Update + Render 的循环
- ✅ **完整游戏流程** - 从启动到结束的示例
- ✅ **Arc + Mutex** - 线程安全机制

---

## 11.9 练习题

1. 尝试画出你理解的系统交互图
2. 如果要在显示角色之前播放音效，代码应该怎么改？
3. 为什么关闭引擎时需要按特定顺序关闭各子系统？

---

## 11.10 明日预告

明天（最后一天）我们将学习：
- **运行项目** - 如何编译和运行 Visual Novel Engine
- **扩展功能** - 如何添加新功能
- **学习建议** - 后续深入学习的方向

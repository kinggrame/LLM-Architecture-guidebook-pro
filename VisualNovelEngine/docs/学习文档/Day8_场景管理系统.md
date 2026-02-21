# 第8天：场景管理系统

> 🎬 **目标**：理解什么是场景（Scene），以及如何管理系统中的多个场景。

---

## 8.1 什么是场景？

在视觉小说中，**场景（Scene）** 代表一个完整的游戏画面或游戏状态。

### 常见的场景类型：

1. **标题画面（Title Scene）**
   - 游戏开始时的画面
   - 显示"开始游戏"、"读取存档"等按钮

2. **主游戏场景（Main Game Scene）**
   - 显示背景、角色、对话框
   - 执行脚本的地方

3. **菜单场景（Menu Scene）**
   - 游戏中的设置菜单

4. **CG画廊（Gallery Scene）**
   - 回顾已解锁的CG图片

5. **结束画面（Ending Scene）**
   - 播放结局

---

## 8.2 场景的生命周期

每个场景都有三个关键方法：

```
┌─────────────┐
│   on_enter  │  ← 进入场景时调用一次
└──────┬──────┘
       │
       ▼
┌─────────────┐
│    update   │  ← 每帧调用（60次/秒）
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   on_exit   │  ← 离开场景时调用一次
└─────────────┘
```

### 各阶段的作用：

| 阶段 | 做什么 | 示例 |
|------|--------|------|
| `on_enter` | 初始化场景 | 加载背景图、播放音乐 |
| `update` | 每帧更新 | 处理输入、检查条件 |
| `on_exit` | 清理资源 | 保存数据、停止音乐 |

---

## 8.3 场景 trait（特征）

在 Rust 中，使用 **trait** 定义场景的接口：

```rust
// 场景 trait（类似其他语言的接口）
pub trait Scene {
    // 进入场景
    fn on_enter(&mut self);
    
    // 退出场景
    fn on_exit(&mut self);
    
    // 每帧更新
    fn update(&mut self, delta_time: f32);
    
    // 渲染画面
    fn render(&mut self);
}
```

---

## 8.4 视觉小说场景示例

```rust
// 实现一个视觉小说场景
pub struct VisualNovelScene {
    name: String,
    background: Option<Texture>,
    characters: Vec<Character>,
    dialog_text: String,
    speaker_name: String,
}

impl Scene for VisualNovelScene {
    fn on_enter(&mut self) {
        println!("进入场景: {}", self.name);
        // 加载背景
    }
    
    fn on_exit(&mut self) {
        println!("离开场景: {}", self.name);
        // 保存进度
    }
    
    fn update(&mut self, delta_time: f32) {
        // 处理玩家输入
        // 检查是否需要切换对话
    }
    
    fn render(&mut self) {
        // 绘制背景
        // 绘制角色
        // 绘制对话框
    }
}
```

---

## 8.5 场景管理器（SceneManager）

SceneManager 负责管理所有场景，控制场景切换。

### 主要功能：
1. **注册场景** - 把场景添加到管理器
2. **切换场景** - 从当前场景切换到另一个
3. **获取场景** - 根据名称获取场景引用

### 代码实现：

```rust
pub struct SceneManager {
    scenes: HashMap<String, Box<dyn Scene>>,  // 所有场景
    current_scene: Option<String>,             // 当前场景名称
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            current_scene: None,
        }
    }
    
    // 注册场景
    pub fn register_scene(&mut self, name: &str, scene: impl Scene + 'static) {
        self.scenes.insert(name.to_string(), Box::new(scene));
    }
    
    // 切换场景
    pub fn change_scene(&mut self, name: &str) {
        // 1. 退出当前场景
        if let Some(ref mut current) = self.current_scene {
            if let Some(scene) = self.scenes.get_mut(current) {
                scene.on_exit();
            }
        }
        
        // 2. 切换到新场景
        self.current_scene = Some(name.to_string());
        
        // 3. 进入新场景
        if let Some(scene) = self.scenes.get_mut(name) {
            scene.on_enter();
        }
    }
    
    // 更新当前场景
    pub fn update(&mut self, delta_time: f32) {
        if let Some(name) = &self.current_scene {
            if let Some(scene) = self.scenes.get_mut(name) {
                scene.update(delta_time);
            }
        }
    }
    
    // 渲染当前场景
    pub fn render(&mut self) {
        if let Some(name) = &self.current_scene {
            if let Some(scene) = self.scenes.get(name) {
                scene.render();
            }
        }
    }
}
```

---

## 8.6 项目中的实际使用

### 8.6.1 创建场景

```rust
// main.rs

// 创建场景
let main_scene = VisualNovelScene::new("main");

// 注册到场景管理器
let mut sm = scene_manager.lock();
sm.register_scene("main", main_scene);
```

### 8.6.2 切换场景

```rust
// 切换到 main 场景
let mut sm = scene_manager.lock();
sm.change_scene("main");
```

### 8.6.3 主循环中的更新

```rust
// 游戏主循环
loop {
    // 更新场景
    scene_manager.lock().update(1.0 / 60.0);
    
    // 渲染场景
    scene_manager.lock().render();
}
```

---

## 8.7 场景切换流程图

```
玩家点击"开始游戏"
        │
        ▼
┌───────────────────┐
│   退出标题场景    │ ──► on_exit()
│   (Title Scene)   │
└────────┬──────────┘
         │
         ▼
┌───────────────────┐
│   进入主游戏场景  │ ──► on_enter()
│   (Game Scene)   │
└────────┬──────────┘
         │
         ▼
┌───────────────────┐
│   每帧更新游戏   │ ──► update()
│   (60 FPS)       │
└────────┬──────────┘
         │
         ▼
┌───────────────────┐
│  玩家结束游戏     │
└────────┬──────────┘
         │
         ▼
┌───────────────────┐
│   退出游戏场景   │ ──► on_exit()
│   进入结束画面   │
└───────────────────┘
```

---

## 8.8 高级特性：场景堆叠

有些游戏支持**叠加场景**（比如在游戏中打开菜单）：

```rust
// 压入新场景（不退出当前场景）
pub fn push_scene(&mut self, name: &str) {
    if let Some(scene) = self.scenes.get_mut(name) {
        scene.on_enter();
    }
    // 保存当前场景到堆栈
}

// 弹出场景（恢复上一个场景）
pub fn pop_scene(&mut self) {
    // 退出当前场景
    // 恢复堆栈中的上一个场景
}
```

---

## 8.9 今日总结

今天我们学习了：
- ✅ **场景（Scene）** 的概念
- ✅ **场景生命周期** - on_enter / update / on_exit
- ✅ **Scene trait** - 定义场景接口
- ✅ **SceneManager** - 管理场景切换
- ✅ **场景注册和切换** 的代码示例

---

## 8.10 练习题

1. 如果要在游戏中添加一个"设置菜单"场景，需要哪些步骤？
2. `on_enter` 和 `update` 有什么区别？为什么需要分开？
3. 如果要实现"按 ESC 打开菜单"，代码应该怎么写？

---

## 8.11 明日预告

明天我们将学习：
- **资源管理系统** - 如何加载和管理图片、音频等素材
- **纹理（Texture）** - 图片的内存表示
- **音频（Audio）** - 音乐和音效的播放
- **缓存机制** - 避免重复加载

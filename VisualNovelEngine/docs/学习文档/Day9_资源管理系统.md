# 第9天：资源管理系统

> 📁 **目标**：理解如何加载和管理游戏中的图片、音频等素材。

---

## 9.1 什么是资源管理？

游戏中的**资源（Resources）** 包括：
- **图片** - 背景、立绘、UI
- **音频** - 背景音乐、音效
- **字体** - 显示文字
- **视频** - 过场动画
- **脚本** - 剧情文本

**资源管理器**的任务：
1. 加载资源（从文件到内存）
2. 缓存资源（避免重复加载）
3. 卸载资源（释放不再使用的内存）

---

## 9.2 为什么需要缓存？

**问题场景**：
```
玩家在剧情中多次经过"教室"场景
```

**如果每次都重新加载**：
```
第1次: 加载 classroom.jpg (0.1秒)
第2次: 加载 classroom.jpg (0.1秒) ❌ 浪费时间
第3次: 加载 classroom.jpg (0.1秒) ❌ 浪费内存
```

**使用缓存后**：
```
第1次: 加载 classroom.jpg (0.1秒) → 保存到缓存
第2次: 从缓存读取 → 几乎瞬间 ✅
第3次: 从缓存读取 → 几乎瞬间 ✅
```

---

## 9.3 项目中的资源管理

### 9.3.1 资源管理器结构

```rust
// rust/src/resources/mod.rs

pub struct ResourceManager {
    textures: HashMap<String, Texture>,      // 图片缓存
    audio: HashMap<String, AudioData>,       // 音频缓存
    // ...
}
```

### 9.3.2 加载图片

```rust
impl ResourceManager {
    // 加载纹理（带缓存）
    pub fn load_texture(&mut self, path: &str) -> Option<&Texture> {
        // 1. 检查是否已缓存
        if let Some(texture) = self.textures.get(path) {
            return Some(texture);
        }
        
        // 2. 如果没有，加载新资源
        if let Ok(texture) = self.load_from_file(path) {
            // 3. 保存到缓存
            self.textures.insert(path.to_string(), texture);
            return self.textures.get(path);
        }
        
        None
    }
    
    // 实际加载文件（伪代码）
    fn load_from_file(&self, path: &str) -> Result<Texture> {
        // 使用 image 库加载图片
        let img = image::open(path)?;
        // 转换为纹理
        Ok(Texture::from_image(img))
    }
}
```

### 9.3.3 加载音频

```rust
impl ResourceManager {
    // 加载音频
    pub fn load_audio(&mut self, path: &str) -> Option<&AudioData> {
        // 类似的缓存逻辑
        if let Some(audio) = self.audio.get(path) {
            return Some(audio);
        }
        
        if let Ok(data) = std::fs::read(path) {
            self.audio.insert(path.to_string(), data);
            return self.audio.get(path);
        }
        
        None
    }
}
```

---

## 9.4 使用资源管理器

```rust
fn main() {
    let mut rm = ResourceManager::new();
    
    // 加载背景（首次加载）
    let bg = rm.load_texture("classroom.jpg");
    
    // 再次加载同一个图片（从缓存读取）
    let bg2 = rm.load_texture("classroom.jpg");
    
    // 播放音乐
    rm.play_music("bgm_main.ogg");
}
```

---

## 9.5 资源生命周期

```
加载 ──► 使用 ──► 卸载

┌─────────────────────────────────────┐
│                加载                 │
│    磁盘文件 ──► 内存（缓存）        │
└─────────────────────────────────────┘
                │
                ▼
┌─────────────────────────────────────┐
│                使用                 │
│    渲染图片 / 播放音频              │
│    （缓存中直接读取，不重复加载）    │
└─────────────────────────────────────┘
                │
                ▼
┌─────────────────────────────────────┐
│                卸载                 │
│    内存释放（可选，取决于策略）      │
└─────────────────────────────────────┘
```

### 卸载策略：
1. **手动卸载** - 需要时手动调用 `unload()`
2. **引用计数** - 没有使用时自动卸载
3. **LRU** - 最近最少使用的先卸载

---

## 9.6 项目中的实际代码

### 9.6.1 资源管理模块

```rust
// rust/src/resources/mod.rs

pub mod texture;
pub mod audio;

pub struct ResourceManager {
    textures: HashMap<String, texture::Texture>,
    audio: HashMap<String, audio::AudioBuffer>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            audio: HashMap::new(),
        }
    }
    
    pub fn initialize(&mut self) -> Result<()> {
        log::info!("ResourceManager initialized");
        Ok(())
    }
    
    pub fn shutdown(&mut self) {
        log::info!("ResourceManager shutdown");
        // 清理所有资源
        self.textures.clear();
        self.audio.clear();
    }
}
```

### 9.6.2 引擎中的资源管理

```rust
// rust/src/core/engine.rs

pub struct Engine {
    // 使用 Arc<Mutex<...>> 允许多线程安全访问
    resource_manager: Arc<Mutex<ResourceManager>>,
    // ...
}

impl Engine {
    pub fn resource_manager(&self) -> Arc<Mutex<ResourceManager>> {
        Arc::clone(&self.resource_manager)
    }
}
```

---

## 9.7 资源路径管理

通常游戏资源放在 `assets` 目录下：

```
assets/
├── images/
│   ├── backgrounds/
│   │   ├── classroom.jpg
│   │   └── hallway.jpg
│   └── characters/
│       ├── heroine.png
│       └── hero.png
├── audio/
│   ├── bgm/
│   │   └── main_theme.ogg
│   └── se/
│       ├── click.ogg
│       └── text_advance.ogg
└── scripts/
    └── demo_script.txt
```

代码中使用相对路径：

```rust
// 加载背景
let bg = rm.load_texture("assets/images/backgrounds/classroom.jpg");

// 播放音乐
rm.play_music("assets/audio/bgm/main_theme.ogg");
```

---

## 9.8 今日总结

今天我们学习了：
- ✅ **资源管理的概念** - 什么是游戏资源
- ✅ **缓存机制** - 避免重复加载
- ✅ **HashMap 缓存** - 使用字典存储资源
- ✅ **资源生命周期** - 加载、使用、卸载
- ✅ **项目中的 ResourceManager 实现**

---

## 9.9 练习题

1. 如果要添加一个新的资源类型（字体），需要修改哪些代码？
2. 解释为什么要使用缓存，而不是每次都从文件加载？
3. 如果内存不足，应该怎么处理？

---

## 9.10 明日预告

明天我们将学习：
- **渲染系统** - 如何在屏幕上绘制图片
- **RenderEngine** - 渲染引擎的工作原理
- **图形 API** - wgpu、OpenGL 等
- **游戏循环** - 每秒60帧的更新机制

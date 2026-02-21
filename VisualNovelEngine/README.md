# Visual Novel Engine

这是一个专为视觉小说/Galgame设计的跨平台游戏引擎，提供了C++和Rust两种实现版本。

## 项目结构

```
VisualNovelEngine/
├── cpp/                    # C++版本
│   ├── CMakeLists.txt
│   ├── include/            # 头文件
│   │   ├── core/          # 核心引擎
│   │   ├── rendering/     # 渲染系统
│   │   ├── resources/     # 资源管理
│   │   ├── scene/         # 场景管理
│   │   └── script/        # 脚本系统
│   └── src/               # 源文件
│       ├── core/
│       ├── rendering/
│       ├── resources/
│       ├── scene/
│       ├── script/
│       └── main.cpp
├── rust/                   # Rust版本
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── core/          # 核心引擎
│       ├── rendering/     # 渲染系统
│       ├── resources/     # 资源管理
│       ├── scene/         # 场景管理
│       └── script/        # 脚本系统
├── assets/                # 游戏资源
│   ├── images/
│   ├── audio/
│   └── scripts/
└── docs/                  # 文档
```

## 架构设计

### 核心组件

1. **Engine** - 主引擎类，协调所有子系统
2. **RenderEngine** - 渲染引擎，处理2D图形显示
3. **ResourceManager** - 资源管理，负责纹理、音频、字体加载
4. **SceneManager** - 场景管理，处理场景切换和生命周期
5. **ScriptEngine** - 脚本引擎，解析和执行视觉小说脚本
6. **EventManager** - 事件系统，处理用户输入

### 特性

- 2D渲染（背景、角色立绘、UI）
- 脚本系统（对话、选择支、跳转）
- 音频播放（背景音乐、音效）
- 资源管理（自动缓存和释放）
- 场景系统（支持场景切换和堆叠）

## C++版本

### 依赖

- SFML 2.6+ (图形、音频、窗口)
- CMake 3.16+
- C++20

### 构建

```bash
cd cpp
mkdir build && cd build
cmake ..
cmake --build .
```

### 运行

```bash
./VisualNovelEngine
```

## Rust版本

### 依赖

- wgpu (图形渲染)
- winit (窗口管理)
- rodio (音频播放)
- image (图片加载)
- serde (脚本序列化)

### 构建

```bash
cd rust
cargo build --release
```

### 运行

```bash
cargo run
```

## 脚本语法

引擎使用简单的文本脚本格式：

```
# 注释以#开头

# 显示背景
@bg classroom.jpg

# 播放音乐
@music bgm_main.ogg

# 显示角色
@char heroine happy.png center

# 对话
小林: 早上好！今天是个美好的一天呢。

# 标签（用于跳转）
*start

# 选择支（开发中）
[What will you do?]
1. 选项一 -> label1
2. 选项二 -> label2

# 跳转到标签
@jump start

# 结束
@end
```

## 示例游戏

项目包含一个示例脚本 `assets/scripts/demo_script.txt`，展示引擎的基本功能：
- 场景切换
- 角色对话
- 背景音乐
- 分支选择

## 开发计划

- [x] 核心引擎框架
- [x] 资源管理系统
- [x] 场景管理系统
- [x] 脚本解析器
- [x] 基本音频播放
- [ ] 完整图形渲染（wgpu/OpenGL）
- [ ] UI系统（文本框、按钮）
- [ ] 完整的选择支系统
- [ ] 存档/读档功能
- [ ] 动画和过渡效果
- [ ] 字体渲染
- [ ] 多语言支持

## 许可证

MIT License

## 贡献

欢迎提交Issue和Pull Request！

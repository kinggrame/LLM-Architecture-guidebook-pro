# 视觉小说引擎启动方案

## 快速启动

### 方式一：C++版本（推荐Windows/跨平台）

#### 前置要求
- CMake 3.16+
- C++20 兼容编译器 (VS2022/ GCC 11+/ Clang 14+)
- SFML 2.6+ 库

#### Windows (Visual Studio)
```powershell
# 1. 安装 vcpkg 和 SFML
vcpkg install sfml:x64-windows

# 2. 进入项目目录
cd VisualNovelEngine/cpp

# 3. 创建构建目录
mkdir build
cd build

# 4. 配置（使用 vcpkg）
cmake .. -DCMAKE_TOOLCHAIN_FILE=[vcpkg路径]/scripts/buildsystems/vcpkg.cmake

# 5. 构建
cmake --build . --config Release

# 6. 运行
.\Release\VisualNovelEngine.exe
```

#### Linux/macOS
```bash
cd VisualNovelEngine/cpp
mkdir build && cd build
cmake ..
make -j$(nproc)
./VisualNovelEngine
```

---

### 方式二：Rust版本（最简单）

#### 前置要求
- Rust 1.75+ (https://rustup.rs/)
- Cargo

#### 启动步骤
```bash
# 1. 进入Rust项目
cd VisualNovelEngine/rust

# 2. 安装依赖并构建
cargo build --release

# 3. 运行引擎
cargo run --release

# 或者直接运行二进制文件
.\target\release\vne.exe  # Windows
./target/release/vne      # Linux/macOS
```

---

## 快速测试脚本

运行以下命令快速验证引擎：

### Windows PowerShell
```powershell
# Rust版本快速测试
cd VisualNovelEngine/rust
cargo run 2>&1 | Select-String -Pattern "小林|小美|script|Engine" | ForEach-Object { $_.Line }
```

### Bash
```bash
# Rust版本快速测试
cd VisualNovelEngine/rust
cargo run 2>&1 | grep -E "(小林|小美|script|Engine)"
```

---

## 添加游戏资源

### 1. 准备资源文件
```
VisualNovelEngine/assets/
├── images/
│   ├── classroom.jpg      # 背景图片
│   ├── hallway.jpg
│   └── characters/
│       └── happy.png      # 角色立绘
├── audio/
│   ├── bgm_main.ogg       # 背景音乐
│   └── sfx_click.wav      # 音效
└── scripts/
    └── demo_script.txt    # 游戏脚本
```

### 2. 编写脚本示例
创建 `assets/scripts/my_game.txt`:
```
# 我的视觉小说

@bg title_screen.jpg

主角: 欢迎来到我的视觉小说世界！
主角: 这是一个测试场景。

@bg scene1.jpg
@char heroine normal.png center

女主角: 你好，很高兴见到你。

主角: 我也很高兴见到你。

@end
```

---

## IDE配置

### Visual Studio (C++)
1. 打开 `cpp/CMakeLists.txt`
2. 配置 CMake 设置，添加 vcpkg toolchain
3. 生成后事件自动复制 assets 目录
4. F5 直接调试运行

### VS Code
安装扩展：
- C++: C/C++ Extension Pack, CMake Tools
- Rust: rust-analyzer, CodeLLDB

配置 `.vscode/launch.json` 进行调试。

---

## 常见问题

### Q: SFML找不到？
**A:** 确保安装了SFML并正确配置CMake：
```cmake
# 在 CMakeLists.txt 中添加
set(SFML_DIR "C:/SFML/lib/cmake/SFML")
find_package(SFML 2.6 COMPONENTS graphics audio REQUIRED)
```

### Q: Rust编译错误？
**A:** 更新Rust工具链：
```bash
rustup update
rustup target add x86_64-pc-windows-msvc  # Windows
```

### Q: 资源加载失败？
**A:** 确保工作目录正确，或使用绝对路径。检查 `assets` 目录是否在可执行文件同级目录。

---

## 下一步

1. 查看 `docs/` 目录了解API文档
2. 修改 `assets/scripts/demo_script.txt` 创建你的故事
3. 添加自定义图片和音频资源
4. 扩展引擎功能（存档系统、UI美化等）

祝开发愉快！

# 第7天：脚本系统 - 让游戏剧情可编写

> 📜 **目标**：理解什么是脚本系统，以及 Visual Novel Engine 如何解析和执行剧情脚本。

---

## 7.1 什么是脚本系统？

**脚本系统**允许创作者用简单的文本格式编写游戏剧情，而不需要编程基础。

**对比**：
- ❌ **硬编码**：把剧情直接写在程序里 → 修改需要改代码
- ✅ **脚本**：把剧情写在文本文件中 → 只需修改文本文件

---

## 7.2 脚本语法示例

让我们看看项目中的示例脚本 `assets/scripts/demo_script.txt`：

```txt
# Visual Novel Demo Script
# # 开头的是注释

# 显示背景
@bg classroom.jpg

# 播放音乐
@music bgm_main.ogg

# 角色对话
小林: 早上好！今天是个美好的一天呢。
小林: 欢迎来到这个视觉小说引擎演示。

# 切换背景
@bg hallway.jpg
小林: 这里是走廊，我们可以在这里遇到其他角色。

# 显示角色
@char heroine happy.png center

# 更多对话
小林: 啊，是小美！
小美: 你好，小林！

# 选择支（玩家决定剧情）
[What will you do?]
1. 当然可以！ -> show_demo
2. 现在还不行 -> not_ready

# 标签（跳转目的地）
*show_demo
小林: 当然可以！让我给你展示一下。
@jump end

*not_ready
小林: 现在还不行，还需要一些时间完善。

*end
小林: 无论如何，谢谢你的支持！

# 结束
@end
```

---

## 7.3 命令类型

脚本中的命令可以分为几类：

| 命令 | 作用 | 示例 |
|------|------|------|
| `@bg` | 切换背景 | `@bg classroom.jpg` |
| `@char` | 显示角色 | `@char heroine happy.png center` |
| `@music` | 播放音乐 | `@music bgm_main.ogg` |
| `@sound` | 播放音效 | `@sound click.ogg` |
| `@jump` | 跳转到标签 | `@jump start` |
| `@end` | 结束游戏 | `@end` |
| `名字:` | 对话 | `小林: 你好！` |
| `*标签` | 定义标签 | `*start` |
| `[标题]` | 选择支 | `[What will you do?]` |

---

## 7.4 脚本引擎的工作原理

### 步骤1：加载脚本文件
```
文本文件 → 读取全部内容 → 按行分割
```

### 步骤2：解析每一行
```
每一行文本 → 识别命令类型 → 提取参数 → 生成 Command 结构体
```

### 步骤3：执行命令
```
Command → 找到对应处理器 → 执行实际操作
```

---

## 7.5 代码实现解析

### 7.5.1 命令类型枚举

```rust
// rust/src/script/command.rs

#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    None,
    Dialogue,     // 对话：名字: 内容
    Choice,       // 选择支
    Background,   // @bg 背景
    Character,    // @char 角色
    Music,        // @music 音乐
    Sound,        // @sound 音效
    Transition,   // @trans 过渡效果
    Wait,         // @wait 等待
    Label,        // *label 标签
    Jump,         // @jump 跳转
    End,          // @end 结束
}
```

### 7.5.2 命令结构体

```rust
#[derive(Debug, Clone)]
pub struct Command {
    pub command_type: CommandType,  // 命令类型
    pub params: Vec<String>,         // 参数列表
}
```

例如 `@bg classroom.jpg` 会被解析为：
```rust
Command {
    command_type: CommandType::Background,
    params: vec!["classroom.jpg".to_string()],
}
```

### 7.5.3 脚本引擎核心

```rust
// 简化版的脚本引擎
pub struct ScriptEngine {
    commands: Vec<Command>,     // 所有命令
    labels: HashMap<String, usize>,  // 标签位置
    current_index: usize,        // 当前执行到哪条命令
    is_running: bool,
}
```

### 7.5.4 解析一行脚本

```rust
fn parse_line(line: &str) -> Option<Command> {
    let line = line.trim();
    
    // 空行或注释，跳过
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    
    // 对话：名字: 内容
    if let Some(pos) = line.find(':') {
        let name = line[..pos].trim();
        let content = line[pos+1..].trim();
        return Some(Command::new(
            CommandType::Dialogue,
            vec![name.to_string(), content.to_string()]
        ));
    }
    
    // @命令
    if line.starts_with('@') {
        let parts: Vec<&str> = line[1..].splitn(2, ' ').collect();
        let cmd = parts[0];
        let param = parts.get(1).unwrap_or(&"");
        
        return Some(match cmd {
            "bg" => Command::new(CommandType::Background, vec![param.to_string()]),
            "char" => parse_char_command(param),
            "music" => Command::new(CommandType::Music, vec![param.to_string()]),
            "jump" => Command::new(CommandType::Jump, vec![param.to_string()]),
            "end" => Command::new(CommandType::End, vec![]),
            _ => Command::new(CommandType::None, vec![]),
        });
    }
    
    None
}
```

### 7.5.5 命令处理器

引擎需要为每种命令注册处理函数：

```rust
// main.rs 中的设置
se.set_command_handler(CommandType::Dialogue, |cmd| {
    if cmd.params.len() >= 2 {
        println!("{}: {}", cmd.params[0], cmd.params[1]);
    }
});

se.set_command_handler(CommandType::Background, |cmd| {
    if let Some(path) = cmd.params.first() {
        println!("切换背景: {}", path);
    }
});

se.set_command_handler(CommandType::Music, |cmd| {
    if let Some(path) = cmd.params.first() {
        println!("播放音乐: {}", path);
    }
});
```

---

## 7.6 处理选择和跳转

### 7.6.1 标签和跳转

```rust
// 找到标签对应的命令索引
fn find_label(&self, label_name: &str) -> Option<usize> {
    self.labels.get(label_name).copied()
}

// 执行跳转
fn jump_to(&mut self, label: &str) {
    if let Some(index) = self.find_label(label) {
        self.current_index = index;
    }
}
```

### 7.6.2 选择支

选择支的解析比较复杂，需要收集所有选项：

```rust
// 解析选择支
[What will you do?]
1. 选项一 -> label1
2. 选项二 -> label2
```

解析后需要：
1. 显示选项给玩家
2. 玩家选择后跳转到对应标签

---

## 7.7 完整流程图

```
┌─────────────────┐
│  加载脚本文件   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  解析每一行     │ ──► 提取命令类型和参数
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  存储命令列表   │ ──► 建立标签索引
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  主循环开始     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  取下一条命令   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  执行命令       │ ──► 调用对应处理器
└────────┬────────┘
         │
    ┌────┴────┐
    │ 等待输入 │
    └────┬────┘
         │
    ┌────┴────┐
    │ 是否结束│
    └────┬────┘
      Yes │ No
         ▼         │
┌─────────────────┐│
│      结束       ││
└─────────────────┘│
                   ▼
            返回"取下一条命令"
```

---

## 7.8 今日总结

今天我们学习了：
- ✅ **脚本系统的概念** - 为什么需要脚本
- ✅ **脚本语法** - @命令、对话、标签、跳转
- ✅ **命令类型枚举** - CommandType
- ✅ **命令结构体** - Command
- ✅ **解析原理** - 文本→命令
- ✅ **执行流程** - 循环执行命令

---

## 7.9 练习题

1. 阅读 `assets/scripts/demo_script.txt`，尝试添加一段新的剧情
2. 如果要添加 `@fade` 命令（淡入淡出），需要在哪些地方修改代码？
3. 解释 `@jump` 命令是如何实现的？

---

## 7.10 明日预告

明天我们将学习：
- **场景管理系统** - 什么是场景，如何切换
- **SceneManager** - 管理所有场景
- **场景的生命周期** - 进入、退出、更新

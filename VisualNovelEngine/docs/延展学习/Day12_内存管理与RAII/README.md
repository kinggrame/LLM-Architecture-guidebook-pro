# Day 12: 内存管理与 RAII

> 🎯 **目标**：掌握 C++ 的内存管理方式，理解 RAII 惯用法

---

## 代码 Demo

### Demo 1: 手动内存管理

```cpp
// 手动内存管理（危险）
#include <iostream>

int main() {
    std::cout << "=== Demo 1: 手动内存管理 ===" << std::endl;
    
    // 分配内存
    int* p = new int(42);
    std::cout << "分配: " << *p << std::endl;
    
    // 使用
    *p = 100;
    std::cout << "修改: " << *p << std::endl;
    
    // 释放内存（必须手动）
    delete p;
    // delete p; // 双重释放危险！
    
    // 数组
    int* arr = new int[5] {1, 2, 3, 4, 5};
    for (int i = 0; i < 5; i++) {
        std::cout << "arr[" << i << "] = " << arr[i] << std::endl;
    }
    delete[] arr;
    
    return 0;
}
```

### Demo 2: 智能指针

```cpp
// 智能指针
#include <iostream>
#include <memory>

int main() {
    std::cout << "=== Demo 2: 智能指针 ===" << std::endl;
    
    // unique_ptr - 独占所有权
    auto p1 = std::make_unique<int>(42);
    std::cout << "unique_ptr: " << *p1 << std::endl;
    // 自动释放
    
    // shared_ptr - 共享所有权
    auto p2 = std::make_shared<int>(100);
    auto p3 = p2;  // 引用计数 +1
    std::cout << "shared_ptr: " << *p2 << std::endl;
    std::cout << "引用计数: " << p2.use_count() << std::endl;
    
    // weak_ptr - 弱引用
    std::weak_ptr<int> wp = p2;
    std::cout << "weak_ptr 过期?: " << wp.expired() << std::endl;
    
    return 0;
}
```

### Demo 3: RAII 模式

```cpp
// RAII - 资源获取即初始化
#include <iostream>
#include <fstream>

class FileWrapper {
private:
    std::fstream file;
    std::string filename;
    
public:
    FileWrapper(const std::string& name) : filename(name) {
        file.open(filename, std::ios::out);
        std::cout << "打开文件: " << filename << std::endl;
    }
    
    ~FileWrapper() {
        if (file.is_open()) {
            file.close();
            std::cout << "关闭文件: " << filename << std::endl;
        }
    }
    
    void write(const std::string& data) {
        file << data << std::endl;
    }
};

int main() {
    std::cout << "=== Demo 3: RAII 模式 ===" << std::endl;
    
    {
        FileWrapper fw("test.txt");
        fw.write("Hello, RAII!");
    } // fw 超出作用域，自动关闭文件
    
    std::cout << "文件已自动关闭" << std::endl;
    
    return 0;
}
```

### Demo 4: 移动语义

```cpp
// 移动语义
#include <iostream>
#include <vector>
#include <string>

class Buffer {
private:
    int* data;
    size_t size;
    
public:
    Buffer(size_t s) : size(s) {
        data = new int[size];
        std::cout << "构造 Buffer(" << size << ")" << std::endl;
    }
    
    ~Buffer() {
        delete[] data;
        std::cout << "析构 Buffer" << std::endl;
    }
    
    // 拷贝构造（危险！浅拷贝）
    Buffer(const Buffer& other) : size(other.size) {
        data = new int[size];
        std::cout << "拷贝构造" << std::endl;
    }
    
    // 移动构造（高效！）
    Buffer(Buffer&& other) noexcept : data(other.data), size(other.size) {
        other.data = nullptr;
        other.size = 0;
        std::cout << "移动构造" << std::endl;
    }
    
    Buffer& operator=(const Buffer&) = delete;
    Buffer& operator=(Buffer&&) = delete;
};

int main() {
    std::cout << "=== Demo 4: 移动语义 ===" << std::endl;
    
    Buffer b1(100);
    Buffer b2 = std::move(b1);  // 移动，不复制
    
    return 0;
}
```

---

## 关键词

- **new/delete**: 手动内存分配和释放
- **RAII**: 资源获取即初始化
- **unique_ptr**: 独占所有权的智能指针
- **shared_ptr**: 共享所有权的智能指针
- **weak_ptr**: 弱引用，不增加引用计数
- **Move Semantics (移动语义)**: 高效转移资源所有权
- **Rule of Three/Five**: 三/五法则

---

## 下一天预告

**Day 13: 类和对象进阶**

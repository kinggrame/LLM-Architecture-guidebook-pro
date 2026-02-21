# C++ 高级特性与系统编程 - 完整指南

> 🎯 目标：从初学者到能够进行系统级C++开发

---

## 目录

1. [现代 C++ 特性 (C++11/14/17/20)](#1-现代-c-特性)
2. [模板编程](#2-模板编程)
3. [内存模型和并发](#3-内存模型和并发)
4. [RAII 和智能指针](#4-raii-和智能指针)
5. [STL 深入](#5-stl-深入)
6. [性能优化](#6-性能优化)
7. [系统级编程](#7-系统级编程)
8. [设计模式](#8-设计模式)

---

## 1. 现代 C++ 特性

### 1.1 auto 和 decltype

```cpp
#include <iostream>
#include <vector>
#include <map>

int main() {
    // auto - 自动类型推断
    auto x = 42;                    // int
    auto y = 3.14;                   // double
    auto z = std::string("hello");  // std::string
    
    // auto 用于迭代
    std::vector<int> v = {1, 2, 3, 4, 5};
    for (auto it = v.begin(); it != v.end(); ++it) {
        std::cout << *it << " ";
    }
    std::cout << "\n";
    
    // 范围 for 循环（C++11）
    for (auto& elem : v) {
        elem *= 2;
    }
    
    // decltype - 获取类型
    std::map<std::string, int> m;
    decltype(m)::value_type item;  // std::pair<const std::string, int>
    
    return 0;
}
```

### 1.2 Lambda 表达式

```cpp
#include <iostream>
#include <vector>
#include <algorithm>

int main() {
    std::vector<int> v = {5, 2, 8, 1, 9};
    
    // 基本 lambda
    auto print = [](int x) { std::cout << x << " "; };
    std::for_each(v.begin(), v.end(), print);
    std::cout << "\n";
    
    // 带捕获的 lambda
    int multiplier = 2;
    auto multiply = [multiplier](int x) { return x * multiplier; };
    
    std::transform(v.begin(), v.end(), v.begin(), multiply);
    
    // 捕获方式：
    // [=] - 按值捕获所有
    // [&] - 按引用捕获所有
    // [x, &y] - x 按值，y 按引用
    // [this] - 捕获 this 指针
    
    // mutable - 修改捕获的值
    int count = 0;
    auto counter = [count]() mutable { return ++count; };
    
    std::cout << counter() << "\n";  // 1
    std::cout << counter() << "\n";  // 2
    
    // generic lambda（C++14）
    auto add = [](auto a, auto b) { return a + b; };
    std::cout << add(1, 2) << "\n";
    std::cout << add(1.5, 2.5) << "\n";
    
    return 0;
}
```

### 1.3 std::optional（C++17）

```cpp
#include <iostream>
#include <optional>
#include <string>

std::optional<std::string> find_user(int id) {
    if (id == 1) {
        return "Alice";
    }
    return std::nullopt;
}

int main() {
    auto user = find_user(1);
    
    // 检查值是否存在
    if (user.has_value()) {
        std::cout << "Found: " << *user << "\n";
    }
    
    // 或者使用 value_or
    auto not_found = find_user(999);
    std::cout << not_found.value_or("Unknown") << "\n";
    
    // lambda 中的 optional
    auto result = find_user(1).transform([](const std::string& s) {
        return s.length();
    });
    
    if (result) {
        std::cout << "Length: " << *result << "\n";
    }
    
    return 0;
}
```

### 1.4 std::variant（C++17）

```cpp
#include <iostream>
#include <variant>
#include <string>

int main() {
    std::variant<int, double, std::string> v;
    
    v = 42;
    std::cout << std::get<int>(v) << "\n";  // 42
    
    v = 3.14;
    std::cout << std::get<double>(v) << "\n";  // 3.14
    
    v = "hello";
    std::cout << std::get<std::string>(v) << "\n";  // hello
    
    // 检查类型
    if (std::holds_alternative<int>(v)) {
        std::cout << "v is int\n";
    }
    
    // 访问（安全）
    std::visit([](auto&& arg) {
        std::cout << "Value: " << arg << "\n";
    }, v);
    
    return 0;
}
```

### 1.5 std::any（C++17）

```cpp
#include <iostream>
#include <any>
#include <string>

int main() {
    std::any a = 42;
    std::cout << std::any_cast<int>(a) << "\n";
    
    a = std::string("hello");
    std::cout << std::any_cast<std::string>(a) << "\n";
    
    // 检查
    if (a.type() == typeid(int)) {
        std::cout << "a is int\n";
    }
    
    return 0;
}
```

---

## 2. 模板编程

### 2.1 模板函数

```cpp
#include <iostream>

// 基本模板函数
template<typename T>
T max(T a, T b) {
    return a > b ? a : b;
}

// 模板参数默认值
template<typename T, typename Compare = std::less<T>>
T find_max(T* arr, size_t size) {
    if (size == 0) return T{};
    T max = arr[0];
    for (size_t i = 1; i < size; ++i) {
        if (Compare{}(max, arr[i])) {
            max = arr[i];
        }
    }
    return max;
}

// 非类型模板参数
template<int N>
int factorial() {
    return N * factorial<N-1>();
}

template<>
int factorial<0>() {
    return 1;
}

int main() {
    std::cout << max(1, 2) << "\n";
    std::cout << max(3.14, 2.71) << "\n";
    std::cout << max(std::string("ab"), std::string("abc")) << "\n";
    
    int arr[] = {3, 1, 4, 1, 5, 9, 2, 6};
    std::cout << find_max(arr, 8) << "\n";
    
    std::cout << factorial<5>() << "\n";  // 120
    
    return 0;
}
```

### 2.2 模板类

```cpp
#include <iostream>
#include <vector>

template<typename T>
class Stack {
private:
    std::vector<T> data_;
    
public:
    void push(const T& value) {
        data_.push_back(value);
    }
    
    T pop() {
        if (data_.empty()) {
            throw std::out_of_range("Stack is empty");
        }
        T value = data_.back();
        data_.pop_back();
        return value;
    }
    
    const T& top() const {
        return data_.back();
    }
    
    bool empty() const {
        return data_.empty();
    }
    
    size_t size() const {
        return data_.size();
    }
};

// 特化
template<>
class Stack<bool> {
private:
    std::vector<char> data_;
    
public:
    void push(bool value) {
        data_.push_back(value ? 1 : 0);
    }
    
    bool pop() {
        return data_.back() == 1;
    }
};

int main() {
    Stack<int> int_stack;
    int_stack.push(1);
    int_stack.push(2);
    std::cout << int_stack.pop() << "\n";  // 2
    
    Stack<bool> bool_stack;
    bool_stack.push(true);
    bool_stack.push(false);
    std::cout << bool_stack.pop() << "\n";  // 0
    
    return 0;
}
```

### 2.3 变长模板

```cpp
#include <iostream>

// 变长模板函数
template<typename T>
T sum(T value) {
    return value;
}

template<typename T, typename... Args>
T sum(T first, Args... args) {
    return first + sum(args...);
}

// 打印所有参数
template<typename... Args>
void print(Args... args) {
    ((std::cout << args << " "), ...);
    std::cout << "\n";
}

// 折叠表达式（C++17）
template<typename... Args>
auto add(Args... args) {
    return (... + args);  // ((1 + 2) + 3) + 4
}

int main() {
    std::cout << sum(1, 2, 3, 4, 5) << "\n";  // 15
    std::cout << sum(1.5, 2.5, 3.0) << "\n";    // 7.0
    
    print(1, "hello", 3.14);
    
    std::cout << add(1, 2, 3, 4) << "\n";  // 10
    
    return 0;
}
```

---

## 3. 内存模型和并发

### 3.1 内存顺序

```cpp
#include <atomic>
#include <thread>
#include <iostream>

// 内存顺序选项：
// memory_order_relaxed - 最宽松，仅保证原子性
// memory_order_acquire - 获取，后续读写能看到变化
// memory_order_release - 释放之前的写入对获取线程可见
// memory_order_seq_cst - 顺序一致（默认，最严格）

std::atomic<int> x{0};
std::atomic<int> y{0};

void write_x() {
    x.store(1, std::memory_order_relaxed);
}

void write_y() {
    y.store(1, std::memory_order_relaxed);
}

void read_x_then_y() {
    while (x.load(std::memory_order_relaxed) == 0) {}
    if (y.load(std::memory_order_relaxed) == 1) {
        std::cout << "x=1 -> y=1\n";
    }
}

void read_y_then_x() {
    while (y.load(std::memory_order_relaxed) == 0) {}
    if (x.load(std::memory_order_relaxed) == 1) {
        std::cout << "y=1 -> x=1\n";
    }
}

int main() {
    std::thread t1(write_x);
    std::thread t2(write_y);
    std::thread t3(read_x_then_y);
    std::thread t4(read_y_then_x);
    
    t1.join();
    t2.join();
    t3.join();
    t4.join();
    
    return 0;
}
```

### 3.2 无锁数据结构

```cpp
#include <atomic>
#include <iostream>

// 简单的无锁栈
template<typename T>
class LockFreeStack {
private:
    struct Node {
        T data;
        Node* next;
        Node(const T& d) : data(d), next(nullptr) {}
    };
    
    std::atomic<Node*> head_{nullptr};
    
public:
    void push(const T& value) {
        Node* new_node = new Node(value);
        new_node->next = head_.load(std::memory_order_relaxed);
        
        // CAS 循环
        while (!head_.compare_exchange_weak(
            new_node->next,
            new_node,
            std::memory_order_release,
            std::memory_order_relaxed
        )) {
            // 重试
        }
    }
    
    std::optional<T> pop() {
        Node* old_head = head_.load(std::memory_order_relaxed);
        
        while (old_head != nullptr) {
            Node* next = old_head->next;
            
            if (head_.compare_exchange_weak(
                old_head,
                next,
                std::memory_order_release,
                std::memory_order_relaxed
            )) {
                T value = old_head->data;
                delete old_head;
                return value;
            }
        }
        
        return std::nullopt;
    }
};

int main() {
    LockFreeStack<int> stack;
    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while (auto value = stack.pop()) {
        std::cout << *value << " ";
    }
    std::cout << "\n";
    
    return 0;
}
```

---

## 4. RAII 和智能指针

### 4.1 unique_ptr

```cpp
#include <iostream>
#include <memory>

class Resource {
public:
    Resource() { std::cout << "Acquired\n"; }
    ~Resource() { std::cout << "Released\n"; }
    void use() { std::cout << "Using resource\n"; }
};

int main() {
    // 创建 unique_ptr
    auto ptr = std::make_unique<Resource>();
    ptr->use();
    
    // 移动所有权
    auto ptr2 = std::move(ptr);
    // ptr 现在为空
    if (ptr) {
        ptr->use();
    } else {
        std::cout << "ptr is null\n";
    }
    
    // 释放所有权
    ptr2.reset();
    
    // 自定义删除器
    auto file = std::unique_ptr<FILE, void(*)(FILE*)>(
        fopen("test.txt", "w"),
        [](FILE* f) { if (f) fclose(f); }
    );
    
    if (file) {
        fprintf(file.get(), "Hello");
    }
    
    return 0;
}
```

### 4.2 shared_ptr

```cpp
#include <iostream>
#include <memory>

class Resource {
public:
    Resource() { std::cout << "Acquired\n"; }
    ~Resource() { std::cout << "Released\n"; }
};

int main() {
    auto sp1 = std::make_shared<Resource>();
    std::cout << "Use count: " << sp1.use_count() << "\n";  // 1
    
    auto sp2 = sp1;  // 复制
    std::cout << "Use count: " << sp1.use_count() << "\n";  // 2
    
    auto sp3 = std::make_shared<Resource>();
    sp1.swap(sp3);  // 交换
    
    std::cout << "sp1: " << sp1.get() << "\n";
    std::cout << "sp3: " << sp3.get() << "\n";
    
    // weak_ptr - 不增加引用计数
    std::weak_ptr<Resource> wp = sp1;
    
    if (auto locked = wp.lock()) {
        std::cout << "Still alive\n";
    }
    
    sp1.reset();  // sp1 释放
    
    if (wp.expired()) {
        std::cout << "Expired\n";
    }
    
    return 0;
}
```

### 4.3 自定义智能指针

```cpp
#include <iostream>
#include <memory>

template<typename T>
class MyUniquePtr {
private:
    T* ptr_;
    
public:
    explicit MyUniquePtr(T* ptr = nullptr) : ptr_(ptr) {}
    
    ~MyUniquePtr() {
        delete ptr_;
    }
    
    // 禁止复制
    MyUniquePtr(const MyUniquePtr&) = delete;
    MyUniquePtr& operator=(const MyUniquePtr&) = delete;
    
    // 允许移动
    MyUniquePtr(MyUniquePtr&& other) noexcept : ptr_(other.ptr_) {
        other.ptr_ = nullptr;
    }
    
    MyUniquePtr& operator=(MyUniquePtr&& other) noexcept {
        if (this != &other) {
            delete ptr_;
            ptr_ = other.ptr_;
            other.ptr_ = nullptr;
        }
        return *this;
    }
    
    T& operator*() const { return *ptr_; }
    T* operator->() const { return ptr_; }
    T* get() const { return ptr_; }
    
    void reset(T* ptr = nullptr) {
        delete ptr_;
        ptr_ = ptr;
    }
    
    T* release() {
        T* tmp = ptr_;
        ptr_ = nullptr;
        return tmp;
    }
};

int main() {
    MyUniquePtr<int> ptr(new int(42));
    std::cout << *ptr << "\n";
    
    return 0;
}
```

---

## 5. STL 深入

### 5.1 容器比较

```cpp
#include <iostream>
#include <vector>
#include <list>
#include <deque>
#include <array>

int main() {
    // vector - 动态数组，随机访问快，插入删除 O(n)
    std::vector<int> v = {1, 2, 3};
    v.push_back(4);  // 末尾插入 O(1) amortized
    v.insert(v.begin() + 1, 5);  // 中间插入 O(n)
    
    // list - 双向链表，插入删除 O(1)，随机访问 O(n)
    std::list<int> l = {1, 2, 3};
    l.push_back(4);
    l.insert(++l.begin(), 5);  // O(1)
    
    // deque - 双端队列，两端操作 O(1)
    std::deque<int> d = {1, 2, 3};
    d.push_front(0);  // O(1)
    d.push_back(4);  // O(1)
    
    // array - 固定大小数组（栈上）
    std::array<int, 4> a = {1, 2, 3, 4};
    
    return 0;
}
```

### 5.2 算法

```cpp
#include <iostream>
#include <vector>
#include <algorithm>
#include <numeric>

int main() {
    std::vector<int> v = {5, 2, 8, 1, 9, 3};
    
    // 排序
    std::sort(v.begin(), v.end());
    
    // 部分排序 - 只找出前3个最小的
    std::partial_sort(v.begin(), v.begin() + 3, v.end());
    
    // nth_element - 第n小的元素在正确位置
    std::nth_element(v.begin(), v.begin() + 2, v.end());
    
    // 二分查找
    if (std::binary_search(v.begin(), v.end(), 5)) {
        std::cout << "Found\n";
    }
    
    // lower_bound / upper_bound
    auto range = std::equal_range(v.begin(), v.end(), 5);
    
    // 变换
    std::vector<int> v2(v.size());
    std::transform(v.begin(), v.end(), v2.begin(), [](int x) { return x * 2; });
    
    // 数值算法
    int sum = std::accumulate(v.begin(), v.end(), 0);
    double mean = (double)sum / v.size();
    
    std::cout << "Sum: " << sum << ", Mean: " << mean << "\n";
    
    return 0;
}
```

### 5.3 迭代器

```cpp
#include <iostream>
#include <vector>
#include <iterator>

int main() {
    std::vector<int> v = {1, 2, 3, 4, 5};
    
    // 正向迭代器
    for (auto it = v.begin(); it != v.end(); ++it) {
        std::cout << *it << " ";
    }
    std::cout << "\n";
    
    // 反向迭代器
    for (auto it = v.rbegin(); it != v.rend(); ++it) {
        std::cout << *it << " ";
    }
    std::cout << "\n";
    
    // 流迭代器
    std::copy(v.begin(), v.end(), 
              std::ostream_iterator<int>(std::cout, " "));
    std::cout << "\n";
    
    // 插入迭代器
    std::vector<int> v2;
    std::copy(v.begin(), v.end(), std::back_inserter(v2));
    
    return 0;
}
```

---

## 6. 性能优化

### 6.1 移动语义

```cpp
#include <iostream>
#include <vector>
#include <string>

class Heavy {
public:
    std::vector<int> data;
    
    Heavy() : data(1000, 0) {
        std::cout << "Construct\n";
    }
    
    Heavy(const Heavy& other) : data(other.data) {
        std::cout << "Copy\n";
    }
    
    Heavy(Heavy&& other) noexcept : data(std::move(other.data)) {
        std::cout << "Move\n";
    }
    
    Heavy& operator=(const Heavy& other) {
        data = other.data;
        std::cout << "Copy assign\n";
        return *this;
    }
    
    Heavy& operator=(Heavy&& other) noexcept {
        data = std::move(other.data);
        std::cout << "Move assign\n";
        return *this;
    }
};

int main() {
    Heavy h1;
    Heavy h2 = std::move(h1);  // 移动构造
    
    h1 = Heavy();  // 移动赋值
    
    std::vector<Heavy> v;
    v.push_back(Heavy());  // 直接构造，避免拷贝
    
    return 0;
}
```

### 6.2 内存对齐

```cpp
#include <iostream>
#include <vector>

// 对齐属性
struct Align16 {
    alignas(16) int x;
    alignas(16) int y;
};

// 模板重排
template<typename T>
struct AlignedStorage {
    alignas(T) char data[sizeof(T)];
    
    T* get() { return reinterpret_cast<T*>(data); }
};

int main() {
    std::cout << "alignof(int): " << alignof(int) << "\n";
    std::cout << "alignof(double): " << alignof(double) << "\n";
    std::cout << "sizeof(Align16): " << sizeof(Align16) << "\n";
    
    // 动态对齐分配
    void* ptr = nullptr;
    posix_memalign(&ptr, 32, 128);
    std::cout << "Aligned pointer: " << ptr << "\n";
    free(ptr);
    
    return 0;
}
```

---

## 7. 系统级编程

### 7.1 内存映射文件

```cpp
#include <iostream>
#include <sys/mman.h>
#include <fcntl.h>
#include <unistd.h>

int main() {
    // 创建并打开文件
    int fd = open("test.bin", O_RDWR | O_CREAT, 0644);
    if (fd == -1) {
        perror("open");
        return 1;
    }
    
    // 扩展文件大小
    size_t size = 4096;
    ftruncate(fd, size);
    
    // 内存映射
    void* addr = mmap(nullptr, size, 
                      PROT_READ | PROT_WRITE,
                      MAP_SHARED, fd, 0);
    
    if (addr == MAP_FAILED) {
        perror("mmap");
        return 1;
    }
    
    // 使用内存
    int* data = static_cast<int*>(addr);
    data[0] = 42;
    
    // 同步到磁盘
    msync(addr, size, MS_SYNC);
    
    // 解除映射
    munmap(addr, size);
    close(fd);
    
    return 0;
}
```

### 7.2 进程间通信

```cpp
#include <iostream>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>

// 共享内存
struct SharedData {
    int value;
    bool ready;
};

int main() {
    // 创建共享内存
    int shm_fd = shm_open("/my_shm", O_CREAT | O_RDWR, 0666);
    ftruncate(shm_fd, sizeof(SharedData));
    
    // 映射
    void* addr = mmap(nullptr, sizeof(SharedData),
                      PROT_READ | PROT_WRITE, MAP_SHARED,
                      shm_fd, 0);
    
    auto* data = static_cast<SharedData*>(addr);
    data->value = 42;
    data->ready = true;
    
    // 同步
    msync(addr, sizeof(SharedData), MS_SYNC);
    
    munmap(addr, sizeof(SharedData));
    close(shm_fd);
    
    return 0;
}
```

---

## 8. 设计模式

### 8.1 单例模式

```cpp
class Singleton {
private:
    static std::once_flag init_flag_;
    static Singleton* instance_;
    
    Singleton() = default;
    
public:
    static Singleton* get_instance() {
        std::call_once(init_flag_, []() {
            instance_ = new Singleton();
        });
        return instance_;
    }
    
    // C++11 建议使用这种方式
    static Singleton& get() {
        static Singleton instance;
        return instance;
    }
};

Singleton* Singleton::instance_ = nullptr;
std::once_flag Singleton::init_flag_;
```

### 8.2 观察者模式

```cpp
#include <iostream>
#include <vector>
#include <algorithm>

class Observer {
public:
    virtual void update(int value) = 0;
    virtual ~Observer() = default;
};

class Subject {
private:
    std::vector<Observer*> observers_;
    int value_;
    
public:
    void attach(Observer* o) {
        observers_.push_back(o);
    }
    
    void detach(Observer* o) {
        observers_.erase(std::remove(observers_.begin(), observers_.end(), o));
    }
    
    void notify() {
        for (auto* o : observers_) {
            o->update(value_);
        }
    }
    
    void set_value(int v) {
        value_ = v;
        notify();
    }
};

class ConcreteObserver : public Observer {
public:
    void update(int value) override {
        std::cout << "Observer got value: " << value << "\n";
    }
};
```

---

## 面试题精选

### Q1: vector 扩容机制

```cpp
// 1. 容量满时，扩容约 1.5-2 倍
// 2. 重新分配内存，拷贝/移动元素
// 3. 释放旧内存
// 4. 建议使用 reserve() 预分配
```

### Q2: shared_ptr 线程安全

```cpp
// shared_ptr 引用计数线程安全
// 多个线程读写同一个 shared_ptr 需要加锁
// shared_ptr 指向的对象默认不保证线程安全
```

### Q3: new 和 malloc

```cpp
// new: 调用构造函数，抛出异常
// malloc: 只分配内存，返回 void*
// new T[]: 调用默认构造函数
// placement new: 在指定地址构造
```

---

## 练习项目

1. **实现线程池**
2. **实现内存池**
3. **实现 JSON 解析器**
4. **实现 Redis 客户端**

---

## 参考资源

- [cppreference.com](https://en.cppreference.com/)
- [learncpp.com](https://www.learncpp.com/)
- [Effective Modern C++](https://www.oreilly.com/library/view/effective-modern-c/9781491908419/)
- [C++ Core Guidelines](https://isocpp.github.io/CppCoreGuidelines/)

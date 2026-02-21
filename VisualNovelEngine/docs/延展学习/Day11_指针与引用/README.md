# Day 11: C++ 指针与引用

> 🎯 **目标**：掌握 C++ 的指针和引用，理解内存布局

---

## 代码 Demo

### Demo 1: 指针基础

```cpp
// 指针基础演示
#include <iostream>

int main() {
    std::cout << "=== Demo 1: 指针基础 ===" << std::endl;
    
    int a = 10;
    int* ptr = &a;
    
    std::cout << "a 的值: " << a << std::endl;
    std::cout << "a 的地址: " << &a << std::endl;
    std::cout << "ptr 的值: " << ptr << std::endl;
    std::cout << "ptr 指向的值: " << *ptr << std::endl;
    
    // 修改指针指向的值
    *ptr = 20;
    std::cout << "修改后 a 的值: " << a << std::endl;
    
    return 0;
}
```

### Demo 2: 引用

```cpp
// 引用演示
#include <iostream>

int main() {
    std::cout << "=== Demo 2: 引用 ===" << std::endl;
    
    int a = 10;
    int& ref = a;  // ref 是 a 的引用
    
    std::cout << "a = " << a << std::endl;
    std::cout << "ref = " << ref << std::endl;
    
    ref = 20;  // 修改 ref 就是修改 a
    std::cout << "修改后 a = " << a << std::endl;
    std::cout << "修改后 ref = " << ref << std::endl;
    
    return 0;
}
```

### Demo 3: 指针运算

```cpp
// 指针运算
#include <iostream>

int main() {
    std::cout << "=== Demo 3: 指针运算 ===" << std::endl;
    
    int arr[] = {10, 20, 30, 40, 50};
    int* ptr = arr;
    
    std::cout << "指针指向: " << *ptr << std::endl;
    
    ptr++;  // 指针算术运算
    std::cout << "ptr++ 后: " << *ptr << std::endl;
    
    ptr += 2;
    std::cout << "ptr+=2 后: " << *ptr << std::endl;
    
    // 数组遍历
    for (int i = 0; i < 5; i++) {
        std::cout << "arr[" << i << "] = " << *(arr + i) << std::endl;
    }
    
    return 0;
}
```

### Demo 4: this 指针

```cpp
// this 指针演示
#include <iostream>
#include <string>

class Point {
private:
    int x, y;
    
public:
    Point(int x, int y) : x(x), y(y) {}
    
    void print() {
        std::cout << "Point(" << this->x << ", " << this->y << ")" << std::endl;
    }
    
    Point* add(Point* other) {
        return new Point(this->x + other->x, this->y + other->y);
    }
    
    Point& addRef(Point& other) {
        this->x += other.x;
        this->y += other.y;
        return *this;
    }
};

int main() {
    std::cout << "=== Demo 4: this 指针 ===" << std::endl;
    
    Point p1(10, 20);
    Point p2(30, 40);
    
    p1.print();
    
    Point* p3 = p1.add(&p2);
    p3->print();
    
    p1.addRef(p2);
    p1.print();
    
    delete p3;
    return 0;
}
```

---

## 关键词

- **Pointer (指针)**: 存储地址的变量
- **Reference (引用)**: 变量的别名
- **& (取地址符)**: 获取变量地址
- **\* (解引用符)**: 访问指针指向的值
- **this**: 指向当前对象的指针
- **Pointer Arithmetic (指针运算)**: 指针的加减运算

---

## 下一天预告

**Day 12: 内存管理与 RAII**

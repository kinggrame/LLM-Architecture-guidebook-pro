#include <iostream>

// Demo 1: 指针基础
void demo1_pointers() {
    std::cout << "=== Demo 1: 指针基础 ===" << std::endl;
    
    int a = 10;
    int* ptr = &a;
    
    std::cout << "a = " << a << std::endl;
    std::cout << "&a = " << &a << std::endl;
    std::cout << "ptr = " << ptr << std::endl;
    std::cout << "*ptr = " << *ptr << std::endl;
    
    *ptr = 20;
    std::cout << "修改后 a = " << a << std::endl;
}

// Demo 2: 引用
void demo2_references() {
    std::cout << "\n=== Demo 2: 引用 ===" << std::endl;
    
    int a = 10;
    int& ref = a;
    
    std::cout << "a = " << a << ", ref = " << ref << std::endl;
    ref = 20;
    std::cout << "修改后 a = " << a << ", ref = " << ref << std::endl;
}

// Demo 3: 指针运算
void demo3_pointer_arithmetic() {
    std::cout << "\n=== Demo 3: 指针运算 ===" << std::endl;
    
    int arr[] = {10, 20, 30, 40, 50};
    int* ptr = arr;
    
    for (int i = 0; i < 5; i++) {
        std::cout << "*(arr + " << i << ") = " << *(arr + i) << std::endl;
    }
}

// Demo 4: this 指针
class Point {
public:
    int x, y;
    Point(int x, int y) : x(x), y(y) {}
    
    void print() {
        std::cout << "Point(" << this->x << ", " << this->y << ")" << std::endl;
    }
};

void demo4_this_pointer() {
    std::cout << "\n=== Demo 4: this 指针 ===" << std::endl;
    
    Point p(10, 20);
    p.print();
}

int main() {
    demo1_pointers();
    demo2_references();
    demo3_pointer_arithmetic();
    demo4_this_pointer();
    
    return 0;
}

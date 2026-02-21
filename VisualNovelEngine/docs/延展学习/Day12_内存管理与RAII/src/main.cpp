#include <iostream>
#include <memory>

void demo1_manual_memory() {
    std::cout << "=== Demo 1: 手动内存管理 ===" << std::endl;
    
    int* p = new int(42);
    std::cout << "值: " << *p << std::endl;
    delete p;
    
    int* arr = new int[5] {1, 2, 3, 4, 5};
    for (int i = 0; i < 5; i++) std::cout << arr[i] << " ";
    std::cout << std::endl;
    delete[] arr;
}

void demo2_smart_pointers() {
    std::cout << "\n=== Demo 2: 智能指针 ===" << std::endl;
    
    auto up = std::make_unique<int>(100);
    std::cout << "unique_ptr: " << *up << std::endl;
    
    auto sp = std::make_shared<int>(200);
    auto sp2 = sp;
    std::cout << "shared_ptr: " << *sp << ", count: " << sp.use_count() << std::endl;
}

void demo3_raii() {
    std::cout << "\n=== Demo 3: RAII ===" << std::endl;
    
    class FileGuard {
    public:
        FileGuard() { std::cout << "获取资源" << std::endl; }
        ~FileGuard() { std::cout << "释放资源" << std::endl; }
    };
    
    {
        FileGuard fg;
    }
    std::cout << "作用域结束，资源已释放" << std::endl;
}

void demo4_move_semantics() {
    std::cout << "\n=== Demo 4: 移动语义 ===" << std::endl;
    
    class MoveOnly {
    public:
        MoveOnly() { std::cout << "构造" << std::endl; }
        ~MoveOnly() { std::cout << "析构" << std::endl; }
        MoveOnly(MoveOnly&&) noexcept { std::cout << "移动" << std::endl; }
    };
    
    MoveOnly m1;
    MoveOnly m2 = std::move(m1);
}

int main() {
    demo1_manual_memory();
    demo2_smart_pointers();
    demo3_raii();
    demo4_move_semantics();
    return 0;
}

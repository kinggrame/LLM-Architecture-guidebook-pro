#!/bin/bash
# Linux/macOS 快速启动脚本
# 保存为 start_engine.sh 并运行: chmod +x start_engine.sh && ./start_engine.sh

echo -e "\033[36m============================================"
echo "  视觉小说引擎启动器"
echo "============================================\033[0m"
echo ""

read -p "选择引擎版本 (1=C++, 2=Rust, 默认=Rust): " engineType

if [ "$engineType" == "1" ]; then
    echo -e "\033[33m启动 C++ 版本...\033[0m"
    
    if [ ! -d "cpp/build" ]; then
        echo -e "\033[32m首次构建，正在配置...\033[0m"
        mkdir -p cpp/build
        cd cpp/build
        cmake ..
        make -j$(nproc)
    else
        cd cpp/build
    fi
    
    if [ -f "./VisualNovelEngine" ]; then
        ./VisualNovelEngine
    else
        echo -e "\033[31m错误：找不到可执行文件，请先构建\033[0m"
    fi
    
    cd ../..
else
    echo -e "\033[33m启动 Rust 版本...\033[0m"
    cd rust
    cargo run --release
    cd ..
fi

echo ""
echo -e "\033[36m引擎已退出\033[0m"
read -p "按回车键继续..."

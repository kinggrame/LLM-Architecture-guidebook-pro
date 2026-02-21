# Windows 快速启动脚本
# 保存为 start_engine.ps1 并运行

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  视觉小说引擎启动器" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""

$engineType = Read-Host "选择引擎版本 (1=C++, 2=Rust, 默认=Rust)"

if ($engineType -eq "1") {
    Write-Host "启动 C++ 版本..." -ForegroundColor Yellow
    
    # 检查构建目录
    if (-not (Test-Path "cpp/build")) {
        Write-Host "首次构建，正在配置..." -ForegroundColor Green
        New-Item -ItemType Directory -Force -Path "cpp/build"
        Set-Location "cpp/build"
        cmake ..
        cmake --build . --config Release
    } else {
        Set-Location "cpp/build"
    }
    
    # 运行
    if (Test-Path "Release/VisualNovelEngine.exe") {
        .\Release\VisualNovelEngine.exe
    } elseif (Test-Path "VisualNovelEngine.exe") {
        .\VisualNovelEngine.exe
    } else {
        Write-Host "错误：找不到可执行文件，请先构建" -ForegroundColor Red
    }
    
    Set-Location ../..
} else {
    Write-Host "启动 Rust 版本..." -ForegroundColor Yellow
    Set-Location "rust"
    cargo run --release
    Set-Location ..
}

Write-Host ""
Write-Host "引擎已退出" -ForegroundColor Cyan
pause

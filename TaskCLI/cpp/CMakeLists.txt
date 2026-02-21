set(CMAKE_POLICY_VERSION_MINIMUM 3.5)
cmake_minimum_required(VERSION 3.16)
project(TaskCLI)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# 消除 CMake 4.x 警告
if(POLICY CMP0135)
  cmake_policy(SET CMP0135 NEW)
endif()

include(FetchContent)

# ✅ 修正：官方源 + 正确文件名 + 无空格 + SHA256 校验
FetchContent_Declare(
  nlohmann_json
  URL https://github.com/nlohmann/json/releases/download/v3.11.3/json.tar.xz
  URL_HASH SHA256=d6c65aca6b1ed68e7a182f4757257b107ae403032760ed6ef121c9d55e81757d
)
FetchContent_MakeAvailable(nlohmann_json)

add_executable(taskcli
    src/main.cpp
    src/model/task.cpp
    src/model/user.cpp
    src/storage/json_store.cpp
)

target_include_directories(taskcli PRIVATE
    ${CMAKE_SOURCE_DIR}/include
)

target_link_libraries(taskcli PRIVATE nlohmann_json::nlohmann_json)
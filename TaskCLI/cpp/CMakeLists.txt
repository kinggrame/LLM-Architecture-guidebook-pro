cmake_minimum_required(VERSION 3.16)
project(TaskCLI)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# JSON library (nlohmann/json)
find_package(nlohmann_json 3.2 REQUIRED)

add_executable(taskcli
    src/main.cpp
    src/model/task.cpp
    src/model/user.cpp
    src/storage/json_store.cpp
    src/cli/commands.cpp
)

target_include_directories(taskcli PRIVATE
    ${CMAKE_SOURCE_DIR}/include
)

target_link_libraries(taskcli PRIVATE
    nlohmann_json::nlohmann_json
)

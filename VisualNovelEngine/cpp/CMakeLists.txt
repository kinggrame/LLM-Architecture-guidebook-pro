cmake_minimum_required(VERSION 3.16)
project(VisualNovelEngine VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find SFML
find_package(SFML 2.6 COMPONENTS graphics audio window system REQUIRED)

# Source files
file(GLOB_RECURSE SOURCES 
    src/*.cpp
    src/**/*.cpp
)

file(GLOB_RECURSE HEADERS
    include/*.h
    include/**/*.h
)

# Create executable
add_executable(${PROJECT_NAME} ${SOURCES} ${HEADERS})

target_include_directories(${PROJECT_NAME} PRIVATE 
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

target_link_libraries(${PROJECT_NAME} PRIVATE
    sfml-graphics
    sfml-audio
    sfml-window
    sfml-system
)

# Copy assets to build directory
add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_directory
    ${CMAKE_SOURCE_DIR}/../assets $<TARGET_FILE_DIR:${PROJECT_NAME}>/assets
)

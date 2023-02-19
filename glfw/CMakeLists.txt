cmake_minimum_required(VERSION 3.25)
project(glfw)

FetchContent_Declare(glfw GIT_REPOSITORY https://github.com/glfw/glfw.git GIT_TAG 3.3.8)
FetchContent_MakeAvailable(glfw)

add_library(glfw-hpp inc/glfw.hpp src/glfw.cpp)
target_link_libraries(glfw-hpp PUBLIC glfw PRIVATE spdlog::spdlog)
target_include_directories(glfw-hpp PUBLIC inc)
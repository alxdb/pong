cmake_minimum_required(VERSION 3.25)
project(opengl)

FetchContent_Declare(glew GIT_REPOSITORY https://github.com/Perlmint/glew-cmake GIT_TAG glew-cmake-2.2.0)
FetchContent_MakeAvailable(glew)

add_library(opengl inc/opengl.hpp src/opengl.cpp)
target_link_libraries(opengl PUBLIC libglew_static PRIVATE spdlog::spdlog)
target_include_directories(opengl PUBLIC inc)
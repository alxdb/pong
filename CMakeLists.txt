cmake_minimum_required(VERSION 3.25)
project(pong)

include(FetchContent)

FetchContent_Declare(spdlog GIT_REPOSITORY https://github.com/gabime/spdlog.git GIT_TAG v1.11.0)
FetchContent_MakeAvailable(spdlog)

add_subdirectory(glfw)
add_subdirectory(opengl)

add_executable(main main.cpp)
target_link_libraries(main PRIVATE glfw-hpp opengl spdlog::spdlog)
cmake_minimum_required(VERSION 3.28)
project(pong)

include(FetchContent)

FetchContent_Declare(spdlog GIT_REPOSITORY https://github.com/gabime/spdlog.git GIT_TAG v1.11.0)
FetchContent_MakeAvailable(spdlog)

FetchContent_Declare(glfw GIT_REPOSITORY https://github.com/glfw/glfw.git GIT_TAG 3.3.8)
FetchContent_MakeAvailable(glfw)

set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 26)

add_subdirectory(src)
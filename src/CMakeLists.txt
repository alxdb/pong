add_executable(pong)
target_sources(pong PRIVATE main.cpp)
target_link_libraries(pong PRIVATE glfw spdlog::spdlog)
#include <chrono>
#include <iostream>

#include <spdlog/cfg/env.h>
#include <spdlog/spdlog.h>

#include <glfw.hpp>
#include <opengl.hpp>

using namespace std;
using namespace glfw;
using namespace opengl;
using namespace std::chrono;
using namespace std::chrono_literals;

using hr_clock = high_resolution_clock;

struct Application {
  Application();

  void main_loop();
  void handle_window_events();

  GLFW glfw;
  Window window;
  OpenGL opengl;

  const hr_clock::duration desired_frame_duration = 16ms;

  hr_clock::time_point last_update;
  hr_clock::duration last_frame_duration;
  bool should_skip = false;
  bool should_exit = false;
};

int main() {
  spdlog::cfg::load_env_levels();
  Application application;
  application.main_loop();
}

Application::Application() : window(1920, 1080, "Pong") {}

void Application::main_loop() {
  glClearColor(0.0f, 0.0f, 0.0f, 1.0f);

  auto time_waited = hr_clock::duration::zero();
  while (!should_exit) {
    auto time_to_wait =
        (desired_frame_duration - last_frame_duration) - time_waited;
    if (time_to_wait > hr_clock::duration::zero()) {
      auto time_since_last_update = hr_clock::now() - last_update;
      should_skip = time_since_last_update < time_to_wait;
    } else {
      should_skip = false;
    }
    if (should_skip) {
      continue;
    }
    auto frame_start = hr_clock::now();
    handle_window_events();
    glClear(GL_COLOR_BUFFER_BIT);
    last_update = hr_clock::now();
    last_frame_duration = last_update - frame_start;
    spdlog::debug("Frame Duration {}ms",
                  duration_cast<milliseconds>(last_frame_duration).count());
  }
}

void Application::handle_window_events() {
  glfwPollEvents();
  if (glfwWindowShouldClose(window)) {
    should_exit = true;
    return;
  }
}
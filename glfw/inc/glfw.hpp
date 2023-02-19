#pragma once

#define GLFW_INCLUDE_NONE
#include <GLFW/glfw3.h>

namespace glfw {

class GLFW {
public:
  GLFW();
  ~GLFW();
};

class Window {
public:
  Window(int height, int width, const char *title,
         GLFWmonitor *monitor = nullptr, GLFWwindow *share = nullptr);
  ~Window();

  operator GLFWwindow *() const { return m_handle; };

private:
  GLFWwindow *m_handle;
};

} // namespace glfw
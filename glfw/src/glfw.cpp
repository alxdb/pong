#include <glfw.hpp>

#include <cstdlib>
#include <exception>

#include <spdlog/spdlog.h>

using namespace glfw;
using namespace std;

GLFW::GLFW() {
  if (glfwInit() != GLFW_TRUE) {
    spdlog::error("Failed to initialize GLFW");
    std::exit(EXIT_FAILURE);
  }
}

GLFW::~GLFW() { glfwTerminate(); }

Window::Window(int width, int height, const char *title, GLFWmonitor *monitor,
               GLFWwindow *share)
    : m_handle(glfwCreateWindow(width, height, title, monitor, share)) {
  glfwMakeContextCurrent(m_handle);
}

Window::~Window() { glfwDestroyWindow(static_cast<GLFWwindow *>(m_handle)); }

#include <opengl.hpp>

#include <cstdlib>

#include <spdlog/spdlog.h>

using namespace opengl;

OpenGL::OpenGL() {
  if (glewInit() != GLEW_OK) {
    spdlog::error("Failed to initialize GLEW");
    std::exit(EXIT_FAILURE);
  }
}
#include "gui.h"
#include <GL/glew.h>
#include <GLFW/glfw3.h> // Include the GLFW header
#include <stdio.h>

void glfw_error_callback(int error, const char *description) {
  fprintf(stderr, "GLFW Error %d: %s\n", error, description);
}

void glfw_framebuffer_size_callback(GLFWwindow *window, int width, int height) {
  glViewport(0, 0, width, height);
}

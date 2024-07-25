#ifndef GUI_H
#define GUI_H

#include <GL/glew.h>
#include <GLFW/glfw3.h> // Include the GLFW header

void glfw_error_callback(int error, const char *description);
void glfw_framebuffer_size_callback(GLFWwindow *window, int width, int height);

#endif // GUI_H

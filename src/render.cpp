#include "render.h"
#include "imgui.h"
#include <GL/glew.h>
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>

void renderGUI() {
  // GUI code here
  ImGui::Begin("Hello, World!");
  ImGui::Text("This is a GUI element");
  ImGui::End();
}

void renderScene(Camera &camera) {
  // 3D rendering code here
  glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
  glMatrixMode(GL_PROJECTION);
  glLoadIdentity();
  gluPerspective(45, 1280 / 720, 0.1, 100);

  glMatrixMode(GL_MODELVIEW);
  glLoadIdentity();
  glm::mat4 view = camera.GetViewMatrix();
  glLoadMatrixf(glm::value_ptr(view));

  // Draw a sphere
  GLUquadric *quadric = gluNewQuadric();
  gluSphere(quadric, 1, 32, 32);
  gluDeleteQuadric(quadric);
}

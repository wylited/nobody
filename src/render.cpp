#include "render.h"
#include "imgui.h"
#include <GL/glew.h>

void renderGUI() {
  // GUI code here
  ImGui::Begin("Hello, World!");
  ImGui::Text("This is a GUI element");
  ImGui::End();
}

void renderScene() {
  // 3D rendering code here
  glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
  glMatrixMode(GL_PROJECTION);
  glLoadIdentity();
  gluPerspective(45, 1280 / 720, 0.1, 100);

  glMatrixMode(GL_MODELVIEW);
  glLoadIdentity();
  gluLookAt(0, 0, 5, 0, 0, 0, 0, 1, 0);

  // Draw a sphere
  GLUquadric *quadric = gluNewQuadric();
  gluSphere(quadric, 1, 32, 32);
  gluDeleteQuadric(quadric);
}

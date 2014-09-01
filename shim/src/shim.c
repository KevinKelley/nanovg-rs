#define NANOVG_GL3_IMPLEMENTATION
#define STB_IMAGE_WRITE_IMPLEMENTATION

#if defined(__APPLE__)
#include <OpenGL/gl.h>
#include <OpenGL/gl3.h>
#include <OpenGL/glu.h>
#include <OpenGL/glext.h>
#include <GLUT/glut.h>
#else
#include <GL/gl.h>
#endif

#include <GLFW/glfw3.h>

#include <nanovg.h>

#include <nanovg_gl.h>

#include <stb_image_write.h>


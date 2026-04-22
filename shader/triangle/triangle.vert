#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

// Input vertex position
layout (location = 0) in vec4 pos;

// Input vertex color
layout (location = 1) in vec4 color;

// Output color to be passed to the fragment shader
layout (location = 0) out vec4 o_color;

// The main function executes once per vertex.
// It takes position and color inputs, outputs the raw position directly to
// clip space without transformation, and passes the color to the fragment shader.
void main() {
    o_color = color;
    gl_Position = pos;
}

#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

// Input vertex position
layout (location = 0) in vec4 pos;

// Input vertex color
layout (location = 1) in vec4 color;

// Uniform buffer object containing the Model-View-Projection (MVP) matrix
layout (binding = 0) uniform UniformBufferObject {
    mat4 mvp;
} ubo;

// Output color to be passed to the fragment shader
layout (location = 0) out vec4 o_color;

// The main function executes once per vertex.
// It transforms the input vertex position using a Model-View-Projection (MVP) matrix
// and passes the color to the fragment shader.
void main() {
    o_color = color;
    gl_Position = ubo.mvp * pos;
}

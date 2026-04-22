#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

// Input vertex position
layout (location = 0) in vec4 pos;

// Input UV coordinates
layout (location = 1) in vec2 uv;

// Uniform buffer object containing the Model-View-Projection (MVP) matrix
layout (binding = 0) uniform UniformBufferObject {
    mat4 mvp;
} ubo;

// Output UV coordinates to be passed to the fragment shader
layout (location = 0) out vec2 o_uv;

// The main function executes once per vertex.
// It transforms the input vertex position using a Model-View-Projection (MVP) matrix
// and passes the UV coordinates to the fragment shader.
void main() {
    o_uv = uv;
    gl_Position = ubo.mvp * pos;
}

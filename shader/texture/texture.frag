#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

// 2D texture sampler
layout (binding = 1) uniform sampler2D samplerColor;

// Uniform buffer object containing color data
layout (binding = 0) uniform UBO{
    vec3 color;
} ubo;

// Input UV coordinates from the vertex shader
layout (location = 0) in vec2 o_uv;

// Output color for the fragment
layout (location = 0) out vec4 uFragColor;

// The main function executes once per fragment (pixel).
// It samples a 2D texture using the interpolated UV coordinates
// from the vertex shader to compute the final pixel color.
void main() {
    vec4 color = texture(samplerColor, o_uv);
    uFragColor = color;
}

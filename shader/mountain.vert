#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

// Input vertex position
layout (location = 0) in vec4 pos;

// Input vertex color
layout (location = 1) in vec4 color;

// Uniform buffer object containing the Model-View-Projection (MVP) matrix and the time-based offset
layout (binding = 0) uniform UniformBufferObject {
    mat4 mvp;
    float offset;
} ubo;

// Output color to be passed to the fragment shader
layout (location = 0) out vec4 o_color;

// The main function executes once per vertex.
// It computes procedural height based on world position and time offset,
// updates the vertex color, and transforms the position using the MVP matrix.
void main() {
    float px = pos.x;
    float pz = pos.z;
    float world_z = pz + ubo.offset;

    // Ratio to determine amount of mountain
    float distance_from_valley = abs(abs(px) - 16.0) / 16.0;

    // Base height from Perlin-like noise (simplified with trig functions)
    float height = sin(world_z * 0.2) * 2.0 + cos(world_z * 0.5) * 1.0;

    // Shape into a ridge
    float ridge_factor = max(1.0 - distance_from_valley, 0.0);
    height = height * ridge_factor + ridge_factor * 5.0;

    // Add details
    height += sin(px * 1.5 + world_z * 0.8) * 0.5;
    height += cos(px * 3.0 - world_z * 2.0) * 0.25;

    // Height-based coloring
    vec4 final_color;
    if (height > 4.5) {
        // Snow
        final_color = vec4(0.9, 0.9, 0.95, 1.0);
    } else if (height > 2.0) {
        // Rock
        final_color = vec4(0.5, 0.45, 0.45, 1.0);
    } else {
        // Grass / Valley
        final_color = vec4(0.2, 0.5, 0.2, 1.0);
    }

    o_color = final_color;
    gl_Position = ubo.mvp * vec4(px, height, pz, 1.0);
}

#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

// Input vertex position
layout (location = 0) in vec4 pos;

// Uniform buffer object containing the Model-View-Projection (MVP) matrix and the time-based offset
layout (binding = 0) uniform UniformBufferObject {
    mat4 mvp;
    float offset;
} ubo;

// Output color to be passed to the fragment shader
layout (location = 0) out vec4 o_color;

// Constants to control terrain height generation
const float valley_width = 24.0;
const float mountain_scale = 9.0;

// Calculate height of a given vertex
float get_height(float px, float pz, float world_z) {
    // Base height from Perlin-like noise (simplified with trig functions)
    float modified_world_z = world_z + sin(px) * 3.0;                       // Blend some x into the base noise so the two mountain ridges don't look so similar
    float height = sin(modified_world_z * 0.2) * 2.0 + cos(modified_world_z * 0.5) * 1.0;

    // Add height to form the mountain ridge
    float mountain_ratio = abs(abs(px) - valley_width) / valley_width;      // closer to 0 -> mountains taller
    float ridge_factor = max(1.0 - mountain_ratio, 0.0);
    height = height * ridge_factor + ridge_factor * mountain_scale;         // Increase the amount of noise height when the mountain is taller

    // Add details
    height += sin(px * 1.5 + world_z * 0.8) * 0.2;
    height += cos(px * 1.0 - world_z * 2.0) * 0.15;
    height -= sin(-px * 0.9 + world_z * 1.3) * 0.1;

    return height;
}

// The main function executes once per vertex.
// It computes procedural height based on world position and time offset,
// updates the vertex color, and transforms the position using the MVP matrix.
void main() {
    float px = pos.x;
    float pz = pos.z;
    float world_z = pz + ubo.offset;

    if (pos.w > 1.5) {
        // Cloud rendering logic
        float cloud_density = sin(px * 0.15 + world_z * 0.1) * 0.5
                            + cos(px * 0.05 - world_z * 0.15) * 0.5
                            + sin(px * 0.3 + world_z * 0.2) * 0.25;

        vec4 final_color = vec4(1.0, 1.0, 1.0, 0.0);
        float cloud_height = 15.0;
        if (cloud_density > 0.3) {
            final_color.a = cloud_density;
            cloud_height += (cloud_density - 0.3) * 5.0;
        }

        o_color = final_color;
        // Place clouds high up
        gl_Position = ubo.mvp * vec4(px, cloud_height, pz, 1.0);
    } else {
        // Mountain terrain logic
        float height = get_height(px, pz, world_z);

        // Height-based coloring
        vec4 final_color;
        if (height > 9.5) {
            // Snow
            final_color = vec4(0.9, 0.9, 0.95, 1.0);
        } else if (height > 5.0) {
            // Rock
            final_color = vec4(0.5, 0.45, 0.45, 1.0);
        } else {
            // Grass / Valley
            float noise = (sin(px * 1.5 + world_z * 0.8) + cos(px * 3.0 - world_z * 2.0)) * 0.05;
            final_color = vec4(0.2 + noise, 0.5 + noise, 0.2 + noise, 1.0);
        }

        // Calculate normals using approximate gradient
        float eps = 0.1;
        float h_x = get_height(px + eps, pz, world_z);
        float h_z = get_height(px, pz + eps, world_z + eps);

        vec3 p = vec3(px, height, pz);
        vec3 p_x = vec3(px + eps, h_x, pz);
        vec3 p_z = vec3(px, h_z, pz + eps);

        vec3 normal = normalize(cross(p_z - p, p_x - p));

        // Calculate simple directional lighting
        vec3 lightDir = normalize(vec3(-1.0, 1.0, -0.5));
        float diffuse = max(dot(normal, lightDir), 0.0);
        float ambient = 0.3;

        final_color.rgb = final_color.rgb * (diffuse + ambient);

        // Calculate horizon fog
        float dist = length(vec2(px, pz));
        float fog_factor = smoothstep(100.0, 520.0, dist);
        vec4 fog_color = vec4(0.3, 0.5, 0.8, 1.0);

        final_color = mix(final_color, fog_color, fog_factor);

        o_color = final_color;
        gl_Position = ubo.mvp * vec4(px, height, pz, 1.0);
    }
}

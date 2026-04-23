use crate::Vertex;

pub fn generate_mountain_ridge() -> (Vec<Vertex>, Vec<u32>) {
    let width = 512;
    let depth = 256;

    let mut vertices = Vec::with_capacity(width * depth);
    let mut indices = Vec::with_capacity((width - 1) * (depth - 1) * 6);

    let scale = 0.5;

    for z in 0..depth {
        for x in 0..width {
            let px = (x as f32 - width as f32 / 2.0) * scale;
            let pz = (z as f32) * scale;

            // Procedural mountain ridge using sine/cosine
            // Main ridge in the center
            let distance_from_center = px.abs() / 16.0;

            // Base height from Perlin-like noise (simplified with trig functions)
            let mut height = (pz * 0.2).sin() * 2.0 + (pz * 0.5).cos() * 1.0;

            // Shape into a ridge
            let ridge_factor = (1.0 - distance_from_center).max(0.0);
            height = height * ridge_factor + ridge_factor * 5.0;

            // Add details
            height += (px * 1.5 + pz * 0.8).sin() * 0.5;
            height += (px * 3.0 - pz * 2.0).cos() * 0.25;

            // Height-based coloring
            let color = if height > 4.5 {
                // Snow
                [0.9, 0.9, 0.95, 1.0]
            } else if height > 2.0 {
                // Rock
                [0.5, 0.45, 0.45, 1.0]
            } else {
                // Grass / Valley
                [0.2, 0.5, 0.2, 1.0]
            };

            vertices.push(Vertex {
                pos: [px, height, pz, 1.0],
                color,
            });
        }
    }

    // Generate indices
    for z in 0..depth - 1 {
        for x in 0..width - 1 {
            let top_left = (z * width + x) as u32;
            let top_right = top_left + 1;
            let bottom_left = ((z + 1) * width + x) as u32;
            let bottom_right = bottom_left + 1;

            // First triangle
            indices.push(top_left);
            indices.push(bottom_left);
            indices.push(top_right);

            // Second triangle
            indices.push(top_right);
            indices.push(bottom_left);
            indices.push(bottom_right);
        }
    }

    (vertices, indices)
}

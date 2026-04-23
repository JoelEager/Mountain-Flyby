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

            // Note: The height (pos.y) and color values are entirely computed
            // and overridden in the vertex shader (shader/color/color.vert).
            // We only need to provide the X and Z grid coordinates here.
            vertices.push(Vertex {
                pos: [px, 0.0, pz, 1.0],
                color: [0.0, 0.0, 0.0, 0.0],
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

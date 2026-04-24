use crate::Vertex;

pub fn generate_terrain() -> (Vec<Vertex>, Vec<u32>) {
    // The mesh will have this many vertices per side
    let width = 256;
    let depth = 1024;

    // The vertices will be spaced by this distance
    let scale = 0.5;

    let mut vertices = Vec::with_capacity(width * depth);
    let mut indices = Vec::with_capacity((width - 1) * (depth - 1) * 6);

    let w = 1.0;
    for z in 0..depth {
        for x in 0..width {
            let px = (x as f32 - width as f32 / 2.0) * scale;
            let pz = -(z as f32) * scale;

            // Note: The height (pos.y) and color values are entirely computed in the vertex shader (shader/landscape.vert).
            // We only need to provide the X and Z grid coordinates here.
            vertices.push(Vertex {
                pos: [px, 0.0, pz, w],
            });
        }
    }

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

pub fn generate_clouds() -> (Vec<Vertex>, Vec<u32>) {
    // The mesh will have this many vertices per side
    let width = 256;
    let depth = 1024;

    // The vertices will be spaced by this distance
    let scale = 0.5;

    let mut vertices = Vec::with_capacity(width * depth);
    let mut indices = Vec::with_capacity((width - 1) * (depth - 1) * 6);

    let w = 2.0;
    for z in 0..depth {
        for x in 0..width {
            let px = (x as f32 - width as f32 / 2.0) * scale;
            let pz = -(z as f32) * scale;

            vertices.push(Vertex {
                pos: [px, 0.0, pz, w],
            });
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_landscape_generation() {
        let (terrain_vertices, terrain_indices) = generate_terrain();
        let (cloud_vertices, cloud_indices) = generate_clouds();

        let width = 256;
        let depth = 1024;

        assert_eq!(terrain_vertices.len(), width * depth);
        assert_eq!(terrain_indices.len(), (width - 1) * (depth - 1) * 6);
        assert_eq!(cloud_vertices.len(), width * depth);
        assert_eq!(cloud_indices.len(), (width - 1) * (depth - 1) * 6);

        // Check if all indices are valid
        for &index in &terrain_indices {
            assert!(
                (index as usize) < terrain_vertices.len(),
                "Index {} is out of bounds (max {})",
                index,
                terrain_vertices.len()
            );
        }
        for &index in &cloud_indices {
            assert!(
                (index as usize) < cloud_vertices.len(),
                "Index {} is out of bounds (max {})",
                index,
                cloud_vertices.len()
            );
        }

        // Check vertex w components
        for i in 0..(width * depth) {
            assert_eq!(
                terrain_vertices[i].pos[3], 1.0,
                "Vertex {} should be ground (w=1.0)",
                i
            );
            assert_eq!(
                cloud_vertices[i].pos[3], 2.0,
                "Vertex {} should be clouds (w=2.0)",
                i
            );
        }

        // Check some coordinate ranges
        let scale = 0.5;
        let max_x = (width as f32 - 1.0 - width as f32 / 2.0) * scale;
        let min_x = (0.0 - width as f32 / 2.0) * scale;
        let min_z = -(depth as f32 - 1.0) * scale;
        let max_z = 0.0;

        for (i, v) in terrain_vertices.iter().enumerate() {
            assert!(
                v.pos[0] >= min_x - 0.0001 && v.pos[0] <= max_x + 0.0001,
                "Vertex {} X coord {} out of range [{}, {}]",
                i,
                v.pos[0],
                min_x,
                max_x
            );
            assert!(
                v.pos[1] == 0.0,
                "Vertex {} Y coord {} should be 0.0",
                i,
                v.pos[1]
            );
            assert!(
                v.pos[2] >= min_z - 0.0001 && v.pos[2] <= max_z + 0.0001,
                "Vertex {} Z coord {} out of range [{}, {}]",
                i,
                v.pos[2],
                min_z,
                max_z
            );
        }
    }

    #[test]
    fn test_vertex_separation() {
        let (terrain_vertices, _) = generate_terrain();
        let (cloud_vertices, _) = generate_clouds();

        let width = 256;
        let depth = 1024;

        // Ensure ground and cloud vertices at the same grid position have same X, Z but different W
        for z in 0..depth {
            for x in 0..width {
                let idx = z * width + x;

                assert_eq!(terrain_vertices[idx].pos[0], cloud_vertices[idx].pos[0]);
                assert_eq!(terrain_vertices[idx].pos[1], cloud_vertices[idx].pos[1]);
                assert_eq!(terrain_vertices[idx].pos[2], cloud_vertices[idx].pos[2]);
                assert_eq!(terrain_vertices[idx].pos[3], 1.0);
                assert_eq!(cloud_vertices[idx].pos[3], 2.0);
            }
        }
    }
}

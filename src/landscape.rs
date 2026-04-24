use crate::Vertex;

pub fn generate_landscape() -> (Vec<Vertex>, Vec<u32>) {
    let width = 256;
    let depth = 1024;

    let mut vertices = Vec::with_capacity(width * depth * 2);
    let mut indices = Vec::with_capacity((width - 1) * (depth - 1) * 6 * 2);

    let scale = 0.5;

    // Generate ground and cloud vertices
    for is_cloud in 0..2 {
        let w = is_cloud as f32 + 1.0;
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
    }

    // Generate ground and cloud indices
    for is_cloud in 0..2 {
        let offset = is_cloud as u32 * (width * depth) as u32;
        for z in 0..depth - 1 {
            for x in 0..width - 1 {
                let top_left = offset + (z * width + x) as u32;
                let top_right = top_left + 1;
                let bottom_left = offset + ((z + 1) * width + x) as u32;
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
    }

    (vertices, indices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_landscape_generation() {
        let (vertices, indices) = generate_landscape();

        let width = 256;
        let depth = 1024;
        let num_grids = 2;

        assert_eq!(vertices.len(), width * depth * num_grids);
        assert_eq!(indices.len(), (width - 1) * (depth - 1) * 6 * num_grids);

        // Check if all indices are valid
        for &index in &indices {
            assert!((index as usize) < vertices.len(), "Index {} is out of bounds (max {})", index, vertices.len());
        }

        // Check vertex w components
        for i in 0..(width * depth) {
            assert_eq!(vertices[i].pos[3], 1.0, "Vertex {} should be ground (w=1.0)", i);
        }
        for i in (width * depth)..(width * depth * 2) {
            assert_eq!(vertices[i].pos[3], 2.0, "Vertex {} should be clouds (w=2.0)", i);
        }

        // Check some coordinate ranges
        let scale = 0.5;
        let max_x = (width as f32 - 1.0 - width as f32 / 2.0) * scale;
        let min_x = (0.0 - width as f32 / 2.0) * scale;
        let min_z = -(depth as f32 - 1.0) * scale;
        let max_z = 0.0;

        for (i, v) in vertices.iter().enumerate() {
            assert!(v.pos[0] >= min_x - 0.0001 && v.pos[0] <= max_x + 0.0001, "Vertex {} X coord {} out of range [{}, {}]", i, v.pos[0], min_x, max_x);
            assert!(v.pos[1] == 0.0, "Vertex {} Y coord {} should be 0.0", i, v.pos[1]);
            assert!(v.pos[2] >= min_z - 0.0001 && v.pos[2] <= max_z + 0.0001, "Vertex {} Z coord {} out of range [{}, {}]", i, v.pos[2], min_z, max_z);
        }
    }

    #[test]
    fn test_vertex_separation() {
        let (vertices, _) = generate_landscape();
        let width = 256;
        let depth = 1024;

        // Ensure ground and cloud vertices at the same grid position have same X, Z but different W
        for z in 0..depth {
            for x in 0..width {
                let ground_idx = z * width + x;
                let cloud_idx = ground_idx + width * depth;

                assert_eq!(vertices[ground_idx].pos[0], vertices[cloud_idx].pos[0]);
                assert_eq!(vertices[ground_idx].pos[1], vertices[cloud_idx].pos[1]);
                assert_eq!(vertices[ground_idx].pos[2], vertices[cloud_idx].pos[2]);
                assert_eq!(vertices[ground_idx].pos[3], 1.0);
                assert_eq!(vertices[cloud_idx].pos[3], 2.0);
            }
        }
    }
}

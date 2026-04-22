use crate::Vertex;
use tobj;

/// Parses the `ferris.obj` file and extracts the vertex data (positions and UVs)
/// and the index data required for rendering the 3D model.
/// It uses the `tobj` crate to load the OBJ file and triangulate the mesh.
pub fn parse_ferris_obj() -> (Vec<Vertex>, Vec<u32>) {
    let (models, _) = tobj::load_obj(
        "assets/ferris.obj",
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ignore_points: true,
            ignore_lines: true,
        },
    )
    .expect("Failed to load OBJ file");

    let mut vertices = Vec::new();
    let mut index_buffer_data = Vec::new();

    for model in models {
        let mesh = &model.mesh;
        let num_vertices = mesh.positions.len() / 3;
        let vertex_offset = vertices.len() as u32;

        for i in 0..num_vertices {
            let x = mesh.positions[i * 3];
            let y = mesh.positions[i * 3 + 1];
            let z = mesh.positions[i * 3 + 2];

            // Extract UV coordinates if available. The V coordinate is flipped (1.0 - v)
            // because Vulkan's texture coordinate system has Y pointing downwards, whereas
            // typical OBJ files assume Y points upwards.
            let uv = if mesh.texcoords.len() >= num_vertices * 2 {
                [mesh.texcoords[i * 2], 1.0 - mesh.texcoords[i * 2 + 1]]
            } else {
                [0.0, 0.0]
            };

            vertices.push(Vertex {
                pos: [x, y, z, 1.0],
                uv,
            });
        }

        for index in &mesh.indices {
            index_buffer_data.push(*index + vertex_offset);
        }
    }

    (vertices, index_buffer_data)
}

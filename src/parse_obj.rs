use crate::Vertex;
use tobj;

/// Parses the `tree.obj` file and extracts the vertex data (positions and colors)
/// and the index data required for rendering the 3D model.
/// It uses the `tobj` crate to load the OBJ file and triangulate the mesh.
pub fn parse_tree_obj() -> (Vec<Vertex>, Vec<u32>) {
    let (models, materials) = tobj::load_obj(
        "assets/tree.obj",
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ignore_points: true,
            ignore_lines: true,
        },
    )
    .expect("Failed to load OBJ file");

    let materials = materials.expect("Failed to load materials");

    let mut vertices = Vec::new();
    let mut index_buffer_data = Vec::new();

    for model in models {
        let mesh = &model.mesh;
        let num_vertices = mesh.positions.len() / 3;
        let vertex_offset = vertices.len() as u32;

        let material_id = mesh.material_id.expect("Model is missing material ID");
        let material = &materials[material_id];

        let color = if material.name == "bark" {
            [0.55, 0.27, 0.07, 1.0] // Brown color for bark
        } else if material.name == "leaf" {
            let kd = material.diffuse.unwrap_or([0.0, 1.0, 0.0]);
            [kd[0], kd[1], kd[2], 1.0]
        } else {
            [1.0, 1.0, 1.0, 1.0]
        };

        for i in 0..num_vertices {
            let x = mesh.positions[i * 3];
            let y = mesh.positions[i * 3 + 1];
            let z = mesh.positions[i * 3 + 2];

            vertices.push(Vertex {
                pos: [x, y, z, 1.0],
                color,
            });
        }

        for index in &mesh.indices {
            index_buffer_data.push(*index + vertex_offset);
        }
    }

    (vertices, index_buffer_data)
}

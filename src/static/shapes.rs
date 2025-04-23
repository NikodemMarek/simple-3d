use crate::types::{mesh::Mesh, vertex::Vertex};

pub fn cube() -> Mesh {
    Mesh::new(
        [
            Vertex::textured((1.0, 1.0, 1.0), (0.0, 0.0)), // 0 - Front top right
            Vertex::textured((1.0, -1.0, 1.0), (0.0, 0.0)), // 1 - Front bottom right
            Vertex::textured((-1.0, -1.0, 1.0), (0.0, 0.0)), // 2 - Front bottom left
            Vertex::textured((-1.0, 1.0, 1.0), (0.0, 0.0)), // 3 - Front top left
            Vertex::textured((1.0, 1.0, -1.0), (1.0, 0.0)), // 4 - Back top right
            Vertex::textured((1.0, -1.0, -1.0), (1.0, 0.0)), // 5 - Back bottom right
            Vertex::textured((-1.0, -1.0, -1.0), (1.0, 0.0)), // 6 - Back bottom left
            Vertex::textured((-1.0, 1.0, -1.0), (1.0, 0.0)), // 7 - Back top left
        ],
        &[
            // Front face (z = +1)
            (0, 2, 1),
            (0, 3, 2),
            // Back face (z = -1)
            (6, 5, 4),
            (7, 6, 4),
            // Right face (x = +1)
            (0, 4, 5),
            (0, 5, 1),
            // Left face (x = -1)
            (7, 6, 2),
            (7, 2, 3),
            // Top face (y = +1)
            (4, 7, 0),
            (7, 3, 0),
            // Bottom face (y = -1)
            (1, 5, 6),
            (1, 6, 2),
        ],
    )
}

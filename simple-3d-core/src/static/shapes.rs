use crate::types::{mesh::Mesh, vertex::Vertex};

pub fn cube() -> Mesh {
    Mesh::new(
        [
            Vertex::textured((1.0, 1.0, 1.0), (1.0, 1.0)), // 0 - Front top right
            Vertex::textured((1.0, -1.0, 1.0), (1.0, 0.0)), // 1 - Front bottom right
            Vertex::textured((-1.0, -1.0, 1.0), (0.0, 0.0)), // 2 - Front bottom left
            Vertex::textured((-1.0, 1.0, 1.0), (0.0, 1.0)), // 3 - Front top left
            Vertex::textured((1.0, 1.0, -1.0), (0.0, 1.0)), // 4 - Back top right
            Vertex::textured((1.0, -1.0, -1.0), (0.0, 0.0)), // 5 - Back bottom right
            Vertex::textured((-1.0, -1.0, -1.0), (1.0, 0.0)), // 6 - Back bottom left
            Vertex::textured((-1.0, 1.0, -1.0), (1.0, 1.0)), // 7 - Back top left
        ],
        &[
            // Front face (z = +1)
            (0, 1, 2), // Top Right, Bottom Right, Bottom Left
            (0, 2, 3), // Top Right, Bottom Left, Top Left
            // Back face (z = -1)
            (4, 6, 5), // Top Right, Bottom Left, Bottom Right
            (4, 7, 6), // Top Right, Top Left, Bottom Left
            // Right face (x = +1)
            (0, 5, 1), // Top Right, Bottom Right, Bottom Left
            (0, 4, 5), // Top Right, Top Left, Bottom Right
            // Left face (x = -1)
            (3, 2, 6), // Top Right, Bottom Left, Bottom Left
            (3, 6, 7), // Top Right, Bottom Left, Top Left
            // Top face (y = +1)
            (0, 3, 7), // Front Top Right, Front Top Left, Back Top Left
            (0, 7, 4), // Front Top Right, Back Top Left, Back Top Right
            // Bottom face (y = -1)
            (1, 6, 2), // Front Bottom Right, Back Bottom Left, Front Bottom Left
            (1, 5, 6), // Front Bottom Right, Back Bottom Right, Back Bottom Left
        ],
    )
}

use crate::types::{mesh::Mesh, vertex::Vertex};

pub fn cube() -> Mesh {
    Mesh::new(
        [
            // Front face (z = +1)
            Vertex::textured((1.0, 1.0, 1.0), (1.0, 1.0)), // 0 - Front top right
            Vertex::textured((1.0, -1.0, 1.0), (1.0, 0.0)), // 1 - Front bottom right
            Vertex::textured((-1.0, -1.0, 1.0), (0.0, 0.0)), // 2 - Front bottom left
            Vertex::textured((-1.0, 1.0, 1.0), (0.0, 1.0)), // 3 - Front top left
            // Back face (z = -1)
            Vertex::textured((1.0, 1.0, -1.0), (1.0, 1.0)), // 4 - Back top right
            Vertex::textured((1.0, -1.0, -1.0), (1.0, 0.0)), // 5 - Back bottom right
            Vertex::textured((-1.0, -1.0, -1.0), (0.0, 0.0)), // 6 - Back bottom left
            Vertex::textured((-1.0, 1.0, -1.0), (0.0, 1.0)), // 7 - Back top left
            // Right face (x = +1)
            Vertex::textured((1.0, 1.0, 1.0), (1.0, 1.0)), // 8 - Right top front
            Vertex::textured((1.0, -1.0, 1.0), (1.0, 0.0)), // 9 - Right bottom front
            Vertex::textured((1.0, -1.0, -1.0), (0.0, 0.0)), // 10 - Right bottom back
            Vertex::textured((1.0, 1.0, -1.0), (0.0, 1.0)), // 11 - Right top back
            // Left face (x = -1)
            Vertex::textured((-1.0, 1.0, 1.0), (1.0, 1.0)), // 12 - Left top front
            Vertex::textured((-1.0, -1.0, 1.0), (1.0, 0.0)), // 13 - Left bottom front
            Vertex::textured((-1.0, -1.0, -1.0), (0.0, 0.0)), // 14 - Left bottom back
            Vertex::textured((-1.0, 1.0, -1.0), (0.0, 1.0)), // 15 - Left top back
            // Top face (y = +1)
            Vertex::textured((1.0, 1.0, 1.0), (1.0, 1.0)), // 16 - Top front right
            Vertex::textured((-1.0, 1.0, 1.0), (0.0, 1.0)), // 17 - Top front left
            Vertex::textured((-1.0, 1.0, -1.0), (0.0, 0.0)), // 18 - Top back left
            Vertex::textured((1.0, 1.0, -1.0), (1.0, 0.0)), // 19 - Top back right
            // Bottom face (y = -1)
            Vertex::textured((1.0, -1.0, 1.0), (1.0, 0.0)), // 20 - Bottom front right
            Vertex::textured((-1.0, -1.0, 1.0), (0.0, 0.0)), // 21 - Bottom front left
            Vertex::textured((-1.0, -1.0, -1.0), (0.0, 1.0)), // 22 - Bottom back left
            Vertex::textured((1.0, -1.0, -1.0), (1.0, 1.0)), // 23 - Bottom back right
        ],
        &[
            // Front face
            (0, 1, 2),
            (0, 2, 3),
            // Back face
            (4, 6, 5),
            (4, 7, 6),
            // Right face
            (8, 10, 9),
            (8, 11, 10),
            // Left face
            (12, 13, 14),
            (12, 14, 15),
            // Top face
            (16, 18, 17),
            (16, 19, 18),
            // Bottom face
            (20, 21, 22),
            (20, 22, 23),
        ],
    )
}

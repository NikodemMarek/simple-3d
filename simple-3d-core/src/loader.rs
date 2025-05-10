use crate::types::{
    mesh::{Indice, Mesh},
    pixel::Pixel,
    textures::Image,
    vector::Vector,
};

pub fn load_obj(data: &[u8]) -> Mesh {
    #[inline]
    fn vec_n<const S: usize>(reminder: std::str::SplitAsciiWhitespace) -> Vector<S> {
        reminder
            .take(S)
            .map(|s| s.parse::<f64>().unwrap())
            .collect()
    }
    #[inline]
    fn faces(reminder: std::str::SplitAsciiWhitespace) -> [(usize, usize); 3] {
        let parts = reminder
            .map(|t| {
                let parts = t
                    .split("/")
                    .map(|s| s.parse::<usize>().unwrap() - 1)
                    .collect::<Vec<_>>();
                if parts.len() == 1 {
                    (parts[0], 0)
                } else {
                    (parts[0], parts[1])
                }
            })
            .collect::<Vec<(usize, usize)>>();
        [parts[0], parts[1], parts[2]]
    }

    let file = String::from_utf8_lossy(data);

    let mut vertices = Vec::<Vector<3>>::new();
    let mut uvs = Vec::<Vector<2>>::new();
    let mut indices = Vec::<Indice>::new();
    let mut texture = "none";

    for line in file.lines() {
        let mut parts = line.split_ascii_whitespace();
        match parts.next() {
            Some("v") => vertices.push(vec_n::<3>(parts)),
            Some("vt") => uvs.push(vec_n::<2>(parts)),
            Some("f") => {
                let indexes = faces(parts);
                indices.push((indexes[0], indexes[1], indexes[2]));
            }
            Some("usemtl") => {
                texture = parts.next().unwrap();
            }
            Some(_) | None => continue,
        }
    }

    Mesh::new(vertices, uvs, &indices, texture.into())
}

pub fn load_image(data: &[u8]) -> Image {
    use image::{GenericImageView, load_from_memory};

    let image = load_from_memory(data).unwrap();
    let pixels = image
        .pixels()
        .map(|(_, _, rgba)| rgba.into())
        .collect::<Box<[Pixel]>>();
    Image::load(image.width() as u32, image.height() as u32, &pixels)
}

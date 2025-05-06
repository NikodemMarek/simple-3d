use crate::{
    types::{
        mesh::Indice,
        pixel::Pixel,
        textures::Texture,
        triangle::{Triangle, TriangleIterator},
    },
    vector::Vector,
};

pub fn rasterize<'a, I>(
    texture: &'a Texture,
    triangles: TriangleIterator<'a, I>,
) -> impl Iterator<Item = ((u32, u32), f32, Pixel)>
where
    I: Iterator<Item = &'a Indice>,
{
    triangles.flat_map(|triangle| triangle_points(texture, triangle))
}

pub fn triangle_points(
    texture: &Texture,
    triangle @ Triangle(a, b, c): Triangle,
) -> impl Iterator<Item = ((u32, u32), f32, Pixel)> {
    #[inline]
    fn det(a: Vector<3>, b: Vector<3>, c: Vector<3>) -> f64 {
        a[0] * (b[1] - c[1]) + b[0] * (c[1] - a[1]) + c[0] * (a[1] - b[1])
    }

    #[inline]
    fn bounds(Triangle(a, b, c): &Triangle) -> (u32, u32, u32, u32) {
        let l = a.position[0].min(b.position[0]).min(c.position[0]).floor() as u32;
        let r = a.position[0].max(b.position[0]).max(c.position[0]).ceil() as u32;
        let t = a.position[1].min(b.position[1]).min(c.position[1]).floor() as u32;
        let b = a.position[1].max(b.position[1]).max(c.position[1]).ceil() as u32;
        (l, t, r, b)
    }

    #[inline]
    fn points(
        det_abc: f64,
        (l, t, r, b): (u32, u32, u32, u32),
    ) -> Box<dyn Iterator<Item = (u32, u32)>> {
        if det_abc.abs() < f64::EPSILON {
            Box::new(std::iter::empty())
        } else {
            Box::new((l..=r).flat_map(move |x| (t..=b).map(move |y| (x, y))))
        }
    }

    let det_abc = det(a.position, b.position, c.position);
    points(det_abc, bounds(&triangle)).filter_map(move |(x, y)| {
        let p = (x as f64, y as f64, 0.0).into();
        let det_abp = det(a.position, b.position, p);
        let det_bcp = det(b.position, c.position, p);
        let det_cap = det(c.position, a.position, p);

        let alpha = det_bcp / det_abc;
        let beta = det_cap / det_abc;
        let gamma = det_abp / det_abc;

        const EPSILON: f64 = 1e-10;
        if alpha >= -EPSILON && beta >= -EPSILON && gamma >= -EPSILON {
            let z_a = 1.0 / a.position[2];
            let z_b = 1.0 / b.position[2];
            let z_c = 1.0 / c.position[2];

            let inv_z = alpha * z_a + beta * z_b + gamma * z_c;
            let z = 1.0 / inv_z;

            let u_over_z =
                alpha * (a.uv[0] * z_a) + beta * (b.uv[0] * z_b) + gamma * (c.uv[0] * z_c);
            let v_over_z =
                alpha * (a.uv[1] * z_a) + beta * (b.uv[1] * z_b) + gamma * (c.uv[1] * z_c);

            let u = u_over_z / inv_z;
            let v = v_over_z / inv_z;

            let tex_width = texture.width() as f64;
            let tex_height = texture.height() as f64;

            let tex_x = (u.clamp(0.0, 1.0) * (tex_width - 1.0)).round() as u32;
            let tex_y = (v.clamp(0.0, 1.0) * (tex_height - 1.0)).round() as u32;

            let tex_x = tex_x.min(texture.width() - 1);
            let tex_y = tex_y.min(texture.height() - 1);

            let pixel = texture.get(tex_x, tex_y);

            Some(((x, y), z as f32, pixel))
        } else {
            None
        }
    })
}

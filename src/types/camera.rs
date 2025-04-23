use super::{matrix::Matrix, vector::Vector};

#[derive(Debug, Clone)]
pub struct Camera {
    position: Vector<3>,
    target: Vector<3>,
    up: Vector<3>,

    properties: CameraProperties,

    transformation: Matrix<4, 4>,
}

impl Camera {
    pub fn new(properties: CameraProperties) -> Self {
        let position = (0.0, 0.0, 5.0).into();
        let target = (0.0, 0.0, 0.0).into();
        let up = (0.0, 1.0, 0.0).into();

        let transformation = properties.transformation_matrix().clone()
            * Self::calculate_view_matrix(position, target, up);

        Self {
            position: (0.0, 0.0, 5.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: (0.0, 1.0, 0.0).into(),
            properties,
            transformation,
        }
    }
    pub fn inherit(
        Camera {
            position,
            target,
            up,
            ..
        }: Camera,
        properties: CameraProperties,
    ) -> Self {
        let transformation =
            Self::calculate_transformation_matrix(&properties, position, target, up);
        Self {
            position,
            target,
            up,
            properties,
            transformation,
        }
    }

    fn calculate_view_matrix(
        position: Vector<3>,
        target: Vector<3>,
        up: Vector<3>,
    ) -> Matrix<4, 4> {
        let f = (target - position).normalize();
        let r = up.cross(f).normalize();
        let u = f.cross(r);
        let p = position;
        [
            [r[0], r[1], r[2], -r.dot(p)],
            [u[0], u[1], u[2], -u.dot(p)],
            [-f[0], -f[1], -f[2], f.dot(p)],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    }
    #[inline]
    fn calculate_transformation_matrix(
        properties: &CameraProperties,
        position: Vector<3>,
        target: Vector<3>,
        up: Vector<3>,
    ) -> Matrix<4, 4> {
        properties.transformation_matrix().clone()
            * Self::calculate_view_matrix(position, target, up)
    }

    #[inline]
    pub fn transformation_matrix(&self) -> &Matrix<4, 4> {
        &self.transformation
    }
    pub fn radius(&self) -> f64 {
        (self.position - self.target).magnitude()
    }

    pub fn r#move(&mut self, vector: impl Into<Vector<3>>) {
        self.position += vector.into();
        self.transformation = Self::calculate_transformation_matrix(
            &self.properties,
            self.position,
            self.target,
            self.up,
        )
    }
    pub fn look(&mut self, target: impl Into<Vector<3>>) {
        self.target = target.into();
        self.transformation = Self::calculate_transformation_matrix(
            &self.properties,
            self.position,
            self.target,
            self.up,
        )
    }
}

#[derive(Debug, Clone)]
pub struct CameraProperties(Matrix<4, 4>);

impl CameraProperties {
    pub fn new(fov: f64, aspect_ratio: f64, near: f64, far: f64) -> Self {
        Self(Self::calculate_projection_matrix(
            fov,
            aspect_ratio,
            near,
            far,
        ))
    }

    fn calculate_projection_matrix(
        fov: f64,
        aspect_ratio: f64,
        near: f64,
        far: f64,
    ) -> Matrix<4, 4> {
        let f = 1.0 / f64::tan(fov / 2.0);
        [
            [f / aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [
                0.0,
                0.0,
                (near + far) / (near - far),
                (2.0 * near * far) / (near - far),
            ],
            [0.0, 0.0, -1.0, 0.0],
        ]
        .into()
    }

    #[inline]
    fn transformation_matrix(&self) -> &Matrix<4, 4> {
        &self.0
    }
}

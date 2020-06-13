use nalgebra::{Vector3};

pub struct Camera {
    pub focal_length: f32,
    pub origin: Vector3<f32>,
    pub horizontal: Vector3<f32>, // horizontal image vector
    pub vertical: Vector3<f32>
}

impl Camera {
    pub fn new() -> Camera {
        return Camera {
            focal_length: 1.,
            origin: Vector3::new(0., 0., 0.),
            horizontal: Vector3::new(crate::constants::VIEWPORT_WIDTH as f32, 0., 0.),
            vertical: Vector3::new(0., crate::constants::VIEWPORT_HEIGHT as f32, 0.)
        };
    }

    pub fn get_lower_left_corner(&self) -> Vector3<f32> {
        return self.origin - self.horizontal/2. - self.vertical/2. - Vector3::new(0., 0., self.focal_length);
    }
}
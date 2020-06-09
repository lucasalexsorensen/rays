use nalgebra::{Vector3};

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        return Ray {
            origin,
            direction
        };
    }

    pub fn get_color(&self) -> Vector3<f32> {
        let unit_direction = self.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.);
        return (1.0 - t) * Vector3::new(1., 1., 1.) + t * Vector3::new(0.5, 0.7, 1.0);
    }
}
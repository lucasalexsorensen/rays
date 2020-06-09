use image::{save_buffer, ColorType};
mod camera;
mod ray;
use camera::{Camera};
use ray::{Ray};

pub const ASPECT_RATIO: f32 = 16. / 9.;
pub const IMAGE_WIDTH: u32 = 400;
pub const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
pub const VIEWPORT_HEIGHT: f32 = 2.;
pub const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;

fn main() {
    let cam = Camera::new();
    let corner = cam.get_lower_left_corner();

    let mut pixels = Vec::new();

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f32) / (IMAGE_WIDTH - 1) as f32;
            let v = (j as f32) / (IMAGE_HEIGHT - 1) as f32;
            let ray = Ray::new(cam.origin, corner + u*cam.horizontal + v*cam.vertical);
            let color = ray.get_color();

            pixels.push((255. * color.x) as u8);
            pixels.push((255. * color.y) as u8);
            pixels.push((255. * color.z) as u8);
        }
    }

    save_buffer("out.png", pixels.as_slice(), IMAGE_WIDTH, IMAGE_HEIGHT, ColorType::Rgb8).unwrap()
}

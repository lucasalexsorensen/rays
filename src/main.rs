use image::{save_buffer, ColorType};
mod camera;
mod ray;
mod hittables;
use camera::{Camera};
use ray::{Ray};
use hittables::{Sphere, ray_color, HittableCollection};
use std::env;

mod constants {
    pub const ASPECT_RATIO: f32 = 16. / 9.;
    pub const IMAGE_WIDTH: u32 = 1200;
    pub const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    pub const VIEWPORT_HEIGHT: f32 = 2.;
    pub const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
}

fn main() {
    let cam = Camera::new();
    let corner = cam.get_lower_left_corner();

    let mut pixels = Vec::new();

    let mut hittables = HittableCollection::new();
    hittables.add(Sphere::new(0., 0., -1., 0.5));

    for j in (0..constants::IMAGE_HEIGHT).rev() {
        for i in 0..constants::IMAGE_WIDTH {
            let u = (i as f32) / (constants::IMAGE_WIDTH - 1) as f32;
            let v = (j as f32) / (constants::IMAGE_HEIGHT - 1) as f32;
            let ray = Ray::new(cam.origin, corner + u*cam.horizontal + v*cam.vertical);

            let color = ray_color(&ray, &hittables);

            pixels.push((255. * color.x) as u8);
            pixels.push((255. * color.y) as u8);
            pixels.push((255. * color.z) as u8);
        }
    }

    let args: Vec<String> = env::args().collect();
    let fname = match args.len() {
        0 | 1 => "out.png",
        _ => args[1].as_str()
    };

    println!("fname is {}", fname);
    save_buffer(fname, pixels.as_slice(), constants::IMAGE_WIDTH, constants::IMAGE_HEIGHT, ColorType::Rgb8).unwrap()
}

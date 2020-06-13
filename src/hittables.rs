use crate::{Ray};
use nalgebra::{Vector3};

pub fn ray_color(ray: &Ray, world: &HittableCollection) -> Vector3<f32> {
    let result = world.trace(ray, 0., std::f32::INFINITY);
    match result {
        Some(result) => 0.5 * (result.normal + Vector3::new(1., 1., 1.)),
        None => {
            let unit_direction = ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.);
            return (1. - t) * Vector3::new(1., 1., 1.) + t*Vector3::new(0.5, 0.7, 1.0);
        }
    }
}

pub struct HittableCollection {
    pub hittables: Vec<Box<dyn Hittable>>
}

impl HittableCollection {
    pub fn new() -> HittableCollection{
        return HittableCollection {
            hittables: Vec::new()
        };
    }

    pub fn add(&mut self, hittable: Sphere) {
        self.hittables.push(Box::new(hittable));
    }

    pub fn trace(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let mut best_result: Option<HitResult> = None;
        let mut smallest_t = t_max;

        for hittable in &self.hittables {
            if let Some(result) = hittable.hit(ray, t_min, smallest_t) {
                if result.t < smallest_t {
                    smallest_t = result.t;
                    best_result = Some(result);
                }
            }
        }

        return best_result;
    }
}


pub struct HitResult {
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32
}

impl HitResult {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3<f32>) {
        let front_face = ray.direction.dot(outward_normal) < 0.;
        if front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult>;
}

////////////// SPHERE IMPLEMENTATION //////////////
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, radius: f32) -> Sphere {
        return Sphere { center: Vector3::new(x, y, z), radius };
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let oc = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        let b_half = oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant: f32 = b_half * b_half - a * c;

        if discriminant > 0. {
            let root = discriminant.sqrt();
            let mut tmp = (-b_half - root) / a;
            if tmp < t_max && tmp > t_min { // try first root
                let t = tmp;
                let p = ray.at(t);
                let outward_normal = (p - self.center) / self.radius;
                let mut result = HitResult { t, p, normal: outward_normal };
                result.set_face_normal(ray, &outward_normal);
                return Some(result);
            }
            tmp = (-b_half + root) / a;
            if tmp < t_max && tmp > t_min { // maybe second root?
                let t = tmp;
                let p = ray.at(t);
                let outward_normal = (p - self.center) / self.radius;
                let mut result = HitResult { t, p, normal: outward_normal };
                result.set_face_normal(ray, &outward_normal);
                return Some(result);
            }
        }
        return None;
    }
}

use crate::rt_classes::vec3::Vec3;

use super::hittable::{HitRecord ,Hittable};

struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius
        }
    }
}


impl Hittable for Sphere {
    fn hit(r: &crate::rt_classes::ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &HitRecord) -> bool {
        true
    }
}
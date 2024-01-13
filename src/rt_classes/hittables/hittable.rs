use crate::rt_classes::ray::Ray;
use crate::rt_classes::vec3::Vec3;

pub struct HitRecord {
    pub pnt: Vec3,
    pub normal: Vec3,
    pub t: f64
}

pub trait Hittable {
    fn hit(r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &HitRecord) -> bool; 
}
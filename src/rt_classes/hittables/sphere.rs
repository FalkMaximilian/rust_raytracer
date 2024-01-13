use crate::rt_classes::vec3::Vec3;
use crate::rt_classes::ray::Ray;
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
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc * r.direction;
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (-half_b + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.pnt = r.at(rec.t);
        let outward_normal = (rec.pnt - &self.center) / self.radius; 
        rec.set_face_normals(r, &outward_normal);
        
        true
    }
}
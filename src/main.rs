#[macro_use] extern crate impl_ops;

// For reading and opening files
use crate::rt_classes::image::Image;
use crate::rt_classes::color::Color;
use crate::rt_classes::vec3::Vec3;
use crate::rt_classes::ray::Ray;

mod config;
use config::Config;

mod rt_classes;

fn ray_color(r: Ray) -> Color {

    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &r);

    if t > 0f64 {
        let N = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
        return 0.5*Color::new(N.x + 1.0, N.y + 1.0, N.z + 1.0)
    }

    let unit_dir = &r.direction / r.direction.length();
    let a = 0.5 * (unit_dir.y + 1.0);
    (1.0-a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - center;
    let a = &r.direction.length_squared();
    let half_b = &oc * &r.direction;
    let c = &oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;

    if discriminant < 0f64 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / (2.0*a)
    }
}

fn main() {


    let config = Config::new();

    let aspect_ratio = 16.0/9.0;
    let image_width = 4096;

    let mut image_height = (image_width as f64 / (aspect_ratio)) as u32;
    if image_height == 0 {
        image_height = 1;
    }

    let mut img = Image::new(image_width, image_height);

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center_pnt = Vec3::new(0.0, 0.0, 0.0);
        

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = &viewport_u / image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    let viewport_upper_left_pnt = &camera_center_pnt - Vec3::new(0.0, 0.0, focal_length) - (&viewport_u/2f64) - (&viewport_v/2f64); 
    let pixel00_pnt = &viewport_upper_left_pnt + 0.5 * (&pixel_delta_u + &pixel_delta_v);


    for y in 0..image_height {
        for x in 0..image_width {
            let index = 3 * (y * image_width + x) as usize; 
            let pixel_center_pnt = &pixel00_pnt + (x as f64 * &pixel_delta_u) + (y as f64 * &pixel_delta_v);
            let ray_dir = &pixel_center_pnt - &camera_center_pnt;
            let ray =  Ray::new(camera_center_pnt.clone(), ray_dir);

            let pixel_color = ray_color(ray);

            img.set_pixel(index, &pixel_color);
        }
    }

    img.save();
}

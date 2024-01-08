use std::mem::Discriminant;

use rt_classes::color;
use rt_classes::vec3::Vec3;

// For reading and opening files
use crate::rt_classes::image::Image;
use crate::rt_classes::color::Color;
use crate::rt_classes::point::Point;
use crate::rt_classes::ray::Ray;

mod rt_classes;

fn ray_color(r: Ray) -> Color {

    if hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, &r) {
        return Color::new(1.0, 0.0, 0.0)
    }

    let unit_dir = &r.direction / r.direction.length();
    let a = 0.5 * (unit_dir.y + 1.0);
    (1.0-a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Point, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = r.direction.dot(&r.direction);
    let b = 2.0 * oc.dot(&r.direction);
    let c = oc.dot(&oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant >= 0.0
}

fn main() {

    let aspect_ratio = 16.0/9.0;
    let image_width = 1920;

    let mut image_height = (image_width as f64 / (aspect_ratio)) as u32;
    if image_height == 0 {
        image_height = 1;
    }

    let mut img = Image::new(image_width, image_height);

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height) as f64;
    let camera_center = Point::new(0.0, 0.0, 0.0);
        

    let viewport_u = Point::new(viewport_width, 0.0, 0.0);
    let viewport_v = Point::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = &viewport_u / image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    let viewport_upper_left = &camera_center - Point::new(0.0, 0.0, focal_length) - (&viewport_u/2f64) - (&viewport_v/2f64); 
    let pixel00_loc = &viewport_upper_left + 0.5 * (&pixel_delta_u + &pixel_delta_v);


    for y in 0..image_height {
        for x in 0..image_width {
            let index = 3 * (y * image_width + x) as usize; 
            let pixel_center = &pixel00_loc + (x as f64 * &pixel_delta_u) + (y as f64 * &pixel_delta_v);
            let ray_direction = &pixel_center - &camera_center;
            let r =  Ray::new(camera_center.clone(), ray_direction);

            let pixel_color = ray_color(r);

            img.set_pixel(index, &pixel_color);
        }
    }

    img.save();
}

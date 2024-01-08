use crate::rt_classes::point::Point;

pub struct Ray {
    pub origin: Point,
    pub direction: Point,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn at(&self, t: f64) -> Point {
        &self.origin + t*&self.direction
    }
}
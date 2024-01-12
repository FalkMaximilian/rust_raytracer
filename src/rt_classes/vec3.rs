use std::ops;

use super::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }

    pub fn as_color(&self) -> Color {
        Color::new(self.x, self.y, self.z)
    }

    pub fn unit(&self) -> Vec3 {
        self / self.length()
    }
}

impl Vec3 {
    // Cross Product
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: (self.y * other.z) - (self.z * other.y), 
            y: (self.z * other.x) - (self.x * other.z), 
            z: (self.x * other.y) - (self.y * other.x)
        } 
    }

    // Vec Length
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
}

// This macro implements addition for all possible permutations. I.e. Point and Point or Point and &Point, ...
impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z) });
impl_op_ex!(+ |a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x + b, a.y + b, a.z + b) });
impl_op_ex!(+= |a: &mut Vec3, b: &Vec3| { a.x += b.x; a.y += b.y; a.z += b.z; });
impl_op_ex!(+= |a: &mut Vec3, b: f64| { a.x += b; a.y += b; a.z += b; });

impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z) });
impl_op_ex!(- |a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x - b, a.y - b, a.z - b) });
impl_op_ex!(-= |a: &mut Vec3, b: &Vec3| { a.x -= b.x; a.y -= b.y; a.z -= b.z; });
impl_op_ex!(-= |a: &mut Vec3, b: f64| { a.x -= b; a.y -= b; a.z -= b; });

impl_op_ex!(* |a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x * b, a.y * b, a.z * b) });
impl_op_ex!(* |a: f64, b: &Vec3| -> Vec3 { Vec3::new(b.x * a, b.y * a, b.z * a) });
impl_op_ex!(* |a: &Vec3, b: &Vec3| -> f64 { (a.x * b.x) + (a.y * b.y) + (a.z * b.z) });
impl_op_ex!(*= |a: &mut Vec3, b: f64| { a.x *= b; a.y *= b; a.z *= b});

impl_op_ex!(/ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x / b.x, a.y / b.y, a.z / b.z) });
impl_op_ex!(/ |a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x / b, a.y / b, a.z / b) });
impl_op_ex!(/= |a: &mut Vec3, b: f64| { a.x /= b; a.y /= b; a.z /= b});
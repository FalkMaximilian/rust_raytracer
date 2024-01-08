use crate::rt_classes::vec3::Vec3;

#[derive(Debug, Clone, Copy)]

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            x,
            y,
            z,
        }
    }
}

impl Vec3 for Point {
    // Dot Product
    fn dot(&self, other: &Self) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    // Cross Product
    fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y*other.z - self.z*other.y, 
            y: self.z*other.x - self.x*other.z, 
            z: self.x*other.y - self.y*other.x
        } 
    }

    // Vec Length
    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
}






// Addition
impl std::ops::Add<f64> for &Point {
    type Output = Point;

    fn add(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        } 
    }
}

impl std::ops::Add<&Point> for f64 {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
        } 
    }
}

impl std::ops::Add<&Point> for &Point { 
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x, 
            y: self.y + rhs.y, 
            z: self.z + rhs.z
        }
    }
}

impl std::ops::Add<Point> for &Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign<f64> for Point {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}


impl std::ops::AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}



// Subtraction
impl std::ops::Sub<f64> for &Point {
    type Output = Point;

    fn sub(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        } 
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Sub<Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}


impl std::ops::Sub<&Point> for &Point { 
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x - rhs.x, 
            y: self.y - rhs.y, 
            z: self.z - rhs.z
        }
    }
}

impl std::ops::SubAssign<f64> for Point {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs; 
    }
}

impl std::ops::SubAssign<&Point> for Point {
    fn sub_assign(&mut self, rhs: &Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}



// Multiplication
impl std::ops::Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        } 
    }
}

impl std::ops::Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl std::ops::Mul<&Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        } 
    }
}


impl std::ops::MulAssign<f64> for Point {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}


impl std::ops::MulAssign<&Point> for Point {
    fn mul_assign(&mut self, rhs: &Point) {
       self.x *= rhs.x; 
       self.y *= rhs.y; 
       self.z *= rhs.z; 
    }
}


// Division
impl std::ops::Div<f64> for &Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        } 
    }
}

impl std::ops::Div<&Point> for &Point {
    type Output = Point;

    fn div(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        } 
    }
}

impl std::ops::Div<Point> for Point {
    type Output = Point;

    fn div(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        } 
    }
}

impl std::ops::DivAssign<f64> for Point {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl std::ops::DivAssign<&Point> for Point {
    fn div_assign(&mut self, rhs: &Point) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
use crate::rt_classes::vec3::Vec3;

#[derive(Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}


impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            r,
            g,
            b,
        }
    }
}

impl Vec3 for Color {
    // Dot product 
    fn dot(&self, other: &Self) -> f64 {
        self.r*other.r + self.g*other.g + self.b*other.b
    }

    // Cross Product
    fn cross(&self, other: &Self) -> Self {
        Self {
            r: self.g*other.b - self.b*other.g, 
            g: self.b*other.r - self.r*other.b, 
            b: self.r*other.g - self.g*other.r
        } 
    }

    // Vec Length
    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.r*self.r + self.g*self.g + self.b*self.b
    }
}




// Addition
impl std::ops::Add<f64> for &Color {
    type Output = Color;

    fn add(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r + rhs,
            g: self.g + rhs,
            b: self.b + rhs,
        } 
    }
}

impl std::ops::Add<&Color> for f64 {
    type Output = Color;

    fn add(self, rhs: &Color) -> Self::Output {
        Color {
            r: self + rhs.r,
            g: self + rhs.g,
            b: self + rhs.b,
        } 
    }
}

impl std::ops::Add<Color> for f64 {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self + rhs.r,
            g: self + rhs.g,
            b: self + rhs.b,
        } 
    }
}

impl std::ops::Add<&Color> for &Color { 
    type Output = Color;

    fn add(self, rhs: &Color) -> Self::Output {
        Color {
            r: self.r + rhs.r, 
            g: self.g + rhs.g, 
            b: self.b + rhs.b
        }
    }
}

impl std::ops::Add<Color> for Color { 
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r, 
            g: self.g + rhs.g, 
            b: self.b + rhs.b
        }
    }
}

impl std::ops::AddAssign<f64> for Color {
    fn add_assign(&mut self, rhs: f64) {
        self.r += rhs;
        self.g += rhs;
        self.b += rhs;
    }
}

impl std::ops::AddAssign<&Color> for Color {
    fn add_assign(&mut self, rhs: &Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}



// Subtraction
impl std::ops::Sub<f64> for &Color {
    type Output = Color;

    fn sub(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r - rhs,
            g: self.g - rhs,
            b: self.b - rhs,
        } 
    }
}


impl std::ops::Sub<&Color> for &Color { 
    type Output = Color;

    fn sub(self, rhs: &Color) -> Self::Output {
        Color {
            r: self.r - rhs.r, 
            g: self.g - rhs.g, 
            b: self.b - rhs.b
        }
    }
}

impl std::ops::SubAssign<f64> for Color {
    fn sub_assign(&mut self, rhs: f64) {
        self.r -= rhs;
        self.g -= rhs;
        self.b -= rhs; 
    }
}

impl std::ops::SubAssign<&Color> for Color {
    fn sub_assign(&mut self, rhs: &Color) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}



// Multiplication
impl std::ops::Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        } 
    }
}

impl std::ops::Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        } 
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        } 
    }
}

impl std::ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}


impl std::ops::MulAssign<&Color> for Color {
    fn mul_assign(&mut self, rhs: &Color) {
       self.r *= rhs.r; 
       self.g *= rhs.g; 
       self.b *= rhs.b; 
    }
}


// Division
impl std::ops::Div<f64> for &Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        } 
    }
}

impl std::ops::Div<&Color> for &Color {
    type Output = Color;

    fn div(self, rhs: &Color) -> Self::Output {
        Color {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        } 
    }
}

impl std::ops::DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

impl std::ops::DivAssign<&Color> for Color {
    fn div_assign(&mut self, rhs: &Color) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
    }
}
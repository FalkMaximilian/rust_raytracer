
pub trait Vec3 {    
    // Dot Product
    fn dot(&self, other: &Self) -> f64;

    // Cross Product
    fn cross(&self, other: &Self) -> Self;

    // Vec Length
    fn length(&self) -> f64;
    fn length_squared(&self) -> f64;
}

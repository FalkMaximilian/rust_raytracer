use std::ops;

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

impl_op_ex!(+ |a: &Color, b: &Color| -> Color { Color::new(a.r + b.r, a.g + b.g, a.b + b.b) });
impl_op_ex!(+ |a: &Color, b: f64| -> Color { Color::new(a.r + b, a.g + b, a.b + b) });
//impl_op_ex!(+ |a: f64, b: &Color| -> Color { Color::new(b.r + a, b.g + a, b.b + a) });
impl_op_ex!(+= |a: &mut Color, b: &Color| {a.r += b.r; a.g += b.g; a.b += b.b} );
impl_op_ex!(+= |a: &mut Color, b: f64| {a.r += b; a.g += b; a.b += b} );

impl_op_ex!(- |a: &Color, b: &Color| -> Color { Color::new(a.r - b.r, a.g - b.g, a.b - b.b) });
impl_op_ex!(- |a: &Color, b: f64| -> Color { Color::new(a.r - b, a.g - b, a.b - b) });
//impl_op_ex!(- |a: f64, b: &Color| -> Color { Color::new(b.r - a, b.g - a, b.b - a) });
impl_op_ex!(-= |a: &mut Color, b: &Color| {a.r -= b.r; a.g -= b.g; a.b -= b.b} );
impl_op_ex!(-= |a: &mut Color, b: f64| {a.r -= b; a.g -= b; a.b -= b} );

impl_op_ex!(* |a: &Color, b: &Color| -> Color { Color::new(a.r * b.r, a.g * b.g, a.b * b.b) });
impl_op_ex!(* |a: &Color, b: f64| -> Color { Color::new(a.r * b, a.g * b, a.b * b) });
impl_op_ex!(* |a: f64, b: &Color| -> Color { Color::new(b.r * a, b.g * a, b.b * a) });
impl_op_ex!(*= |a: &mut Color, b: f64| { a.r *= b; a.g *= b; a.b *= b; });
impl_op_ex!(*= |a: &mut Color, b: &Color| { a.r *= b.r; a.g *= b.g; a.b *= b.b; });

impl_op_ex!(/ |a: &Color, b: &Color| -> Color { Color::new(a.r / b.r, a.g / b.g, a.b / b.b)});
impl_op_ex!(/ |a: &Color, b: f64| -> Color { Color::new(a.r / b, a.g / b, a.b / b)});
impl_op_ex!(/= |a: &mut Color, b: f64| { a.r /= b; a.g /= b; a.b /= b});
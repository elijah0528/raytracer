use crate::vec3::Vec3;
use crate::interval::{Interval};
use crate::constants::{linear_to_gamma};
use std::fmt;
use std::ops::{Deref, DerefMut, Div, Mul, Add, Sub};


// Creating a new type wrapper
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color(Vec3);

impl Deref for Color {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// DerefMut is of type Deref so the output vector is inherited
impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Self::Output {
        Color::new(self[0] + other[0], self[1] + other[1], self[2] + other[2])
    }
}

impl Add<Vec3> for Color {
    type Output = Color;

    fn add(self, other: Vec3) -> Self::Output {
        Color::new(self[0] + other[0], self[1] + other[1], self[2] + other[2])
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Self::Output {
        Color::new(self[0] - other[0], self[1] - other[1], self[2] - other[2])
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, t: f32) -> Self::Output {
        Color::new(self[0] / t, self[1] / t, self[2] / t)
    }

}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        Color::new(self[0]* other[0], self[1] * other[1], self[2] * other[2])
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, t: f32) -> Self::Output {
        Color::new(self[0] * t, self[1] * t, self[2] * t)
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, v: Color) -> Self::Output {
        Color::new(v[0] * self, v[1] * self, v[2] * self)
    }
}


impl fmt::Display for Color {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        r = linear_to_gamma(r);
        g = linear_to_gamma(g);
        b = linear_to_gamma(b);

        let intensity: Interval = Interval::new(0.000, 0.999);
        let ir = (intensity.clamp(r) * 256.0) as i32;
        let ig = (intensity.clamp(g) * 256.0) as i32;
        let ib = (intensity.clamp(b) * 256.0) as i32;

        write!(f, "{} {} {}", ir, ig, ib)

    }
}

impl Color {
    pub fn new (r: f32, g: f32, b: f32) -> Self{
/*         assert!(r <= 1.0);
        assert!(g <= 1.0);
        assert!(b <= 1.0); */

        Color ( Vec3::new(r, g, b) )
    }

    pub fn r (&self) -> f32 {
        self[0]
    }

    pub fn g (&self) -> f32 {
        self[1]
    }

    pub fn b (&self) -> f32 {
        self[2]
    }

    pub fn length_squared (&self) -> f32 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn length (&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot (&self, other: &Color) -> f32 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn cross (&self, other: &Color) -> Color {
        Color::new(self[1] * other[2] - self[2] * other[1], 
                self[2] * other[0] - self[0] * other[2], 
                self[0] * other[1] - self[1] * other[0] 
                )
    }

    pub fn unit_vector (&self) -> Color {
        *self / self.length()
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_color_display () {
        let c = Color::new(0.5, 0.4, 0.3);
        assert_eq!(format!("{}", c), "127 102 76");
    }

    #[test]
    fn test_color_vec3_add () {
        let c = Color::new(0.5, 0.4, 0.3);
        let v = Vec3::new(0.5, 0.4, 0.3);
        let result = c + v;
        assert_eq!(result[0], 1.0);
        assert_eq!(result[1], 0.8);
        assert_eq!(result[2], 0.6);

    }
}

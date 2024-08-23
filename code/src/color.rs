use crate::vec3::Vec3;
use std::fmt;
use std::ops::{Deref, DerefMut, Div};


// Creating a new type wrapper
#[derive(Copy, Clone, Debug)]
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

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, t: f32) -> Self::Output {
        Color::new(self[0] / t, self[1] / t, self[2] / t)
    }

}

impl fmt::Display for Color {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        let ir = (r * 255.999) as i32;
        let ig = (g * 255.999) as i32;
        let ib = (b * 255.999) as i32;

        write!(f, "{} {} {}", ir, ig, ib)

    }
}

impl Color {
    pub fn new (r: f32, g: f32, b: f32) -> Self{
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
}

use std::ops::{Neg, Index, IndexMut, AddAssign, MulAssign, DivAssign, Add, Sub, Mul, Div};
use std::fmt;

pub type Point3 = Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
}

impl fmt::Display for Vec3 {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1} {:.1} {:.1}", self[0] as f32, self[1] as f32, self[2] as f32)
    }
}


impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 { 
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output{
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32{
        &mut self.e[i]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        self[0] += v.e[0];
        self[1] += v.e[1];
        self[2] += v.e[2];
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self[0] *= t;
        self[1] *= t;
        self[2] *= t;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        *self *= 1.0/t;
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3 { e: [ self[0] + other[0], self[1] + other[1], self[2] + other[2] ] }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3 { e: [ self[0] - other[0], self[1] - other[1], self[2] - other[2] ] }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 { e: [ self[0]* other[0], self[1] * other[1], self[2] * other[2] ] }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Self::Output {
        Vec3 { e: [ self[0] * t, self[1] * t, self[2] * t ] }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3 { e: [ v[0] * self, v[1] * self, v[2] * self ] }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Self::Output {
        Vec3 { e: [ self[0] * 1.0/t, self[1] * 1.0/t, self[2] * 1.0/t ] }
    }
}



impl Vec3 {
    fn new (x: f32, y: f32, z: f32) -> Self{
        Vec3 { e: [x, y, z] }
    }

    fn x (&self) -> f32 {
        self[0]
    }

    fn y (&self) -> f32 {
        self[1]
    }

    fn z (&self) -> f32 {
        self[2]
    }

    fn length_squared (&self) -> f32 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    fn length (&self) -> f32 {
        self.length_squared().sqrt()
    }

    fn dot (&self, other: &Vec3) -> f32 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    fn cross (&self, other: &Vec3) -> Vec3 {
        Vec3 { e: [ self[1] * other[2] - self[2] * other[1], 
                self[2] * other[0] - self[0] * other[2], 
                self[0] * other[1] - self[1] * other[0] 
                ] 
            }
    }

    fn unit_vector (&self) -> Vec3 {
        *self / self.length()
    }

}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_vec3_creation_default(){
        let v = Vec3::default();
        assert_eq!(v.e[0], 0.0);
        assert_eq!(v.e[1], 0.0);
        assert_eq!(v.e[2], 0.0);
    }

    #[test]
    fn test_vec3_creation(){
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.e[0], 1.0);
        assert_eq!(v.e[1], 2.0);
        assert_eq!(v.e[2], 3.0);
    }

    #[test]
    fn test_vec3_print(){
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(format!("{}", v), "1.0 2.0 3.0");
    }

}
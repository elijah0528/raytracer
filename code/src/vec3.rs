use std::ops::{Neg, Index, IndexMut, AddAssign, MulAssign, DivAssign, Add, Sub, Mul, Div};
use std::fmt;

pub type Point3 = Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
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
    pub fn new (x: f32, y: f32, z: f32) -> Self{
        Vec3 { e: [x, y, z] }
    }

    pub fn x (&self) -> f32 {
        self[0]
    }

    pub fn y (&self) -> f32 {
        self[1]
    }

    pub fn z (&self) -> f32 {
        self[2]
    }

    pub fn length_squared (&self) -> f32 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn length (&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot (&self, other: &Vec3) -> f32 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn cross (&self, other: &Vec3) -> Vec3 {
        Vec3 { e: [ self[1] * other[2] - self[2] * other[1], 
                self[2] * other[0] - self[0] * other[2], 
                self[0] * other[1] - self[1] * other[0] 
                ] 
            }
    }

    pub fn unit_vector (&self) -> Vec3 {
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

    #[test]
    fn test_vec3_neg(){
        let v = Vec3::new(1.0, 2.0, 3.0);
        let u = v.neg();
        let w = u.neg();
        assert_eq!(u.e[0], -1.0);
        assert_eq!(u.e[1], -2.0);
        assert_eq!(u.e[2], -3.0);    

        assert_eq!(w[0], 1.0);
        assert_eq!(w[1], 2.0);
        assert_eq!(w[2], 3.0);
    }

    #[test]
    fn test_vec3_add_assign(){
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        let u = Vec3::new(4.0, 5.0, 6.0);
        v += u;

        assert_eq!(v[0], 5.0);
        assert_eq!(v[1], 7.0);
        assert_eq!(v[2], 9.0);
    }

    #[test]
    fn test_vec3_mul_assign(){
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        let u = 3.0;
        v *= u;

        assert_eq!(v[0], 3.0);
        assert_eq!(v[1], 6.0);
        assert_eq!(v[2], 9.0);
    }

    #[test]
    fn test_vec3_div_assign(){
        let mut v = Vec3::new(6.0, 2.0, 8.0);
        let u = 8.0;
        v /= u;

        assert_eq!(v[0], 0.75);
        assert_eq!(v[1], 0.25);
        assert_eq!(v[2], 1.0);
    }

    #[test]
    fn test_vec3_add(){
        let v = Vec3::new(1.0, 2.0, 3.0);
        let u = Vec3::new(4.0, 5.0, 6.0);
        let w = v + u;

        assert_eq!(w[0], 5.0);
        assert_eq!(w[1], 7.0);
        assert_eq!(w[2], 9.0);
    }
    
    #[test]
    fn test_vec3_sub(){
        let v = Vec3::new(1.0, 2.0, 3.0);
        let u = Vec3::new(4.0, 5.0, 6.0);
        let w = v - u;

        assert_eq!(w[0], -3.0);
        assert_eq!(w[1], -3.0);
        assert_eq!(w[2], -3.0);
    }

    #[test]
    fn test_vec3_mul(){
        let v = Vec3::new(1.0, 2.0, 3.0);
        let u = Vec3::new(4.0, 5.0, 6.0);
        let w = u * v;

        assert_eq!(w[0], 4.0);
        assert_eq!(w[1], 10.0);
        assert_eq!(w[2], 18.0);
    }

    #[test]
    fn test_vec3_mul_float(){
        let v = Vec3::new(1.0, 2.0, 3.0);
        let t = 2.0;
        let w = v * t;

        assert_eq!(w[0], 2.0);
        assert_eq!(w[1], 4.0);
        assert_eq!(w[2], 6.0);
    }

    #[test]
    fn test_vec3_div_float(){
        let v = Vec3::new(1.0, 2.0, 3.0);
        let t = 2.0;
        let w = v / t;

        assert_eq!(w[0], 0.5);
        assert_eq!(w[1], 1.0);
        assert_eq!(w[2], 1.5);
    }

    #[test]
    fn test_vec3_length(){
        let v = Vec3::new(2.0, 2.0, 3.0);

        assert_eq!(v.length_squared(), 17.0);
        assert_eq!(v.length(), 17.0_f32.sqrt());
    }

    #[test]
    fn test_vec3_dot(){
        let v = Vec3::new(2.0, 2.0, 3.0);
        let u = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(v.dot(&u), 22.0);
    }

    #[test]
    fn test_vec3_cross(){
        let v = Vec3::new(1.0, 6.0, 0.0);
        let u = Vec3::new(-2.0, 5.0, 0.0);

        let w = v.cross(&u);

        assert_eq!(w[0], 0.0);
        assert_eq!(w[1], 0.0);
        assert_eq!(w[2], 17.0);

        // Testing ownership
        println!("{}", v);
        println!("{}", u);
        println!("{}", w);

    }

    #[test]
    fn test_vec3_unit_vector(){
        // Testing alias
        let v = Point3::new(3.0, 4.0, 10.0);

        let w = v.unit_vector();

        assert_eq!(w[0], 3.0 / (125_f32.sqrt()));
        assert_eq!(w[1], 4.0 / (125_f32.sqrt()));
        assert_eq!(w[2], 10.0 / (125_f32.sqrt()));  


    }

}
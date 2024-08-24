use crate::vec3::Vec3;
use crate::vec3::Point3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    origin: Point3, 
    direction: Vec3,
}


impl Default for Ray {
    fn default() -> Self {

        let origin = Point3::default();
        let direction = Vec3::default();

        Ray { origin, direction }
    }
}


impl Ray {
    pub fn new (origin: Point3, direction: Vec3) -> Self{
        Ray { origin, direction }
    }

    pub fn origin (&self) -> Point3 {
        return self.origin
    }

    pub fn direction (&self) -> Vec3 {
        return self.direction
    }

    pub fn at (&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_creation_default(){
        let v = Ray::default();
        assert_eq!(v.origin[0], 0.0);
        assert_eq!(v.origin[1], 0.0);
        assert_eq!(v.origin[2], 0.0);
        assert_eq!(v.direction[0], 0.0);
        assert_eq!(v.direction[1], 0.0);
        assert_eq!(v.direction[2], 0.0);
    }

    #[test]
    fn test_ray_creation(){
        
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        
        let v = Ray::new(origin, direction);


        assert_eq!(v.origin, origin);
        assert_eq!(v.direction, direction);

        assert_eq!(v.origin[0], 1.0);
        assert_eq!(v.origin[1], 2.0);
        assert_eq!(v.origin[2], 3.0);
        assert_eq!(v.direction[0], 4.0);
        assert_eq!(v.direction[1], 5.0);
        assert_eq!(v.direction[2], 6.0);
    }


    #[test]
    fn test_ray_at(){
        
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        
        let v = Ray::new(origin, direction);

        let ans = Vec3::new(9.0, 12.0, 15.0);

        assert_eq!(v.at(2.0), ans);
    }
}

use crate::vec3::Vec3;
use crate::vec3::Point3;

pub struct Ray {
    origin: Point3, 
    dir: Vec3,
}


impl Default for Ray {
    fn default() -> Self {

        let origin = Point3::default();
        let dir = Vec3::default();

        Ray { origin, dir }
    }
}


impl Ray {
    pub fn new (origin: Point3, dir: Vec3) -> Self{
        Ray { origin, dir }
    }

    pub fn origin (&self) -> Point3 {
        return self.origin
    }

    pub fn direction (&self) -> Vec3 {
        return self.dir
    }

    pub fn at (&self, t: f32) -> Point3 {
        self.origin + self.dir * t
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
        assert_eq!(v.dir[0], 0.0);
        assert_eq!(v.dir[1], 0.0);
        assert_eq!(v.dir[2], 0.0);
    }

    #[test]
    fn test_ray_creation(){
        
        let origin = Point3::new(1.0, 2.0, 3.0);
        let dir = Vec3::new(4.0, 5.0, 6.0);
        
        let v = Ray::new(origin, dir);


        assert_eq!(v.origin, origin);
        assert_eq!(v.dir, dir);

        assert_eq!(v.origin[0], 1.0);
        assert_eq!(v.origin[1], 2.0);
        assert_eq!(v.origin[2], 3.0);
        assert_eq!(v.dir[0], 4.0);
        assert_eq!(v.dir[1], 5.0);
        assert_eq!(v.dir[2], 6.0);
    }


    #[test]
    fn test_ray_at(){
        
        let origin = Point3::new(1.0, 2.0, 3.0);
        let dir = Vec3::new(4.0, 5.0, 6.0);
        
        let v = Ray::new(origin, dir);

        let ans = Vec3::new(9.0, 12.0, 15.0);

        assert_eq!(v.at(2.0), ans);


    }
}

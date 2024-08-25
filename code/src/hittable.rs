use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};

pub struct hit_record {
    p: Point3,
    normal: Vec3,
    t: f32,
}

trait Hittable {
    fn hit (&self, r: Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut hit_record) -> bool;
} 


pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new (center: Point3, radius: f32) -> Self{
        Self {
            center,
            radius: radius.abs()
        }
    }
}

impl Hittable for Sphere {
    fn hit (&self, r: Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut hit_record) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(&r.direction());
        let b = 2.0 * oc.dot(&r.direction());
        let h = r.direction().dot(&oc);

        let c = oc.dot(&oc) - self.radius * self.radius;
    
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return false;
        } 
        
        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }

        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        return true;
        
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_hittable_sphere(){
        let center = Point3::new(0.0, 0.0, -1.0);
        let radius = 0.5;
        let sphere = Sphere::new(center, radius);

        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);

        let ray_tmin = 0.0;
        let ray_tmax = 100.0;
        let mut rec = hit_record {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
        };

        let hit = sphere.hit(ray, ray_tmin, ray_tmax, &mut rec);

        assert!(hit);
        assert!((rec.t - 0.5).abs() < 1e-6);
        assert!((rec.p - Point3::new(0.0, 0.0, -0.5)).length() < 1e-6);
    }
}
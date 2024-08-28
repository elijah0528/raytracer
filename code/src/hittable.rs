use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use std::sync::Arc;

#[derive(Copy, Clone)]
pub struct Hit_record {
    p: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl Hit_record {
    fn set_face_normal (&mut self, r: &Ray, outward_normal: Vec3){
        let truth = r.direction().dot(&outward_normal);
        if truth > 0.0 {
            self.front_face = true
        } else {
            self.front_face = false
        }
        if self.front_face == false {
            self.normal = -outward_normal;
        } else {
            self.normal = outward_normal;
        }
    }
}

impl Default for Hit_record {
    fn default () -> Self {
        Hit_record { p: Point3::default(),
                    normal: Vec3::default(),
                    t: 0.0,
                    front_face: false,
        }
    }
}
trait Hittable {
    fn hit (&self, r: Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut Hit_record) -> bool;
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
    fn hit (&self, r: Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut Hit_record) -> bool {
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
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&r, outward_normal);

        return true;
    }
}

pub struct Hittable_list {
    objects: Vec<Arc<dyn Hittable>>,
    
}

impl Hittable_list {
    pub fn new () -> Self {
        Hittable_list {objects: vec!()}
    }

    pub fn objects(&self) -> &Vec<Arc<dyn Hittable>> {
        &self.objects
    } 

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for Hittable_list {
    fn hit (&self, r: Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut Hit_record) -> bool {
        let mut temp_rec: Hit_record = Hit_record::default();
        let mut hit_anything: bool = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
           if object.hit(r, ray_tmin, ray_tmax, rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
           } 
        }

        hit_anything
   
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
        let mut rec = Hit_record {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        };

        let hit = sphere.hit(ray, ray_tmin, ray_tmax, &mut rec);

        assert!(hit);
        assert!((rec.t - 0.5).abs() < 1e-6);
        assert!((rec.p - Point3::new(0.0, 0.0, -0.5)).length() < 1e-6);
    }
}
use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use std::sync::Arc;
use crate::constants::{INFINITY, PI};
use crate::interval::{Interval};


// A hit record is a point on an object that is hit. The normal vector is captured, distance from the camera and whether or not it hit the front face
#[derive(Default, Copy, Clone)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn p(&self) -> Point3 {
        self.p
    }

    pub fn normal(&self)-> Vec3 {
        self.normal
    }

    pub fn t(&self)-> f32 {
        self.t
    }

    pub fn front_face(&self)-> bool {
        self.front_face
    }

    pub fn set_face_normal (&mut self, r: &Ray, outward_normal: Vec3){
        let truth = r.direction().dot(&outward_normal);

        self.front_face = truth < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

// Hittable is a trait implemented for all objects that can be hit by a ray
pub trait Hittable {
    fn hit (&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> Option<HitRecord>;
} 

// Sphere is an example of a hittable object
pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new (center: Point3, radius: f32) -> Self{
        Self {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit (&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let b = 2.0 * oc.dot(&r.direction());
        let h = r.direction().dot(&oc);

        let c = oc.length_squared() - self.radius * self.radius;
    
        let discriminant = h * h - a * c;


        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-h - root) / a;
            if ray_t.surrounds(temp) {
                let p = r.at(temp);
                let normal = (p - self.center) / self.radius;
                let front_face = r.direction().dot(&normal) < 0.0;
                

                let mut hit_record = HitRecord{
                    p,
                    normal: normal,
                    t: temp,
                    front_face,
                };
                hit_record.set_face_normal(&r, normal);
                *rec = hit_record.clone();
                return Some(hit_record);
            }
        }
        None
    }
}

// A hittable list is a list of objects which implement the hittable trait
#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new () -> Self {
        HittableList {objects: vec!()}
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

impl Hittable for HittableList {
    fn hit (&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> Option<HitRecord> {
        let mut temp_rec: HitRecord = HitRecord::default();
        let mut hit_anything: bool = false;
        let mut closest_so_far = ray_t.max();

        for object in &self.objects {
            let interval: Interval = Interval::new(ray_t.min(), closest_so_far);
            if let Some(hit) = object.hit(r, interval, &mut temp_rec) { 
                hit_anything = true;
                closest_so_far = hit.t();
                
                *rec = temp_rec.clone();
            } 
        }
        
        if hit_anything {
            Some(*rec) 
        } else {
            None
        }
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

        let ray_t: Interval = Interval::new(0.0, INFINITY);
        let mut rec = HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        };

        let hit = sphere.hit(ray, ray_t, &mut rec);

        assert!(hit.is_some());
        let hit_record = hit.unwrap();
        assert!((hit_record.t - 0.5).abs() < 1e-6);
        assert!((hit_record.p - Point3::new(0.0, 0.0, -0.5)).length() < 1e-6);
    }
}
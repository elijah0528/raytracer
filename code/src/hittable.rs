use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use crate::material::Material;
use std::sync::Arc;
use crate::interval::Interval;

/// A hit record stores information about a ray-object intersection
#[derive(Clone)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
    pub material: Option<Arc<dyn Material>>,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
            material: None,
        }
    }
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f32, front_face: bool, material: Option<Arc<dyn Material>>) -> Self {
        HitRecord { p, normal, t, front_face, material }
    }

    pub fn p(&self) -> Point3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

/// Hittable trait for objects that can be hit by a ray
pub trait Hittable: Send + Sync {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> Option<HitRecord>;
}

/// Sphere primitive
pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Arc<dyn Material>) -> Self {
        Self { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let h = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            
            // Try the closer root first
            let temp = (-h - root) / a;
            if ray_t.surrounds(temp) {
                let p = r.at(temp);
                let outward_normal = (p - self.center) / self.radius;
                
                let mut hit_record = HitRecord {
                    p,
                    normal: outward_normal,
                    t: temp,
                    front_face: false,
                    material: Some(Arc::clone(&self.material)),
                };
                hit_record.set_face_normal(&r, outward_normal);
                *rec = hit_record.clone();
                return Some(hit_record);
            }
            
            // Try the farther root
            let temp = (-h + root) / a;
            if ray_t.surrounds(temp) {
                let p = r.at(temp);
                let outward_normal = (p - self.center) / self.radius;
                
                let mut hit_record = HitRecord {
                    p,
                    normal: outward_normal,
                    t: temp,
                    front_face: false,
                    material: Some(Arc::clone(&self.material)),
                };
                hit_record.set_face_normal(&r, outward_normal);
                *rec = hit_record.clone();
                return Some(hit_record);
            }
        }
        None
    }
}

/// A list of hittable objects
#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> Option<HitRecord> {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max();

        for object in &self.objects {
            let interval = Interval::new(ray_t.min(), closest_so_far);
            if let Some(hit) = object.hit(r, interval, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = hit.t();
                *rec = temp_rec.clone();
            }
        }

        if hit_anything {
            Some(rec.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::INFINITY;
    use crate::material::Lambertian;
    use crate::color::Color;

    #[test]
    fn test_hittable_sphere() {
        let center = Point3::new(0.0, 0.0, -1.0);
        let radius = 0.5;
        let material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let sphere = Sphere::new(center, radius, material);

        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);

        let ray_t = Interval::new(0.0, INFINITY);
        let mut rec = HitRecord::default();

        let hit = sphere.hit(ray, ray_t, &mut rec);

        assert!(hit.is_some());
        let hit_record = hit.unwrap();
        assert!(hit_record.front_face());
        assert!((hit_record.t - 0.5).abs() < 1e-6);
        assert!((hit_record.p - Point3::new(0.0, 0.0, -0.5)).length() < 1e-6);
        assert!(hit_record.material.is_some());
    }
}

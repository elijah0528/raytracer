use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use crate::material::Material;
use crate::aabb::AABB;
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

    pub fn set_all(&mut self, p: Point3, normal: Vec3, t: f32, material: Arc<dyn Material>) {
        self.p = p;
        self.normal = normal;
        self.t = t;
        self.material = Some(material);
    }
}

/// Hittable trait for objects that can be hit by a ray
pub trait Hittable: Send + Sync {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB> { None }
}

/// A list of hittable objects
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl Default for HittableList {
    fn default() -> Self {
        HittableList {
            objects: vec![],
            bbox: AABB::default(),
        }
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        if let Some(obj_box) = object.bounding_box() {
            self.bbox = AABB::surrounding_box(&self.bbox, &obj_box);
        }
        self.objects.push(object)
    }

    pub fn objects(&self) -> &Vec<Arc<dyn Hittable>> {
        &self.objects
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

    fn bounding_box(&self) -> Option<AABB> {
        if self.objects.is_empty() {
            None
        } else {
            Some(self.bbox)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_record_default() {
        let rec = HitRecord::default();
        assert_eq!(rec.t(), 0.0);
        assert!(!rec.front_face());
        assert!(rec.material.is_none());
    }
}

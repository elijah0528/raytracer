use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use crate::material::Material;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::aabb::AABB;
use std::sync::Arc;

/// Infinite plane primitive defined by a point and normal
pub struct Plane {
    point: Point3,
    normal: Vec3,
    material: Arc<dyn Material>,
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3, material: Arc<dyn Material>) -> Self {
        Self {
            point,
            normal: normal.unit_vector(),
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> Option<HitRecord> {
        let denom = self.normal.dot(&r.direction());

        // Check if ray is parallel to plane
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.point - r.origin()).dot(&self.normal) / denom;

        if !ray_t.surrounds(t) {
            return None;
        }

        let p = r.at(t);

        let mut hit_record = HitRecord::default();
        hit_record.set_all(p, self.normal, t, Arc::clone(&self.material));
        hit_record.set_face_normal(&r, self.normal);
        *rec = hit_record.clone();
        Some(hit_record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::INFINITY;
    use crate::material::Lambertian;
    use crate::color::Color;

    #[test]
    fn test_plane_hit() {
        let point = Point3::new(0.0, 0.0, 0.0);
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let plane = Plane::new(point, normal, material);

        let origin = Point3::new(0.0, 1.0, 0.0);
        let direction = Vec3::new(0.0, -1.0, 0.0);
        let ray = Ray::new(origin, direction);

        let ray_t = Interval::new(0.001, INFINITY);
        let mut rec = HitRecord::default();

        let hit = plane.hit(ray, ray_t, &mut rec);

        assert!(hit.is_some());
        let hit_record = hit.unwrap();
        assert!((hit_record.t() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_plane_parallel_ray() {
        let point = Point3::new(0.0, 0.0, 0.0);
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let plane = Plane::new(point, normal, material);

        let origin = Point3::new(0.0, 1.0, 0.0);
        let direction = Vec3::new(1.0, 0.0, 0.0); // Parallel to plane
        let ray = Ray::new(origin, direction);

        let ray_t = Interval::new(0.001, INFINITY);
        let mut rec = HitRecord::default();

        let hit = plane.hit(ray, ray_t, &mut rec);
        assert!(hit.is_none());
    }
}

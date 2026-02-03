use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use crate::material::Material;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use std::sync::Arc;

/// Triangle primitive using Möller-Trumbore intersection algorithm
pub struct Triangle {
    v0: Point3,
    v1: Point3,
    v2: Point3,
    normal: Vec3,
    material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, material: Arc<dyn Material>) -> Self {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = edge1.cross(&edge2).unit_vector();

        Self { v0, v1, v2, normal, material }
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> Option<HitRecord> {
        const EPSILON: f32 = 1e-8;

        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;

        let h = r.direction().cross(&edge2);
        let a = edge1.dot(&h);

        // Ray is parallel to triangle
        if a.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = r.origin() - self.v0;
        let u = f * s.dot(&h);

        // Check barycentric coordinate u
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * r.direction().dot(&q);

        // Check barycentric coordinate v
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);

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
    fn test_triangle_hit() {
        let v0 = Point3::new(-1.0, 0.0, 0.0);
        let v1 = Point3::new(1.0, 0.0, 0.0);
        let v2 = Point3::new(0.0, 1.0, 0.0);
        let material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let triangle = Triangle::new(v0, v1, v2, material);

        let origin = Point3::new(0.0, 0.3, 1.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);

        let ray_t = Interval::new(0.001, INFINITY);
        let mut rec = HitRecord::default();

        let hit = triangle.hit(ray, ray_t, &mut rec);

        assert!(hit.is_some());
        let hit_record = hit.unwrap();
        assert!((hit_record.t() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_triangle_miss() {
        let v0 = Point3::new(-1.0, 0.0, 0.0);
        let v1 = Point3::new(1.0, 0.0, 0.0);
        let v2 = Point3::new(0.0, 1.0, 0.0);
        let material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let triangle = Triangle::new(v0, v1, v2, material);

        let origin = Point3::new(5.0, 5.0, 1.0); // Outside triangle
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);

        let ray_t = Interval::new(0.001, INFINITY);
        let mut rec = HitRecord::default();

        let hit = triangle.hit(ray, ray_t, &mut rec);
        assert!(hit.is_none());
    }
}

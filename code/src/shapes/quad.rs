use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use crate::material::Material;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::aabb::AABB;
use std::sync::Arc;

/// Quad (parallelogram) primitive defined by a corner and two edge vectors
pub struct Quad {
    q: Point3,      // Corner point
    u: Vec3,        // First edge vector
    v: Vec3,        // Second edge vector
    normal: Vec3,   // Unit normal
    d: f32,         // Plane constant
    w: Vec3,        // For barycentric coordinates
    material: Arc<dyn Material>,
    bbox: AABB,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let n = u.cross(&v);
        let normal = n.unit_vector();
        let d = normal.dot(&q);
        let w = n / n.dot(&n);

        // Compute bounding box from the four corners
        let bbox_diagonal1 = AABB::from_points(q, q + u + v);
        let bbox_diagonal2 = AABB::from_points(q + u, q + v);
        let bbox = AABB::surrounding_box(&bbox_diagonal1, &bbox_diagonal2).pad();

        Self { q, u, v, normal, d, w, material, bbox }
    }

    /// Check if hit point is inside the quad using barycentric coordinates
    fn is_interior(a: f32, b: f32) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        unit_interval.contains(a) && unit_interval.contains(b)
    }
}

impl Hittable for Quad {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> Option<HitRecord> {
        let denom = self.normal.dot(&r.direction());

        // Check if ray is parallel to plane
        if denom.abs() < 1e-8 {
            return None;
        }

        // Calculate t for ray-plane intersection
        let t = (self.d - self.normal.dot(&r.origin())) / denom;
        if !ray_t.surrounds(t) {
            return None;
        }

        // Check if hit point is inside the quad
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        if !Self::is_interior(alpha, beta) {
            return None;
        }

        let mut hit_record = HitRecord::default();
        hit_record.set_all(intersection, self.normal, t, Arc::clone(&self.material));
        hit_record.set_face_normal(&r, self.normal);
        *rec = hit_record.clone();
        Some(hit_record)
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bbox)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::INFINITY;
    use crate::material::Lambertian;
    use crate::color::Color;

    #[test]
    fn test_quad_hit_center() {
        let q = Point3::new(-1.0, -1.0, 0.0);
        let u = Vec3::new(2.0, 0.0, 0.0);
        let v = Vec3::new(0.0, 2.0, 0.0);
        let material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let quad = Quad::new(q, u, v, material);

        let origin = Point3::new(0.0, 0.0, 1.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);

        let ray_t = Interval::new(0.001, INFINITY);
        let mut rec = HitRecord::default();

        let hit = quad.hit(ray, ray_t, &mut rec);

        assert!(hit.is_some());
        let hit_record = hit.unwrap();
        assert!((hit_record.t() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_quad_miss_outside() {
        let q = Point3::new(-1.0, -1.0, 0.0);
        let u = Vec3::new(2.0, 0.0, 0.0);
        let v = Vec3::new(0.0, 2.0, 0.0);
        let material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let quad = Quad::new(q, u, v, material);

        let origin = Point3::new(5.0, 5.0, 1.0); // Outside the quad
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);

        let ray_t = Interval::new(0.001, INFINITY);
        let mut rec = HitRecord::default();

        let hit = quad.hit(ray, ray_t, &mut rec);
        assert!(hit.is_none());
    }
}

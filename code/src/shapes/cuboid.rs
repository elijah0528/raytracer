use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use crate::material::Material;
use crate::hittable::{Hittable, HitRecord, HittableList};
use crate::interval::Interval;
use crate::shapes::Quad;
use std::sync::Arc;

/// Axis-aligned box (cuboid) made of 6 quads
pub struct Cuboid {
    sides: HittableList,
}

impl Cuboid {
    /// Create a box with opposite corners at p0 and p1
    pub fn new(p0: Point3, p1: Point3, material: Arc<dyn Material>) -> Self {
        let min = Point3::new(
            p0.x().min(p1.x()),
            p0.y().min(p1.y()),
            p0.z().min(p1.z()),
        );
        let max = Point3::new(
            p0.x().max(p1.x()),
            p0.y().max(p1.y()),
            p0.z().max(p1.z()),
        );

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        let mut sides = HittableList::new();

        // Front face (z = max)
        sides.add(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), max.z()),
            dx,
            dy,
            Arc::clone(&material),
        )));

        // Back face (z = min)
        sides.add(Arc::new(Quad::new(
            Point3::new(max.x(), min.y(), min.z()),
            -dx,
            dy,
            Arc::clone(&material),
        )));

        // Left face (x = min)
        sides.add(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dz,
            dy,
            Arc::clone(&material),
        )));

        // Right face (x = max)
        sides.add(Arc::new(Quad::new(
            Point3::new(max.x(), min.y(), max.z()),
            -dz,
            dy,
            Arc::clone(&material),
        )));

        // Top face (y = max)
        sides.add(Arc::new(Quad::new(
            Point3::new(min.x(), max.y(), max.z()),
            dx,
            -dz,
            Arc::clone(&material),
        )));

        // Bottom face (y = min)
        sides.add(Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dx,
            dz,
            material,
        )));

        Self { sides }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> Option<HitRecord> {
        self.sides.hit(r, ray_t, rec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::INFINITY;
    use crate::material::Lambertian;
    use crate::color::Color;

    #[test]
    fn test_cuboid_hit() {
        let p0 = Point3::new(-1.0, -1.0, -1.0);
        let p1 = Point3::new(1.0, 1.0, 1.0);
        let material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let cuboid = Cuboid::new(p0, p1, material);

        let origin = Point3::new(0.0, 0.0, 5.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);

        let ray_t = Interval::new(0.001, INFINITY);
        let mut rec = HitRecord::default();

        let hit = cuboid.hit(ray, ray_t, &mut rec);

        assert!(hit.is_some());
        let hit_record = hit.unwrap();
        assert!((hit_record.t() - 4.0).abs() < 1e-6);
    }
}

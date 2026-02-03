use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use crate::material::Material;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use std::sync::Arc;

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

                let mut hit_record = HitRecord::default();
                hit_record.set_all(p, outward_normal, temp, Arc::clone(&self.material));
                hit_record.set_face_normal(&r, outward_normal);
                *rec = hit_record.clone();
                return Some(hit_record);
            }

            // Try the farther root
            let temp = (-h + root) / a;
            if ray_t.surrounds(temp) {
                let p = r.at(temp);
                let outward_normal = (p - self.center) / self.radius;

                let mut hit_record = HitRecord::default();
                hit_record.set_all(p, outward_normal, temp, Arc::clone(&self.material));
                hit_record.set_face_normal(&r, outward_normal);
                *rec = hit_record.clone();
                return Some(hit_record);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::INFINITY;
    use crate::material::Lambertian;
    use crate::color::Color;

    #[test]
    fn test_sphere_hit() {
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
        assert!((hit_record.t() - 0.5).abs() < 1e-6);
    }
}

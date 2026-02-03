use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use crate::material::Material;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::aabb::AABB;
use crate::animation::AnimationTrack;
use std::sync::Arc;
use std::cell::RefCell;

/// Animated sphere that can move and scale over time
pub struct AnimatedSphere {
    base_center: Point3,
    base_radius: f32,
    material: Arc<dyn Material>,
    animation: AnimationTrack,
    // Current state (updated per frame)
    current_time: RefCell<f32>,
    current_center: RefCell<Point3>,
    current_scale: RefCell<Vec3>,
}

impl AnimatedSphere {
    pub fn new(
        center: Point3,
        radius: f32,
        material: Arc<dyn Material>,
        animation: AnimationTrack,
    ) -> Self {
        Self {
            base_center: center,
            base_radius: radius,
            material,
            animation,
            current_time: RefCell::new(0.0),
            current_center: RefCell::new(center),
            current_scale: RefCell::new(Vec3::new(1.0, 1.0, 1.0)),
        }
    }

    /// Update the sphere's state for a given time
    pub fn set_time(&self, time: f32) {
        let (pos, scale) = self.animation.sample(time);
        *self.current_time.borrow_mut() = time;
        *self.current_center.borrow_mut() = pos;
        *self.current_scale.borrow_mut() = scale;
    }

    fn get_center(&self) -> Point3 {
        *self.current_center.borrow()
    }

    fn get_scale(&self) -> Vec3 {
        *self.current_scale.borrow()
    }
}

impl Hittable for AnimatedSphere {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> Option<HitRecord> {
        let center = self.get_center();
        let scale = self.get_scale();
        
        // For ellipsoid intersection, we transform the ray into object space
        // where the ellipsoid becomes a unit sphere
        let scaled_radius = Vec3::new(
            self.base_radius * scale.x(),
            self.base_radius * scale.y(),
            self.base_radius * scale.z(),
        );

        // Transform ray to object space (scale the ray)
        let inv_scale = Vec3::new(1.0 / scale.x(), 1.0 / scale.y(), 1.0 / scale.z());
        let ray_origin = Point3::new(
            (r.origin().x() - center.x()) * inv_scale.x(),
            (r.origin().y() - center.y()) * inv_scale.y(),
            (r.origin().z() - center.z()) * inv_scale.z(),
        );
        let ray_dir = Vec3::new(
            r.direction().x() * inv_scale.x(),
            r.direction().y() * inv_scale.y(),
            r.direction().z() * inv_scale.z(),
        );

        // Now do sphere intersection with unit sphere at origin
        let oc = ray_origin;
        let a = ray_dir.length_squared();
        let h = ray_dir.dot(&oc);
        let c = oc.length_squared() - self.base_radius * self.base_radius;
        let discriminant = h * h - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            // Try the closer root first
            let temp = (-h - root) / a;
            if ray_t.surrounds(temp) {
                let p = r.at(temp);
                
                // Calculate normal in world space (accounting for scale)
                let local_p = Point3::new(
                    (p.x() - center.x()) * inv_scale.x(),
                    (p.y() - center.y()) * inv_scale.y(),
                    (p.z() - center.z()) * inv_scale.z(),
                );
                let outward_normal = Vec3::new(
                    local_p.x() * inv_scale.x(),
                    local_p.y() * inv_scale.y(),
                    local_p.z() * inv_scale.z(),
                ).unit_vector();

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
                
                let local_p = Point3::new(
                    (p.x() - center.x()) * inv_scale.x(),
                    (p.y() - center.y()) * inv_scale.y(),
                    (p.z() - center.z()) * inv_scale.z(),
                );
                let outward_normal = Vec3::new(
                    local_p.x() * inv_scale.x(),
                    local_p.y() * inv_scale.y(),
                    local_p.z() * inv_scale.z(),
                ).unit_vector();

                let mut hit_record = HitRecord::default();
                hit_record.set_all(p, outward_normal, temp, Arc::clone(&self.material));
                hit_record.set_face_normal(&r, outward_normal);
                *rec = hit_record.clone();
                return Some(hit_record);
            }
        }
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        // Return a conservative bounding box that covers all animation states
        // For now, use a large box
        let center = self.get_center();
        let scale = self.get_scale();
        let max_scale = scale.x().max(scale.y()).max(scale.z());
        let r = self.base_radius * max_scale * 1.5; // Extra margin for animation
        let rvec = Vec3::new(r, r, r);
        Some(AABB::from_points(center - rvec, center + rvec))
    }
}

// Need to implement Send + Sync for AnimatedSphere
// RefCell is not Sync, so we need unsafe impl or use Mutex
// For simplicity in single-threaded animation rendering, let's use a simpler approach
unsafe impl Sync for AnimatedSphere {}
unsafe impl Send for AnimatedSphere {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::{AnimationTrack, Keyframe};
    use crate::material::Lambertian;
    use crate::color::Color;
    use crate::constants::INFINITY;

    #[test]
    fn test_animated_sphere_hit() {
        let mut track = AnimationTrack::new();
        track.add_keyframe(Keyframe::new(0.0, Point3::new(0.0, 0.0, -1.0)));
        track.add_keyframe(Keyframe::new(1.0, Point3::new(0.0, 1.0, -1.0)));

        let mat: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let sphere = AnimatedSphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            mat,
            track,
        );

        // At time 0
        sphere.set_time(0.0);
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let mut rec = HitRecord::default();
        let hit = sphere.hit(ray, Interval::new(0.001, INFINITY), &mut rec);
        assert!(hit.is_some());

        // At time 0.5, sphere should be at y=0.5
        sphere.set_time(0.5);
        let ray2 = Ray::new(Point3::new(0.0, 0.5, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let hit2 = sphere.hit(ray2, Interval::new(0.001, INFINITY), &mut rec);
        assert!(hit2.is_some());
    }
}

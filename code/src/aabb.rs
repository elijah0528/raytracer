use crate::vec3::Point3;
use crate::ray::Ray;
use crate::interval::Interval;

/// Axis-Aligned Bounding Box
#[derive(Clone, Copy, Default)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        AABB { x, y, z }
    }

    /// Create AABB from two corner points
    pub fn from_points(a: Point3, b: Point3) -> Self {
        AABB {
            x: if a.x() <= b.x() { Interval::new(a.x(), b.x()) } else { Interval::new(b.x(), a.x()) },
            y: if a.y() <= b.y() { Interval::new(a.y(), b.y()) } else { Interval::new(b.y(), a.y()) },
            z: if a.z() <= b.z() { Interval::new(a.z(), b.z()) } else { Interval::new(b.z(), a.z()) },
        }
    }

    /// Create AABB that encloses two other AABBs
    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> Self {
        AABB {
            x: Interval::surrounding(&box0.x, &box1.x),
            y: Interval::surrounding(&box0.y, &box1.y),
            z: Interval::surrounding(&box0.z, &box1.z),
        }
    }

    /// Get the interval for a specific axis (0=x, 1=y, 2=z)
    pub fn axis_interval(&self, axis: usize) -> &Interval {
        match axis {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }

    /// Test if ray hits the bounding box
    pub fn hit(&self, r: &Ray, ray_t: Interval) -> bool {
        let mut t_min = ray_t.min();
        let mut t_max = ray_t.max();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / r.direction()[axis];

            let t0 = (ax.min() - r.origin()[axis]) * adinv;
            let t1 = (ax.max() - r.origin()[axis]) * adinv;

            let (t0, t1) = if t0 < t1 { (t0, t1) } else { (t1, t0) };

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    /// Pad the AABB to avoid zero-thickness slabs
    pub fn pad(&self) -> AABB {
        let delta = 0.0001;
        let new_x = if self.x.size() >= delta { self.x } else { self.x.expand(delta) };
        let new_y = if self.y.size() >= delta { self.y } else { self.y.expand(delta) };
        let new_z = if self.z.size() >= delta { self.z } else { self.z.expand(delta) };
        AABB::new(new_x, new_y, new_z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Vec3;

    #[test]
    fn test_aabb_from_points() {
        let a = Point3::new(0.0, 0.0, 0.0);
        let b = Point3::new(1.0, 2.0, 3.0);
        let aabb = AABB::from_points(a, b);

        assert_eq!(aabb.x.min(), 0.0);
        assert_eq!(aabb.x.max(), 1.0);
        assert_eq!(aabb.y.min(), 0.0);
        assert_eq!(aabb.y.max(), 2.0);
    }

    #[test]
    fn test_aabb_hit() {
        let aabb = AABB::from_points(
            Point3::new(-1.0, -1.0, -1.0),
            Point3::new(1.0, 1.0, 1.0),
        );

        // Ray that hits
        let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(aabb.hit(&ray, Interval::new(0.0, f32::INFINITY)));

        // Ray that misses
        let ray_miss = Ray::new(Point3::new(5.0, 5.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(!aabb.hit(&ray_miss, Interval::new(0.0, f32::INFINITY)));
    }

    #[test]
    fn test_surrounding_box() {
        let box0 = AABB::from_points(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0));
        let box1 = AABB::from_points(Point3::new(2.0, 2.0, 2.0), Point3::new(3.0, 3.0, 3.0));
        let combined = AABB::surrounding_box(&box0, &box1);

        assert_eq!(combined.x.min(), 0.0);
        assert_eq!(combined.x.max(), 3.0);
    }
}

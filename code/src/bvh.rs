use crate::aabb::AABB;
use crate::hittable::{Hittable, HitRecord, HittableList};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::constants::random_generator_range;
use std::sync::Arc;
use std::cmp::Ordering;

/// Bounding Volume Hierarchy node for acceleration
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    /// Build BVH from a list of objects
    pub fn new(list: &HittableList) -> Self {
        let objects: Vec<Arc<dyn Hittable>> = list.objects().clone();
        Self::build(objects)
    }

    /// Build BVH from a vector of objects
    fn build(mut objects: Vec<Arc<dyn Hittable>>) -> Self {
        let axis = random_generator_range(0.0, 3.0) as usize;

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = match objects.len() {
            1 => {
                let obj = objects.pop().unwrap();
                (Arc::clone(&obj), obj)
            }
            2 => {
                let b = objects.pop().unwrap();
                let a = objects.pop().unwrap();
                if Self::box_compare(&a, &b, axis) == Ordering::Less {
                    (a, b)
                } else {
                    (b, a)
                }
            }
            _ => {
                objects.sort_by(|a, b| Self::box_compare(a, b, axis));
                let mid = objects.len() / 2;
                let right_objects: Vec<Arc<dyn Hittable>> = objects.drain(mid..).collect();
                let left_objects = objects;

                (
                    Arc::new(Self::build(left_objects)) as Arc<dyn Hittable>,
                    Arc::new(Self::build(right_objects)) as Arc<dyn Hittable>,
                )
            }
        };

        let box_left = left.bounding_box();
        let box_right = right.bounding_box();

        let bbox = match (box_left, box_right) {
            (Some(bl), Some(br)) => AABB::surrounding_box(&bl, &br),
            (Some(bl), None) => bl,
            (None, Some(br)) => br,
            (None, None) => AABB::default(),
        };

        BvhNode { left, right, bbox }
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
        let box_a = a.bounding_box();
        let box_b = b.bounding_box();

        match (box_a, box_b) {
            (Some(ba), Some(bb)) => {
                let av = ba.axis_interval(axis).min();
                let bv = bb.axis_interval(axis).min();
                av.partial_cmp(&bv).unwrap_or(Ordering::Equal)
            }
            _ => Ordering::Equal,
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> Option<HitRecord> {
        if !self.bbox.hit(&r, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let left_t = hit_left.as_ref().map(|h| h.t()).unwrap_or(ray_t.max());

        let hit_right = self.right.hit(r, Interval::new(ray_t.min(), left_t), rec);

        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bbox)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::{Vec3, Point3};
    use crate::shapes::Sphere;
    use crate::material::{Material, Lambertian};
    use crate::color::Color;
    use crate::constants::INFINITY;

    #[test]
    fn test_bvh_construction() {
        let mat: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        
        let mut list = HittableList::new();
        list.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Arc::clone(&mat))));
        list.add(Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Arc::clone(&mat))));
        list.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat)));

        let bvh = BvhNode::new(&list);
        assert!(bvh.bounding_box().is_some());
    }

    #[test]
    fn test_bvh_hit() {
        let mat: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        
        let mut list = HittableList::new();
        list.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -2.0), 0.5, mat)));

        let bvh = BvhNode::new(&list);

        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let mut rec = HitRecord::default();
        let hit = bvh.hit(ray, Interval::new(0.001, INFINITY), &mut rec);

        assert!(hit.is_some());
    }
}

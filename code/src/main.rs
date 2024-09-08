mod vec3;
mod color;
mod ray;
mod camera;
mod hittable;
mod constants;
mod interval;
// mod hittable;

use vec3::{Vec3, Point3};
use color::Color;
use ray::Ray;
use camera::Camera;
use hittable::{HitRecord, HittableList, Hittable, Sphere};
use constants::{INFINITY, PI};
use interval::{Interval};
// use hittable::{Sphere, HitRecord};

use std::sync::{Arc};

// let unit_direction = r.direction().unit_vector();
// let a = 0.5 * (unit_direction.y() + 1.0);
// (1.0-a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)


fn main() {

    let mut debug_count = 0;
    
    let aspect_ratio: f32 = 16.0/9.0;
    let image_width: i32 = 400;

    
    let mut image_height: i32 = ((image_width as f32) / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1
    }

    let mut world: HittableList = HittableList::default();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    
    
    let mut cam: Camera = Camera::new(400);
    cam.render(&mut world);

    // println!("{}", debug_count);

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_color() {
        let p = Point3::new(0.0, 0.0, 0.0);
        let q = Point3::new(1.0, 0.0, 0.0);
        let r = Ray::new(p, q);
        assert_eq!(ray_color(r), Color::new(0.75, 0.85, 1.0));
    }
}
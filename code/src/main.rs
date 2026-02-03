mod vec3;
mod color;
mod ray;
mod camera;
mod hittable;
mod constants;
mod interval;
// mod hittable;

use vec3::Point3;
use camera::Camera;
use hittable::{HittableList, Sphere};
// use hittable::{Sphere, HitRecord};

use std::sync::{Arc};

// let unit_direction = r.direction().unit_vector();
// let a = 0.5 * (unit_direction.y() + 1.0);
// (1.0-a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)


fn main() {
    let image_height: i32 = 400;

    let mut world: HittableList = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    
    
    let mut cam: Camera = Camera::new(image_height);
    cam.render(&world);

}


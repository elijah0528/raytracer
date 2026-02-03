mod vec3;
mod color;
mod ray;
mod camera;
mod hittable;
mod constants;
mod interval;
mod material;
mod shapes;

use vec3::{Vec3, Point3};
use color::Color;
use camera::CameraBuilder;
use hittable::HittableList;
use material::{Lambertian, Metal, Dielectric};
use shapes::Sphere;

use std::sync::Arc;

fn main() {
    // Materials
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    // World
    let mut world = HittableList::new();

    // Ground
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    // Center sphere (diffuse blue)
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));

    // Left sphere (glass)
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));

    // Left sphere inner (hollow glass effect)
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    )));

    // Right sphere (metal)
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera with position, FOV, and depth of field
    let cam = CameraBuilder::new()
        .image_height(400)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(20.0)
        .lookfrom(Point3::new(-2.0, 2.0, 1.0))
        .lookat(Point3::new(0.0, 0.0, -1.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(10.0)
        .focus_dist(3.4)
        .build();

    cam.render(&world);
}

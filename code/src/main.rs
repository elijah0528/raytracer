mod vec3;
mod color;
mod ray;
mod camera;
mod hittable;
mod constants;
mod interval;
mod material;
mod shapes;
mod aabb;
mod bvh;

use vec3::{Vec3, Point3};
use color::Color;
use camera::CameraBuilder;
use hittable::HittableList;
use material::{Lambertian, Metal, Dielectric, Material, DiffuseLight};
use shapes::{Sphere, Quad};
use bvh::BvhNode;
use constants::random_generator;

use std::sync::Arc;
use std::env;

/// Generate the classic random spheres scene
fn random_spheres_scene() -> HittableList {
    let mut world = HittableList::new();

    // Ground
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // Random small spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_generator();
            let center = Point3::new(
                a as f32 + 0.9 * random_generator(),
                0.2,
                b as f32 + 0.9 * random_generator(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::new(
                        random_generator() * random_generator(),
                        random_generator() * random_generator(),
                        random_generator() * random_generator(),
                    );
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::new(
                        0.5 + 0.5 * random_generator(),
                        0.5 + 0.5 * random_generator(),
                        0.5 + 0.5 * random_generator(),
                    );
                    let fuzz = 0.5 * random_generator();
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // Glass
                    Arc::new(Dielectric::new(1.5))
                };

                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    // Three large showcase spheres
    let material1: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    world
}

/// Simple scene with three spheres for quick testing
fn simple_scene() -> HittableList {
    let mut world = HittableList::new();

    // Materials
    let material_ground: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let material_right: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

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

    world
}

/// Cornell Box scene with emissive light
fn cornell_box_scene() -> HittableList {
    let mut world = HittableList::new();

    // Materials
    let red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    // Walls
    // Left wall (green)
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));

    // Right wall (red)
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));

    // Light
    world.add(Arc::new(Quad::new(
        Point3::new(213.0, 554.0, 227.0),
        Vec3::new(130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 105.0),
        light,
    )));

    // Floor
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    // Ceiling
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));

    // Back wall
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // Add two spheres inside
    let metal_sphere: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 0.0));
    let glass_sphere: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    
    world.add(Arc::new(Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        metal_sphere,
    )));
    
    world.add(Arc::new(Sphere::new(
        Point3::new(370.0, 90.0, 350.0),
        90.0,
        glass_sphere,
    )));

    world
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let scene_name = args.get(1).map(|s| s.as_str()).unwrap_or("simple");
    let output_file = args.get(2).map(|s| s.as_str()).unwrap_or("output.png");

    eprintln!("Scene: {}", scene_name);
    eprintln!("Output: {}", output_file);

    match scene_name {
        "random" => {
            // Random spheres scene
            let world = random_spheres_scene();
            let bvh = BvhNode::new(&world);

            let cam = CameraBuilder::new()
                .image_height(400)
                .samples_per_pixel(100)
                .max_depth(50)
                .vfov(20.0)
                .lookfrom(Point3::new(13.0, 2.0, 3.0))
                .lookat(Point3::new(0.0, 0.0, 0.0))
                .vup(Vec3::new(0.0, 1.0, 0.0))
                .defocus_angle(0.6)
                .focus_dist(10.0)
                .build();

            cam.render_to_png(&bvh, output_file);
        }
        "cornell" => {
            // Cornell box scene
            let world = cornell_box_scene();
            let bvh = BvhNode::new(&world);

            let cam = CameraBuilder::new()
                .image_height(400)
                .samples_per_pixel(200)
                .max_depth(50)
                .vfov(40.0)
                .lookfrom(Point3::new(278.0, 278.0, -800.0))
                .lookat(Point3::new(278.0, 278.0, 0.0))
                .vup(Vec3::new(0.0, 1.0, 0.0))
                .defocus_angle(0.0)
                .focus_dist(10.0)
                .build();

            cam.render_to_png(&bvh, output_file);
        }
        _ => {
            // Simple scene (default)
            let world = simple_scene();
            let bvh = BvhNode::new(&world);

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

            cam.render_to_png(&bvh, output_file);
        }
    }
}

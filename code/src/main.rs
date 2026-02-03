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
mod animation;

use vec3::{Vec3, Point3};
use color::Color;
use camera::CameraBuilder;
use hittable::HittableList;
use material::{Lambertian, Metal, Dielectric, Material, DiffuseLight};
use shapes::{Sphere, Quad, AnimatedSphere};
use bvh::BvhNode;
use constants::random_generator;
use animation::{AnimationTrack, Keyframe, create_bounce_animation};

use std::sync::Arc;
use std::env;
use std::fs;
use std::process::Command;

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
                    let albedo = Color::new(
                        random_generator() * random_generator(),
                        random_generator() * random_generator(),
                        random_generator() * random_generator(),
                    );
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = Color::new(
                        0.5 + 0.5 * random_generator(),
                        0.5 + 0.5 * random_generator(),
                        0.5 + 0.5 * random_generator(),
                    );
                    let fuzz = 0.5 * random_generator();
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
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

    let material_ground: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let material_right: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    )));

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

    let red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(213.0, 554.0, 227.0),
        Vec3::new(130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 105.0),
        light,
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

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

/// Bouncing glass ball animation scene
fn bouncing_ball_scene() -> (HittableList, Arc<AnimatedSphere>) {
    let mut world = HittableList::new();

    // Floor
    let floor_material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.4, 0.4, 0.45)));
    world.add(Arc::new(Quad::new(
        Point3::new(-10.0, 0.0, -10.0),
        Vec3::new(20.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 20.0),
        floor_material,
    )));

    // Back wall for reflections
    let wall_material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.7, 0.7, 0.75)));
    world.add(Arc::new(Quad::new(
        Point3::new(-10.0, 0.0, 5.0),
        Vec3::new(20.0, 0.0, 0.0),
        Vec3::new(0.0, 10.0, 0.0),
        wall_material,
    )));

    // Static metal sphere for reference
    let metal_mat: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));
    world.add(Arc::new(Sphere::new(
        Point3::new(2.0, 0.5, 0.0),
        0.5,
        metal_mat,
    )));

    // Static diffuse sphere
    let diffuse_mat: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.8, 0.3, 0.3)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-2.0, 0.5, 0.0),
        0.5,
        diffuse_mat,
    )));

    // Animated glass ball
    let glass_material: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    
    // Create bounce animation
    let bounce_track = create_bounce_animation(
        Point3::new(0.0, 4.0, -2.0),  // Start position (high up)
        0.5,                            // Floor Y (ball radius)
        5,                              // Number of bounces
        3.0,                            // Total duration in seconds
        0.25,                           // Energy loss per bounce
    );

    let animated_ball = Arc::new(AnimatedSphere::new(
        Point3::new(0.0, 4.0, -2.0),
        0.5,
        glass_material,
        bounce_track,
    ));

    (world, animated_ball)
}

/// Render animation frames
fn render_animation(
    output_dir: &str,
    fps: u32,
    duration: f32,
    samples_per_pixel: i32,
) {
    // Create output directory
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let (mut world, animated_ball) = bouncing_ball_scene();
    
    // Add the animated ball to the world
    world.add(animated_ball.clone());

    // Camera setup
    let cam = CameraBuilder::new()
        .image_height(360)
        .samples_per_pixel(samples_per_pixel)
        .max_depth(50)
        .vfov(40.0)
        .lookfrom(Point3::new(0.0, 2.0, -8.0))
        .lookat(Point3::new(0.0, 1.5, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.0)
        .focus_dist(10.0)
        .build();

    let total_frames = (fps as f32 * duration) as u32;
    
    eprintln!("Rendering {} frames at {} FPS ({} seconds)...", total_frames, fps, duration);
    eprintln!("Output directory: {}", output_dir);

    for frame in 0..total_frames {
        let time = frame as f32 / fps as f32;
        
        // Update animated objects
        animated_ball.set_time(time);

        // Render frame
        eprint!("\rFrame {}/{} (t={:.2}s)... ", frame + 1, total_frames, time);
        
        let buffer = cam.render_frame(&world);
        
        let filename = format!("{}/frame_{:04}.png", output_dir, frame);
        cam.save_buffer_to_png(&buffer, &filename);
    }

    eprintln!("\nAll frames rendered!");
    eprintln!("To create video, run:");
    eprintln!("  ffmpeg -framerate {} -i {}/frame_%04d.png -c:v libx264 -pix_fmt yuv420p output.mp4", fps, output_dir);
}

/// Render animation and create video using ffmpeg
fn render_animation_to_video(
    output_file: &str,
    fps: u32,
    duration: f32,
    samples_per_pixel: i32,
) {
    let temp_dir = "/tmp/raytracer_frames";
    
    // Render frames
    render_animation(temp_dir, fps, duration, samples_per_pixel);
    
    // Use ffmpeg to create video
    eprintln!("\nCreating video with ffmpeg...");
    
    let status = Command::new("ffmpeg")
        .args(&[
            "-y",  // Overwrite output file
            "-framerate", &fps.to_string(),
            "-i", &format!("{}/frame_%04d.png", temp_dir),
            "-c:v", "libx264",
            "-pix_fmt", "yuv420p",
            "-crf", "18",  // Quality (lower = better, 18-28 is good)
            output_file,
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            eprintln!("Video saved to: {}", output_file);
            // Clean up frames
            let _ = fs::remove_dir_all(temp_dir);
        }
        Ok(_) => {
            eprintln!("ffmpeg failed. Frames are still available at: {}", temp_dir);
        }
        Err(e) => {
            eprintln!("Failed to run ffmpeg: {}. Frames are still available at: {}", e, temp_dir);
            eprintln!("You can manually run:");
            eprintln!("  ffmpeg -framerate {} -i {}/frame_%04d.png -c:v libx264 -pix_fmt yuv420p {}", fps, temp_dir, output_file);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let scene_name = args.get(1).map(|s| s.as_str()).unwrap_or("simple");
    let output_file = args.get(2).map(|s| s.as_str()).unwrap_or("output.png");

    eprintln!("Scene: {}", scene_name);
    eprintln!("Output: {}", output_file);

    match scene_name {
        "random" => {
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
        "bounce" | "animation" => {
            // Render bouncing ball animation
            let fps = 30;
            let duration = 3.0;  // 3 seconds
            let samples = 25;   // Lower samples for faster animation rendering
            
            render_animation_to_video(output_file, fps, duration, samples);
        }
        "frames" => {
            // Just render frames without creating video
            let output_dir = if output_file.ends_with(".mp4") || output_file.ends_with(".png") {
                "frames"
            } else {
                output_file
            };
            render_animation(output_dir, 30, 3.0, 25);
        }
        _ => {
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

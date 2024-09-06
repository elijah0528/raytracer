mod vec3;
mod color;
mod ray;
mod camera;
mod hittable;
mod constants;
// mod hittable;

use vec3::{Vec3, Point3};
use color::Color;
use ray::Ray;
use camera::Camera;
use hittable::{HitRecord, HittableList, Hittable, Sphere};
use constants::{INFINITY, PI};
// use hittable::{Sphere, HitRecord};

use std::sync::{Arc};



fn ray_color (r: Ray, world: &dyn Hittable) -> Color {

    let mut rec: HitRecord = HitRecord::default();
    let hit = world.hit(r, 0.0, INFINITY, &mut rec); 
    match hit {
        Some(hit_record) => {
            let n = hit_record.normal();
            // println!("{}", hit_record.normal());
            return Color::new(
                0.5 * n.x() + 0.5,
                0.5 * n.y() + 0.5,
                0.5 * n.z() + 0.5,
            );        
        }
        None => {
            let t: f32 = 0.5 * (r.direction().unit_vector().y() as f32 + 1.0);
            return Color::new(
                (1.0 - t) * 1.0 + t * 0.5,
                (1.0 - t) * 1.0 + t * 0.7,
                (1.0 - t) * 1.0 + t * 1.0,
            );
        }
    }


    // Background color if no hit
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(
        (1.0 - t) * 1.0 + t * 0.5,
        (1.0 - t) * 1.0 + t * 0.7,
        (1.0 - t) * 1.0 + t * 1.0,
    )
}


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

    
    

    // Camera
    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * (image_width as f32) / (image_height as f32);
    
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // camera_center:Point3, focal_length: f32, viewport_height: f32, viewport_width: f32
    let camera = Camera::new(Point3::default(), 1.0, viewport_height, viewport_width);

    let pixel00_loc = camera.viewport_upper_left() + 0.5 * (pixel_delta_u + pixel_delta_v);


    println!("P3\n{} {} \n255\n", image_width, image_height);
    
    for j in 0..image_height {
        for i in 0..image_width{
            let pixel_center = pixel00_loc + (pixel_delta_u * (i as f32)) + (pixel_delta_v * (j as f32));
            let ray_direction = pixel_center - camera.camera_center();
            let r = Ray::new(camera.camera_center(), ray_direction);

            let pixel_color: Color = ray_color(r, &world);
            
            println!("{}", pixel_color);
        
        }
    }
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
mod vec3;
mod color;
mod ray;

use vec3::{Vec3, Point3};
use color::Color;
use ray::Ray;

fn hit_sphere(center: Point3, radius: f32, r: Ray) -> f32 {
    let oc = r.origin() - center;
    let a = r.direction().dot(&r.direction());
    let b = 2.0 * oc.dot(&r.direction());
    let c = oc.dot(&oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color (r: Ray) -> Color {

    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    // print!("{} ", t);
    if t > 0.0  {
        // println!("\n");
        let n: Vec3 = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        assert!(n.x() >= -1.0 && n.x() <= 1.0, "n.x out of range: {}", n.x());
        assert!(n.y() >= -1.0 && n.y() <= 1.0, "n.y out of range: {}", n.y());
        assert!(n.z() >= -1.0 && n.z() <= 1.0, "n.z out of range: {}", n.z());
        
        // println!("{}", *count);
        return Color::new(0.5 * n.x() + 0.5, 0.5 * n.y() + 0.5, 0.5 * n.z() + 0.5);
    }
    let unit_direction = r.direction().unit_vector();
    let a = 0.5*(unit_direction.y() + 1.0);
    (1.0-a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
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


fn main() {

    let mut debug_count = 0;
    
    let aspect_ratio: f32 = 16.0/9.0;
    let image_width: i32 = 400;

    
    let mut image_height: i32 = ((image_width as f32) / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1
    }
    

    // Camera
    let focal_length = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * (image_width as f32) / (image_height as f32);
    let camera_center = Point3::default();

    // Viewport
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);


    println!("P3\n{} {} \n255\n", image_width, image_height);
    
    for j in 0..image_height {
        for i in 0..image_width{
            let pixel_center = pixel00_loc + (pixel_delta_u * (i as f32)) + (pixel_delta_v * (j as f32));
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color: Color = ray_color(r);
            
            println!("{}", pixel_color);
        
        }
    }
    // println!("{}", debug_count);

}

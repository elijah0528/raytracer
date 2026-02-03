use crate::vec3::{Vec3, Point3};
use crate::color::Color;
use crate::ray::Ray;
use crate::constants::{INFINITY, random_generator};
use crate::interval::Interval;
use crate::hittable::{HitRecord, Hittable};

#[derive(Default)]
pub struct Camera {
    pub image_height: i32,
    pub image_width: i32,
    pub camera_center: Point3,
    pub pixel00_loc: Point3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub viewport_upper_left: Vec3,
    pub viewport_u: Vec3,
    pub viewport_v: Vec3,
    pub focal_length: f32,
    pub samples_per_pixel: f32,
    pub pixel_sample_scale: f32,
    pub max_recursive_depth: i32,
    pub background: Color,
}

impl Camera {
    pub fn new(image_height: i32) -> Self {
        let mut camera = Camera { 
            image_height, 
            background: Color::new(0.7, 0.8, 1.0), // Default sky blue
            ..Default::default() 
        };
        camera.initialize();
        camera
    }

    pub fn initialize(&mut self) {
        let aspect_ratio = 9.0 / 16.0;
        let width: i32 = ((self.image_height as f32) / aspect_ratio) as i32;

        self.image_width = if width < 1 { 1 } else { width };

        self.focal_length = 1.0;

        // Viewport dimensions
        self.viewport_height = 2.0;
        self.viewport_width = self.viewport_height * (self.image_width as f32) / (self.image_height as f32);

        self.viewport_u = Vec3::new(self.viewport_width, 0.0, 0.0);
        self.viewport_v = Vec3::new(0.0, -self.viewport_height, 0.0);

        self.pixel_delta_u = self.viewport_u / self.image_width as f32;
        self.pixel_delta_v = self.viewport_v / self.image_height as f32;

        self.camera_center = Point3::default();
        self.viewport_upper_left = self.camera_center 
            - Vec3::new(0.0, 0.0, self.focal_length) 
            - self.viewport_u / 2.0 
            - self.viewport_v / 2.0;
        self.pixel00_loc = self.viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        
        self.samples_per_pixel = 100.0;
        self.pixel_sample_scale = 1.0 / self.samples_per_pixel;
        self.max_recursive_depth = 50;
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_generator() - 0.5, random_generator() - 0.5, 0.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc 
            + (self.pixel_delta_u * (i as f32 + offset.x())) 
            + (self.pixel_delta_v * (j as f32 + offset.y()));
        let ray_direction = pixel_sample - self.camera_center;
        Ray::new(self.camera_center, ray_direction)
    }

    fn ray_color(&self, r: Ray, depth: i32, world: &dyn Hittable) -> Color {
        // Exceeded ray bounce limit, no more light gathered
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();
        let interval = Interval::new(0.001, INFINITY);

        if let Some(hit_record) = world.hit(r, interval, &mut rec) {
            // Get emission from material (for light sources)
            let emitted = if let Some(ref mat) = hit_record.material {
                mat.emitted(0.0, 0.0, &hit_record.p())
            } else {
                Color::new(0.0, 0.0, 0.0)
            };

            // Try to scatter the ray
            if let Some(ref mat) = hit_record.material {
                if let Some((attenuation, scattered)) = mat.scatter(&r, &hit_record) {
                    return emitted + attenuation * self.ray_color(scattered, depth - 1, world);
                }
            }
            
            // No scatter (e.g., light source), just return emission
            return emitted;
        }

        // No hit - return background/sky gradient
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..(self.samples_per_pixel as i32) {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(r, self.max_recursive_depth, world);
                }
                println!("{}", self.pixel_sample_scale * pixel_color);
            }
        }
        eprintln!("\rDone.                          ");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_new() {
        let c = Camera::new(400);
        assert_eq!(c.image_height, 400);
        assert_eq!(c.image_width, 711);
        assert_eq!(c.viewport_width, c.viewport_height * 1.7775);
    }
}

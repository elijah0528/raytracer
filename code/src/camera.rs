use crate::vec3::{Vec3, Point3};
use crate::color::Color;
use crate::ray::Ray;
use crate::constants::{INFINITY, random_generator_range, random_generator, linear_to_gamma};
use crate::interval::{Interval};
use crate::hittable::{HitRecord, HittableList, Hittable, Sphere};


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
}

impl Camera {
    pub fn new(image_height: i32) -> Self {
        let mut camera = Camera { image_height, ..Default::default() };
        camera.initialize();
        camera
    }
    
    pub fn initialize(&mut self) {

        let aspect_ratio = 9.0 / 16.0;
        let mut width: i32 = ((self.image_height as f32) / aspect_ratio) as i32;

        if width < 1 {
            self.image_width = 1;
        } else {
            self.image_width = width;
        }

        self.focal_length = 1.0;

        // Camera
        self.viewport_height = 2.0;
        self.viewport_width = self.viewport_height * (self.image_width as f32) / (self.image_height as f32);

        self.viewport_u = Vec3::new(self.viewport_width, 0.0, 0.0);
        self.viewport_v = Vec3::new(0.0, -self.viewport_height, 0.0);

        self.pixel_delta_u = self.viewport_u / self.image_width as f32;
        self.pixel_delta_v = self.viewport_v / self.image_height as f32;

        self.camera_center = Point3::default();
        // camera_center:Point3, focal_length: f32, viewport_height: f32, viewport_width: f32
        self.viewport_upper_left = self.camera_center - Vec3::new(0.0, 0.0, self.focal_length) - self.viewport_u/2.0 - self.viewport_v/2.0;
        self.pixel00_loc = self.viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        self.samples_per_pixel = 100.0;
        self.pixel_sample_scale = 1.0 / self.samples_per_pixel;
        self.max_recursive_depth = 50;
    }

    fn sample_square (&self) -> Vec3 {
        Vec3::new((random_generator() as f32) - 0.5, (random_generator() as f32) - 0.5, 0.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc + (self.pixel_delta_u * (i as f32 + offset.x())) + (self.pixel_delta_v * (j as f32 + offset.y()));
        let ray_direction = pixel_sample - self.camera_center;
        let r = Ray::new(self.camera_center, ray_direction);
        r

    }

    fn ray_color (&self, r: Ray, max_recursive_depth: i32, world: &dyn Hittable) -> Color {

        if max_recursive_depth <= 0 {
            return Color::new(0.0, 0.0, 0.0)
        }
        let mut rec: HitRecord = HitRecord::default();
        let interval = Interval::new(0.0001, INFINITY);
        let hit = world.hit(r, interval, &mut rec); 
        match hit {
            Some(hit_record) => {
                let n = hit_record.normal();
                let direction: Vec3 = Vec3::random_on_hemisphere(&n) + Vec3::random_unit_vector();
                
                return 0.5 * self.ray_color(Ray::new(hit_record.p(), direction), max_recursive_depth - 1,  world)
                // println!("{}", hit_record.normal());
                /* return Color::new(
                    0.5 * n.x() + 0.5,
                    0.5 * n.y() + 0.5,
                    0.5 * n.z() + 0.5,
                );    */     
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

    pub fn render(&mut self, world: &dyn Hittable) {

        println!("P3\n{} {} \n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width{
/*                 let pixel_center = self.pixel00_loc + (self.pixel_delta_u * (i as f32)) + (self.pixel_delta_v * (j as f32));
                let ray_direction = pixel_center - self.camera_center;
                let r = Ray::new(self.camera_center, ray_direction);

                let pixel_color: Color = self.ray_color(r, world);
                
                println!("{}", pixel_color);
 */
                let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
                for sample in 0..(self.samples_per_pixel as i32) {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(r, self.max_recursive_depth, world); 
                }
                println!("{}", self.pixel_sample_scale * pixel_color);
 
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_new(){
        let c: Camera = Camera::new(400);
        assert_eq!(c.image_height, 400);        
        assert_eq!(c.image_width, 711);
        assert_eq!(c.viewport_width, c.viewport_height * 1.7775);
        assert_eq!(c.viewport_width, c.viewport_height * 1.7775);
        assert_eq!(c.viewport_width, c.viewport_height * 1.7775);
        assert_eq!(c.viewport_width, c.viewport_height * 1.7775);

    }
}

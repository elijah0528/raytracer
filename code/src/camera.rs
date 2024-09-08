use crate::vec3::{Vec3, Point3};
use crate::color::Color;
use crate::ray::Ray;
use crate::constants::{INFINITY};
use crate::interval::{Interval};
use crate::hittable::{HitRecord, HittableList, Hittable, Sphere};
/* pub struct Camera {
    focal_length: f32,
    camera_center: Point3,
    viewport_upper_left: Point3,
    viewport_u: Vec3,
    viewport_v: Vec3
}

impl Camera {
    pub fn new (camera_center:Point3, focal_length: f32, viewport_height: f32, viewport_width: f32) -> Self {
        let u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);
        let center: Point3 = Point3::default();
        let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - u/2.0 - v/2.0;

        Camera { 
            focal_length,  
            camera_center: Point3::default(),
            viewport_upper_left: viewport_upper_left,
            viewport_u: u,
            viewport_v: v,
        }
    }

    pub fn focal_length(&self) -> f32 {
        self.focal_length
    }

    pub fn camera_center(&self) -> Point3 {
        self.camera_center
    }

    pub fn viewport_upper_left(&self) -> Point3 {
        self.viewport_upper_left
    }

    pub fn viewport_u(&self) -> Vec3 {
        self.viewport_u
    }

    pub fn viewport_v(&self) -> Vec3 {
        self.viewport_v
    }

    pub fn render (&self, &world) {

        let aspect_ratio: f32 = 16.0/9.0;
        let image_width: i32 = 400;

        let mut image_height: i32 = ((image_width as f32) / aspect_ratio) as i32;
        if image_height < 1 {
            image_height = 1
        }

        for j in 0..image_height {
            for i in 0..image_width{
                let pixel_center = pixel00_loc + (pixel_delta_u * (i as f32)) + (pixel_delta_v * (j as f32));
                let ray_direction = pixel_center - camera.camera_center();
                let r = Ray::new(camera.camera_center(), ray_direction);
    
                let pixel_color: Color = ray_color(r, &world);
                
                println!("{}", pixel_color);
            
            }
        }
    }
} */

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
}

impl Camera {
    pub fn new(image_height: i32) -> Self {
        let mut camera = Camera { image_height, ..Default::default() };
        camera.initialize();
        camera
    }
    
    pub fn initialize(&mut self) {

        let aspect_ratio = 1.0;
        let mut width: i32 = ((self.image_height as f32) / aspect_ratio) as i32;

        if width < 1 {
            self.image_width = 1;
        } else {
            self.image_width = width;
        }

        // Camera
        self.viewport_height = 2.0;
        self.viewport_width = self.viewport_height * (self.image_width as f32) / (self.image_height as f32);

        self.viewport_u = Vec3::new(self.viewport_width, 0.0, 0.0);
        self.viewport_v = Vec3::new(0.0, -self.viewport_height, 0.0);

        self.pixel_delta_u = self.viewport_u / self.image_width as f32;
        self.pixel_delta_v = self.viewport_v / self.image_height as f32;

        // camera_center:Point3, focal_length: f32, viewport_height: f32, viewport_width: f32
        self.viewport_upper_left = self.camera_center - Vec3::new(0.0, 0.0, self.focal_length) - self.viewport_u/2.0 - self.viewport_v/2.0;
        self.pixel00_loc = self.viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

    }

    fn ray_color (&self, r: Ray, world: &dyn Hittable) -> Color {

        let mut rec: HitRecord = HitRecord::default();
        let interval = Interval::new(0.0, INFINITY);
        let hit = world.hit(r, interval, &mut rec); 
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

    pub fn render(&mut self, world: &dyn Hittable) {

        println!("P3\n{} {} \n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width{
                let pixel_center = self.pixel00_loc + (self.pixel_delta_u * (i as f32)) + (self.pixel_delta_v * (j as f32));
                let ray_direction = pixel_center - self.camera_center;
                let r = Ray::new(self.camera_center, ray_direction);

                let pixel_color: Color = self.ray_color(r, world);
                
                println!("{}", pixel_color);
            
            }
        }
    }
}


mod tests {
    #[test]
    fn test_camera_new(){
        let c: Camera = Camera::new(400.0);
        assert_eq!(c.viewport_height, 2.0);

    }
}

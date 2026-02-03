use crate::vec3::{Vec3, Point3};
use crate::color::Color;
use crate::ray::Ray;
use crate::constants::{INFINITY, random_generator};
use crate::interval::Interval;
use crate::hittable::{HitRecord, Hittable};
use rayon::prelude::*;
use std::io::Write;

/// Camera with configurable position, orientation, FOV, and depth of field
pub struct Camera {
    // Image settings
    pub image_height: i32,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,

    // Camera position and orientation
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub vfov: f32, // Vertical field of view in degrees

    // Depth of field
    pub defocus_angle: f32, // Variation angle of rays through each pixel
    pub focus_dist: f32,    // Distance from camera to perfect focus plane

    // Computed values
    pixel_sample_scale: f32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3, // Camera basis vectors
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            image_height: 400,
            image_width: 711,
            samples_per_pixel: 100,
            max_depth: 50,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            vfov: 90.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
            pixel_sample_scale: 0.0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }
}

impl Camera {
    pub fn new(image_height: i32) -> Self {
        let mut camera = Camera {
            image_height,
            ..Default::default()
        };
        camera.initialize();
        camera
    }

    /// Initialize computed camera parameters
    pub fn initialize(&mut self) {
        // Calculate image width from aspect ratio (16:9)
        let aspect_ratio = 16.0 / 9.0;
        let width = ((self.image_height as f32) * aspect_ratio) as i32;
        self.image_width = if width < 1 { 1 } else { width };

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f32;
        self.center = self.lookfrom;

        // Viewport dimensions
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        // Calculate camera basis vectors
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        // Viewport edge vectors
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * (-self.v);

        // Pixel delta vectors
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Upper left pixel location
        let viewport_upper_left = self.center
            - (self.focus_dist * self.w)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Defocus disk basis vectors
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    /// Sample a random point in the unit square [-0.5, 0.5]
    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_generator() - 0.5, random_generator() - 0.5, 0.0)
    }

    /// Sample a random point in the unit disk
    fn sample_disk(&self) -> Vec3 {
        loop {
            let p = Vec3::new(
                random_generator() * 2.0 - 1.0,
                random_generator() * 2.0 - 1.0,
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Get a random point on the defocus disk
    fn defocus_disk_sample(&self) -> Point3 {
        let p = self.sample_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    /// Generate a ray for pixel (i, j) with random sampling
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f32 + offset.x()))
            + (self.pixel_delta_v * (j as f32 + offset.y()));

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    /// Calculate the color for a ray
    fn ray_color(&self, r: Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();
        let interval = Interval::new(0.001, INFINITY);

        if let Some(hit_record) = world.hit(r, interval, &mut rec) {
            // Get emission from material
            let emitted = if let Some(ref mat) = hit_record.material {
                mat.emitted(0.0, 0.0, &hit_record.p())
            } else {
                Color::new(0.0, 0.0, 0.0)
            };

            // Try to scatter
            if let Some(ref mat) = hit_record.material {
                if let Some((attenuation, scattered)) = mat.scatter(&r, &hit_record) {
                    return emitted + attenuation * self.ray_color(scattered, depth - 1, world);
                }
            }

            return emitted;
        }

        // Sky gradient background
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    /// Render the scene to stdout in PPM format
    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(r, self.max_depth, world);
                }
                println!("{}", self.pixel_sample_scale * pixel_color);
            }
        }
        eprintln!("\rDone.                          ");
    }

    /// Render the scene using parallel processing (multi-threaded)
    pub fn render_parallel(&self, world: &(dyn Hittable + Sync)) -> Vec<u8> {
        let width = self.image_width as usize;
        let height = self.image_height as usize;

        eprintln!("Rendering {}x{} image with {} samples/pixel...", width, height, self.samples_per_pixel);

        // Render all scanlines in parallel
        let pixels: Vec<Color> = (0..height)
            .into_par_iter()
            .flat_map(|j| {
                if j % 50 == 0 {
                    eprint!("\rScanlines remaining: {} ", height - j);
                    let _ = std::io::stderr().flush();
                }
                (0..width)
                    .map(|i| {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for _ in 0..self.samples_per_pixel {
                            let r = self.get_ray(i as i32, j as i32);
                            pixel_color = pixel_color + self.ray_color(r, self.max_depth, world);
                        }
                        self.pixel_sample_scale * pixel_color
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        eprintln!("\rDone.                          ");

        // Convert to RGB bytes
        let mut buffer = Vec::with_capacity(width * height * 3);
        for color in pixels {
            let (r, g, b) = color.to_rgb_bytes();
            buffer.push(r);
            buffer.push(g);
            buffer.push(b);
        }

        buffer
    }

    /// Render and save to PNG file
    pub fn render_to_png(&self, world: &(dyn Hittable + Sync), filename: &str) {
        let buffer = self.render_parallel(world);
        
        let img = image::RgbImage::from_raw(
            self.image_width as u32,
            self.image_height as u32,
            buffer,
        ).expect("Failed to create image from buffer");

        img.save(filename).expect("Failed to save image");
        eprintln!("Saved to {}", filename);
    }

    /// Render a single frame and return the buffer (for animation)
    pub fn render_frame(&self, world: &(dyn Hittable + Sync)) -> Vec<u8> {
        self.render_parallel(world)
    }

    /// Save a buffer to PNG
    pub fn save_buffer_to_png(&self, buffer: &[u8], filename: &str) {
        let img = image::RgbImage::from_raw(
            self.image_width as u32,
            self.image_height as u32,
            buffer.to_vec(),
        ).expect("Failed to create image from buffer");

        img.save(filename).expect("Failed to save image");
    }
}

/// Builder pattern for Camera configuration
pub struct CameraBuilder {
    camera: Camera,
}

impl CameraBuilder {
    pub fn new() -> Self {
        CameraBuilder {
            camera: Camera::default(),
        }
    }

    pub fn image_height(mut self, height: i32) -> Self {
        self.camera.image_height = height;
        self
    }

    pub fn samples_per_pixel(mut self, samples: i32) -> Self {
        self.camera.samples_per_pixel = samples;
        self
    }

    pub fn max_depth(mut self, depth: i32) -> Self {
        self.camera.max_depth = depth;
        self
    }

    pub fn lookfrom(mut self, point: Point3) -> Self {
        self.camera.lookfrom = point;
        self
    }

    pub fn lookat(mut self, point: Point3) -> Self {
        self.camera.lookat = point;
        self
    }

    pub fn vup(mut self, up: Vec3) -> Self {
        self.camera.vup = up;
        self
    }

    pub fn vfov(mut self, fov: f32) -> Self {
        self.camera.vfov = fov;
        self
    }

    pub fn defocus_angle(mut self, angle: f32) -> Self {
        self.camera.defocus_angle = angle;
        self
    }

    pub fn focus_dist(mut self, dist: f32) -> Self {
        self.camera.focus_dist = dist;
        self
    }

    pub fn build(mut self) -> Camera {
        self.camera.initialize();
        self.camera
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self::new()
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
    }

    #[test]
    fn test_camera_builder() {
        let cam = CameraBuilder::new()
            .image_height(200)
            .samples_per_pixel(50)
            .vfov(60.0)
            .lookfrom(Point3::new(0.0, 1.0, 2.0))
            .lookat(Point3::new(0.0, 0.0, 0.0))
            .build();

        assert_eq!(cam.image_height, 200);
        assert_eq!(cam.samples_per_pixel, 50);
        assert_eq!(cam.vfov, 60.0);
    }
}

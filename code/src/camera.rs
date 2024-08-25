use crate::vec3::{Vec3, Point3};
use crate::color::Color;
use crate::ray::Ray;

pub struct Camera {
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
}

use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::constants::random_generator;

/// Material trait for different surface types
pub trait Material: Send + Sync {
    /// Scatter an incoming ray, returning attenuation color and scattered ray
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    
    /// Emitted light (for light sources)
    fn emitted(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

/// Lambertian (diffuse) material
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal() + Vec3::random_unit_vector();
        
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal();
        }
        
        let scattered = Ray::new(rec.p(), scatter_direction);
        Some((self.albedo, scattered))
    }
}

/// Metal (reflective) material
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Metal { 
            albedo, 
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } 
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(ray_in.direction().unit_vector(), rec.normal());
        let scattered = Ray::new(rec.p(), reflected + self.fuzz * Vec3::random_unit_vector());
        
        if scattered.direction().dot(&rec.normal()) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

/// Dielectric (glass/water) material
pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Dielectric { refraction_index }
    }
    
    /// Schlick's approximation for reflectance
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        
        let unit_direction = ray_in.direction().unit_vector();
        let cos_theta = (-unit_direction).dot(&rec.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_generator() {
            Vec3::reflect(unit_direction, rec.normal())
        } else {
            Vec3::refract(unit_direction, rec.normal(), refraction_ratio)
        };
        
        let scattered = Ray::new(rec.p(), direction);
        Some((attenuation, scattered))
    }
}

/// Emissive (light-emitting) material
pub struct DiffuseLight {
    emit: Color,
}

impl DiffuseLight {
    pub fn new(emit: Color) -> Self {
        DiffuseLight { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None // Light sources don't scatter
    }
    
    fn emitted(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
        self.emit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lambertian_scatter() {
        let mat = Lambertian::new(Color::new(0.5, 0.5, 0.5));
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let rec = HitRecord::new(
            Point3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, 1.0),
            0.5,
            true,
            None,
        );
        
        let result = mat.scatter(&ray, &rec);
        assert!(result.is_some());
        let (attenuation, _scattered) = result.unwrap();
        assert_eq!(attenuation, Color::new(0.5, 0.5, 0.5));
    }
    
    #[test]
    fn test_metal_scatter() {
        let mat = Metal::new(Color::new(0.8, 0.8, 0.8), 0.0);
        let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        let rec = HitRecord::new(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            true,
            None,
        );
        
        let result = mat.scatter(&ray, &rec);
        assert!(result.is_some());
    }
    
    #[test]
    fn test_dielectric_scatter() {
        let mat = Dielectric::new(1.5);
        let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        let rec = HitRecord::new(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            true,
            None,
        );
        
        let result = mat.scatter(&ray, &rec);
        assert!(result.is_some());
    }
    
    #[test]
    fn test_diffuse_light_no_scatter() {
        let mat = DiffuseLight::new(Color::new(1.0, 1.0, 1.0));
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let rec = HitRecord::new(
            Point3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, 1.0),
            0.5,
            true,
            None,
        );
        
        let result = mat.scatter(&ray, &rec);
        assert!(result.is_none());
        
        let emitted = mat.emitted(0.0, 0.0, &Point3::new(0.0, 0.0, 0.0));
        assert_eq!(emitted, Color::new(1.0, 1.0, 1.0));
    }
}

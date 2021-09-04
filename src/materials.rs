use crate::{vectors::*, rays::*, hittable::*};
use rand::prelude::*;


pub trait Material {
    fn scatter(&self, rec: &HitRecord, scattered: &Ray) -> bool;
    fn get_attenuation(&self) -> Color;
    fn get_scatter_ray(&self, r_in: &Ray, rec: &HitRecord) -> Ray;
}


// Lambertian struct & implementations
#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color
}


impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian { 
        Lambertian {albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, _rec: &HitRecord, _scattered: &Ray) -> bool { 
        true 
    }


    fn get_attenuation(&self) -> Color {
        self.albedo//.clone()
    }


    fn get_scatter_ray(&self, r_in: &Ray, rec: &HitRecord) -> Ray{
        let mut scatter_direction: Vector3 = rec.normal + Vector3::random_unit_vector();

        // catch degenerate scatter direction 
        if scatter_direction.near_zero(){
            scatter_direction = rec.normal;
        }

        Ray::new(rec.p, scatter_direction, r_in.time)

    }
}



// Metal
#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}


impl Metal {
    pub fn new(albedo: Color, f: f32) -> Metal { 
        Metal {
            albedo, 
            fuzz: f.min(1.0)
        }
    }
}


impl Material for Metal {
    fn scatter(&self, rec: &HitRecord, scattered: &Ray) -> bool {
       Vector3::dot(&scattered.direction, &rec.normal) > 0.0
    }
    
    
    fn get_attenuation(&self) -> Color {
        self.albedo//.clone()
    }


    fn get_scatter_ray(&self, r_in: &Ray, rec: &HitRecord) -> Ray{
        let reflected = Vector3::reflect(&r_in.direction.normalized(), &rec.normal);
        Ray::new(rec.p, reflected + self.fuzz * Vector3::random_in_unit_sphere(), r_in.time)
    }
}

// Dielectric
#[derive(Copy, Clone)]
pub struct Dielectric {
    pub ir: f32,
}


impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Dielectric {
        Dielectric {ir: index_of_refraction}
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32{
        // Christophe Schlick's approximation for reflectance.
        let r0: f32 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powf(2.0);
        r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
    }
}

impl Material for Dielectric {
    fn scatter(&self, _rec: &HitRecord, _scattered: &Ray) -> bool {
        true
    }

    fn get_attenuation(&self) -> Color {Color::fromv(1.0)}

    fn get_scatter_ray(&self, r_in: &Ray, rec: &HitRecord) -> Ray {
        let refraction_ratio: f32 = if rec.front_face {1.0 / self.ir} else {self.ir};
        let unit_direction: Vector3 = r_in.direction.normalized();

        let cos_theta: f32 =  Vector3::dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta: f32 = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let mut rng = thread_rng();

        let direction: Vector3 = 
            if cannot_refract || (Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen::<f32>()) {
                Vector3::reflect(&unit_direction, &rec.normal)
            } else {
                Vector3::refract(&unit_direction, &rec.normal, refraction_ratio)
            };

        Ray::new(rec.p, direction, r_in.time)
    }
}

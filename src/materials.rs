use crate::{vectors::*, rays::*, hittable::*};


pub trait Material {
    fn scatter(&self, rec: &HitRecord, scattered: Ray) -> bool;
    fn get_attenuation(&self) -> Color;
    fn get_scatter_ray(&self, r_in: Ray, rec: &HitRecord) -> Ray;
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
    fn scatter(&self, _rec: &HitRecord, _scattered: Ray) -> bool { true }


    fn get_attenuation(&self) -> Color {
        self.albedo.clone()
    }


    fn get_scatter_ray(&self, r_in: Ray, rec: &HitRecord) -> Ray{
        let mut scatter_direction: Vector3 = rec.normal + Vector3::random_unit_vector();
        if scatter_direction.near_zero(){
            scatter_direction = rec.normal;
        }

        Ray::new(rec.p, scatter_direction)

    }
}



// Metal
#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
}


impl Metal {
    pub fn new(albedo: Color) -> Metal { 
        Metal {albedo}
    }
}


impl Material for Metal {
    fn scatter(&self, rec: &HitRecord, scattered: Ray) -> bool {
       Vector3::dot(scattered.direction, rec.normal) > 0.0
    }
    
    
    fn get_attenuation(&self) -> Color {
        self.albedo.clone()
    }


    fn get_scatter_ray(&self, r_in: Ray, rec: &HitRecord) -> Ray{
        let reflected = Vector3::reflect(r_in.direction.normalized(), rec.normal);
        Ray::new(rec.p, reflected)
    }
}
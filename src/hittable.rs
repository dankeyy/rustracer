use crate::{vectors::*, materials::*};
use crate::rays::Ray;
use std::sync::Arc;

// #[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub t: f32,
    pub front_face: bool,   
    pub mat: Arc<dyn Material>,
}


impl HitRecord {

    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::zeros(),
            normal: Vector3::zeros(),
            t: 0.0,
            front_face: false,
            mat: Arc::new(Lambertian::new(Color::zeros())),
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3) {
        self.front_face = Vector3::dot(&r.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {-outward_normal};
    }
}


pub trait Hittable{
   fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool; 
}
use crate::{vectors::*};
use crate::rays::Ray;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}


impl HitRecord {

    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::zeros(),
            normal: Vector3::zeros(),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vector3) {
        self.front_face = Vector3::dot(r.direction, outward_normal) > 0.0;
        self.normal = if self.front_face {outward_normal} else {-outward_normal};
    }
}


pub trait Hittable{
   fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool; 
}
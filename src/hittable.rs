use crate::{vectors::*};


#[derive(Copy, Clone)]
pub struct hit_record {
    pub p: Point3;
    pub normal: Vector3;
    pub t: f64;
    pub front_face: bool;
}


impl hit_record {
    fn set_face_normal(&self, r: Ray, outward_normal: Vector3) {
        self.front_face = Vector3::dot(r.direction, outward_normal);
        self.normal = if front_face {outward_normal} else {-outward_normal};
    }
}


#[derive(Copy, Clone)]
trait hittable{
   fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: hit_record) -> bool; 
}
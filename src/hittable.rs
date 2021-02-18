use crate::{vectors::*};

pub struct hit_record {
    pub p: Point3;
    pub normal: Vector3;
    pub t: f64;
}


trait hittable{
   fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: hit_record) -> bool; 
}
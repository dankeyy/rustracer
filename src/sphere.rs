use crate::{hittable::*, vectors::*};

pub struct sphere {
    center: Point3;
    radius: f64;
}

impl hittable for sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: hit_record) -> bool {
        
    }
}
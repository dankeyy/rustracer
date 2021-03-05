use crate::{hittable::*, vectors::*, rays::*, materials::*};
use std::sync::Arc;


pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}


impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        Sphere { center, radius, mat }
    }


}


impl Hittable for Sphere {

    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vector3 = r.origin - self.center;

        let a: f64 = r.direction.magnitude_squared(); // remember vec dotted with itself is the equivalent to its length squared
        let half_b: f64 = Vector3::dot(oc, r.direction); // 2 cancelled out
        let c: f64 = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c; // here too
        if discriminant < 0.0 {
            return false
        }

        let mut root = ( -half_b -discriminant.sqrt() ) / a;
        if root < t_min || root > t_max {
            root = ( -half_b +discriminant.sqrt() ) / a;
            if root < t_min || root > t_max {
                return false;
            }
        } 

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();

        return true;

    }

}
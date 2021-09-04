use crate::{hittable::*, vectors::*, rays::*, materials::*};
use std::sync::Arc;


pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub mat: Arc<dyn Material>,
}


impl MovingSphere {
    pub fn new(center0: Point3, center1: Point3, time0: f32, time1: f32, radius: f32, mat: Arc<dyn Material>) -> MovingSphere {
        MovingSphere { center0, center1, time0, time1, radius, mat }
    }

    pub fn center(&self, time: f32) -> Point3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }


}


impl Hittable for MovingSphere {

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: Vector3 = r.origin - self.center(r.time);

        let a: f32 = r.direction.magnitude_squared(); // remember vec dotted with itself is the equivalent to its length squared
        let half_b: f32 = Vector3::dot(&oc, &r.direction); // 2 cancelled out
        let c: f32 = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c; // here too
        if discriminant < 0.0 {
            return false
        }

        // find nearest root in the acceptable range
        let mut root = ( -half_b -discriminant.sqrt() ) / a;
        if root < t_min || root > t_max {
            root = ( -half_b +discriminant.sqrt() ) / a;
            if root < t_min || root > t_max {
                return false;
            }
        } 

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center(r.time)) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();

        return true;

    }

}
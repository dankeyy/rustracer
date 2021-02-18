use crate::{hittable::*, vectors::*};

pub struct sphere {
    center: Point3;
    radius: f64;
}


impl Sphere {
    fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }


}


impl hittable for sphere {

    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: hit_record) -> bool {
        let oc: Vector3 = r.origin - self.center;

        let a: f64 = r.direction.magnitude_squared(); // remember vec dotted with itself is the equivalent to its length squared
        let half_b: f64 = Vector3::dot(oc, r.direction); // 2 cancelled out
        let c: f64 = oc.magnitude_squared() - radius * radius;

        let discriminant = half_b * half_b - a * c; // here too
        if discriminant < 0.0 {
            return false
        }

        let squared: f64 = discriminant.sqrt();
        let root = ( -half_b -discriminant.sqrt() ) / a;
        if root < t_min || root > t_max {
            root = ( -half_b +discriminant.sqrt() ) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t)
        let outward_normal = (rec.p - self.center) / radius;
        rec.set_face_normal(r, outward_normal);

        return true;

    }

}
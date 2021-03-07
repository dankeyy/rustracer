use crate::vectors::{Vector3, Point3};


#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}


impl Ray {

    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }

}
use crate::vectors::{Vector3, Point3};


#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
    pub time: f32,
}


impl Ray {
    //                                             leaving at
    pub fn new(origin: Point3, direction: Vector3, time: f32) -> Ray {
        Ray { origin, direction, time }
    }

    pub fn at(self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }

}
use std::ops::*;

pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl Vector3 {

    pub fn zero() -> Vector3 {
        x = 0.0,
        y = 0.0,
        z = 0.0,
    }


    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { 
            x: x,
            y: y,
            z: z,
        }
    }


    pub fn cross(&u: Vector3, &v: &Vector3) -> Vector3 {
        Vector3 {
            x: u.y * v.z - v.y * u.z,
            y: u.x * v.z - v.x * u.z,
            z: u.x * v.y - v.x * u.y,
        }
    }


    pub fn dot(&u: Vector3, &v: &Vector3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }


    pub fn magnitude(&self) -> f64 {
        dot(&self, &self).sqrt()
    }

    



}
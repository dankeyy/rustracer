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


    pub fn normalize(&self) -> Vector3 {
        let n: f64 = self.magnitude();

        Vector3 { 
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }

}
impl Add for Vector3 {
    type Output = Vector3;

    fn add(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}



impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}


impl Div for Vector3 {
    type Output = Vector3;

    fn div(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}


impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: -self.x
            y: -self.y
            z: -self.z
        }
    }
}


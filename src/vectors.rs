use std::ops::*;


pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


// pub type Point = Vector3; // 3D point
pub type Color = Vector3; // RGB color


impl Vector3 {

    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { 
            x: x,
            y: y,
            z: z,
        }
    }


    pub fn from_val(v: f64) -> Vector3 {
        Vector3 { 
            x: v,
            y: v,
            z: v,
        }
    }


    pub fn zeros() -> Vector3 {
        Vector3::from_val(0.0)
    }


    pub fn cross(u: Vector3, v: Vector3) -> Vector3 {
        Vector3 {
            x: u.y * v.z - v.y * u.z,
            y: u.x * v.z - v.x * u.z,
            z: u.x * v.y - v.x * u.y,
        }
    }


    pub fn dot(u: Vector3, v: Vector3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }


    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }


    pub fn scaled(&self, n: f64) -> Vector3 {
        Vector3 {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }


    pub fn normalized(&self) -> Vector3 { // unit vector
        self.scaled(1.0 / self.magnitude())
    }


    pub fn print(&self) {
        println!("{} {} {}", self.x, self.y, self.z);
    }

}


impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}


impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}


impl Div for Vector3 {
    type Output = Vector3;

    fn div(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}


impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}


impl AddAssign for Vector3 {

    fn add_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}


impl SubAssign for Vector3 {

    fn sub_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


impl MulAssign for Vector3 {

    fn mul_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}


impl DivAssign for Vector3 {

    fn div_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}


impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

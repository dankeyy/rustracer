use std::ops::*;

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


pub type Point3 = Vector3; // 3D point
pub type Color = Vector3; // RGB color


impl Vector3 {

    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }

    }
    pub fn newi(x: i32, y: i32, z: i32) -> Vector3 {
        Vector3 { 
            x: x as f64,
            y: y as f64,
            z: z as f64,
        }

    }

    pub fn fromv(v: f64) -> Vector3 {
        Vector3 { 
            x: v,
            y: v,
            z: v,
        }
    }


    pub fn zeros() -> Vector3 {
        Vector3::fromv(0.0)
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

    pub fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
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


impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, n: f64) -> Vector3 {
        Vector3 {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }
}


impl Mul<i32> for Vector3 {
    type Output = Vector3;

    fn mul(self, n: i32) -> Vector3 {
        Vector3 {
            x: self.x * n as f64,
            y: self.y * n as f64,
            z: self.z * n as f64,
        }
    }
}


impl Add<f64> for Vector3 {
    type Output = Vector3;

    fn add(self, n: f64) -> Vector3 {
        Vector3 {
            x: self.x + n,
            y: self.y + n,
            z: self.z + n,
        }
    }
}


impl Add<Vector3> for f64 {
    type Output = Vector3;

    fn add(self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self + v.x,
            y: self + v.y,
            z: self + v.z,
        }
    }
}


impl Add<i32> for Vector3 {
    type Output = Vector3;

    fn add(self, n: i32) -> Vector3 {
        Vector3 {
            x: self.x + n as f64,
            y: self.y + n as f64,
            z: self.z + n as f64,
        }
    }
}

impl Add<Vector3> for i32 {
    type Output = Vector3;

    fn add(self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self as f64 + v.x,
            y: self as f64 + v.y,
            z: self as f64 + v.z,
        }
    }
}

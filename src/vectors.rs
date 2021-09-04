use std::ops::*;
use rand::prelude::*;


#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


pub type Point3 = Vector3; // 3D point
pub type Color = Vector3; // RGB color


impl Vector3 {

    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }

    }


    pub fn newi(x: i32, y: i32, z: i32) -> Vector3 {
        Vector3 { 
            x: x as f32,
            y: y as f32,
            z: z as f32,
        }

    }


    pub fn fromv(v: f32) -> Vector3 {
        Vector3 { 
            x: v,
            y: v,
            z: v,
        }
    }


    pub fn zeros() -> Vector3 {
        Vector3::fromv(0.0)
    }


    pub fn cross(u: &Vector3, v: &Vector3) -> Vector3 {
        Vector3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }


    pub fn dot(u: &Vector3, v: &Vector3) -> f32 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    
    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }


    pub fn scaled(&self, n: f32) -> Vector3 {
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


    pub fn random() -> Vector3 {
        let mut rng = thread_rng();

        Vector3 {
            x: rng.gen::<f32>(), 
            y: rng.gen::<f32>(), 
            z: rng.gen::<f32>(), 
        }
    }

    pub fn random_by_range(min: f32, max: f32) -> Vector3 {

        let mut rng = thread_rng();
        Vector3 {
            x: rng.gen_range(min..=max), 
            y: rng.gen_range(min..=max), 
            z: rng.gen_range(min..=max), 
        }
    }


    pub fn random_in_unit_sphere() -> Vector3 {
        loop {
            let p: Vector3 = Vector3::random_by_range(-1.0, 1.0);
            if p.magnitude_squared() < 1.0 {
                return p;
            }
        }
    }


    pub fn random_unit_vector() -> Vector3 {
        Vector3::random_in_unit_sphere().normalized()
    }


    pub fn random_in_hemisphere(normal: &Vector3) -> Vector3 {
        let in_unit_sphere: Vector3 = Vector3::random_unit_vector();

        if Vector3::dot(&in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }


    pub fn near_zero(&self) -> bool {
        let s: f32 = 1e-8;
        (self.x < s) && (self.y < s) && (self.z < s)
    }


    pub fn reflect(v: &Vector3, n: &Vector3) -> Vector3{
        (*v) - 2.0 * Vector3::dot(&v, &n) * (*n)
    }


    pub fn refract(uv: &Vector3, n: &Vector3, etai_over_etat: f32) -> Vector3{
        let cos_theta: f32 = Vector3::dot(&-*uv, n).min(1.0);

        let r_out_perp: Vector3 = etai_over_etat * ((*uv) + cos_theta * (*n));
        let r_out_parallel: Vector3 = (1.0 - r_out_perp.magnitude_squared())
                                      .abs()
                                      .sqrt()
                                      * (-1.0) 
                                      * (*n);

        r_out_perp + r_out_parallel

    }


    pub fn random_in_unit_disk() -> Vector3 {
        let mut rng = thread_rng();

        loop {
            let p = Vector3 {
                x: rng.gen_range(-1.0..1.0),
                y: rng.gen_range(-1.0..1.0), 
                z: 0.0
            };
            if p.magnitude_squared() < 1.0 {
                return p;
            }
        }
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


impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}


impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, n: f32) -> Vector3 {
        Vector3 {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }
}


impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, n: f32) -> Vector3 {
        Vector3 {
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }
}

impl Div<i32> for Vector3 {
    type Output = Vector3;

    fn div(self, n: i32) -> Vector3 {
        Vector3 {
            x: self.x / n as f32,
            y: self.y / n as f32,
            z: self.z / n as f32,
        }
    }
}

impl Mul<i32> for Vector3 {
    type Output = Vector3;

    fn mul(self, n: i32) -> Vector3 {
        Vector3 {
            x: self.x * n as f32,
            y: self.y * n as f32,
            z: self.z * n as f32,
        }
    }
}


impl Add<f32> for Vector3 {
    type Output = Vector3;

    fn add(self, n: f32) -> Vector3 {
        Vector3 {
            x: self.x + n,
            y: self.y + n,
            z: self.z + n,
        }
    }
}


impl Add<Vector3> for f32 {
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
            x: self.x + n as f32,
            y: self.y + n as f32,
            z: self.z + n as f32,
        }
    }
}

impl Add<Vector3> for i32 {
    type Output = Vector3;

    fn add(self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self as f32 + v.x,
            y: self as f32 + v.y,
            z: self as f32 + v.z,
        }
    }
}

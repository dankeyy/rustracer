use crate::{vectors::*, rays::*};

pub struct Camera {

    pub origin: Point3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub lower_left_corner: Point3,

}


impl Camera {

    pub fn new() -> Camera {

        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.0;

        let origin = Point3::zeros();
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - Vector3::new(0.0, 0.0, focal_length);
        
        Camera { origin, horizontal, vertical, lower_left_corner }

    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical)
    }

}
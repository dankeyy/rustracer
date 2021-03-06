use crate::{vectors::*, rays::*};

pub struct Camera {

    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,
    lens_radius: f64,
    u: Vector3,
    v: Vector3,
}


impl Camera {

    pub fn new(look_from: Point3, look_at: Point3, vup: Vector3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalized();
        let u = Vector3::cross(&vup, &w).normalized();
        let v = Vector3::cross(&w, &u);



        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Camera { origin, lower_left_corner, horizontal, vertical, lens_radius, u, v}

    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vector3 = self.lens_radius *Vector3::random_in_unit_disk();
        let offset: Vector3 = self.u * rd.x + self.v * rd.y;

        Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical -self.origin - offset)
    }

}
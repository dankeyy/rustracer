mod vectors;
mod colors;
mod rays;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;

use crate::{
            vectors::{Vector3, Color, Point3},
            colors::*,
            rays::Ray,
            hittable::*,
            hittable_list::*,
            sphere::*,
            camera::*,
};

extern crate rand;
// use rand::prelude::*;
// use rand::thread_rng;
// use rand::Rng;
// let mut rng = thread_rng();
// let y: f64 = rng.gen_range(-10.0, 10.0);


fn coloray(r: Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();

    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::newi(1,1,1))
    }
    let unit_direction: Vector3 = r.direction.normalized();
    let t: f64 = (unit_direction.y + 1.0) * 0.5;

    (1.0 - t) * Color::fromv(1.0) + t * Color::new(0.5, 0.7, 1.0)
}



fn main() {

    // image scaling
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
    

    // world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new( Point3::new(0.0, 0.0, -1.0), 0.5 )));
    world.add(Box::new(Sphere::new( Point3::new(0.0, -100.5, -1.0), 100.0 )));


    // camera
    // let viewport_height: f64 = 2.0;
    // let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    // let focal_length: f64 = 1.0;

    // let origin = Point3::zeros();
    // let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    // let vertical = Vector3::new(0.0, viewport_height, 0.0);
    // let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - Vector3::new(0.0, 0.0, focal_length);
    let cam = Camera::new();

    // render
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for j in (0..HEIGHT-1).rev() {
        eprintln!("\rLines remaining - {}", j);
        for i in 0..WIDTH {
            let u = i as f64 / (WIDTH - 1) as f64;
            let v = j as f64 / (HEIGHT - 1) as f64;

            // let r: Ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let r: Ray = cam.get_ray(u, v);
            let pixel_color: Color = coloray(r, &world);
            write_color(pixel_color);

        }
    }

    eprintln!("Done!")

}
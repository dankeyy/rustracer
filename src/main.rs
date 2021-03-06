mod vectors;
mod colors;
mod rays;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;
mod materials;

use crate::{
            vectors::{Vector3, Color, Point3},
            colors::*,
            rays::Ray,
            hittable::*,
            hittable_list::*,
            sphere::*,
            camera::*,
            materials::*,
};

extern crate rand;
use rand::prelude::*;
use std::sync::Arc;
// use std::f64::consts::PI;

fn coloray(r: Ray, world: &dyn Hittable, depth: u8) -> Color {
    if depth <= 0 { return Color::zeros(); }

    let mut rec = HitRecord::new();

    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        
        let attenuation: Color = rec.mat.get_attenuation();
        let scatter_ray: Ray = rec.mat.get_scatter_ray(r, &rec);

        if rec.mat.scatter(&rec, scatter_ray) {
            return attenuation * coloray(scatter_ray, world, depth -1);
        }

        // let target: Point3 = rec.p + Point3::random_in_unit_sphere();
        // return 0.5 * coloray(Ray::new(rec.p, target - rec.p), world, depth-1);//    (rec.normal + Color::fromv(1.0))
        return Color::zeros();
    }

    let unit_direction: Vector3 = r.direction.normalized();
    let t: f64 = (unit_direction.y + 1.0) * 0.5;

    return (1.0 - t) * Color::fromv(1.0) + t * Color::new(0.5, 0.7, 1.0);
}


fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground = Arc::new(Lambertian::new(Color::fromv(0.5)));
    world.add(Box::new(Sphere::new(Point3::new( 0.0, -1000.0, 0.0), 1000.0, ground)));

    let mut rng = thread_rng();
    
    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(i as f64 + 0.9 * rng.gen::<f64>(), 0.2, j as f64 + 0.9 * rng.gen::<f64>());
            
            let sphere_material: Arc<dyn Material>;

            if choose_mat < 0.8 {
                // diffuse
                let albedo = Color::random() * Color::random();
                sphere_material = Arc::new(Lambertian::new(albedo));

            } else if choose_mat < 0.95 {
                // metal
                let albedo = Color::random_by_range(0.5, 1.0);
                let fuzz = rng.gen_range(0.0..0.5);

                sphere_material = Arc::new(Metal::new(albedo, fuzz));

            } else {
                // glass
                sphere_material = Arc::new(Dielectric::new(1.5));
            }

            world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));

        }
    }


    let m1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Point3::newi(0, 1, 0), 1.0, m1)));

    let m2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Point3::newi(-4, 1, 0), 1.0, m2)));

    let m3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Point3::newi(4, 1, 0), 1.0, m3)));
    

    world
}


fn main() {

    // image scaling
    const ASPECT_RATIO: f64 = 3.0 / 2.0; //16.0 / 9.0;
    const WIDTH: u32 = 1200; //400;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_DEPTH: u8 = 50;

    // world
    let world = random_scene();
    // let mut world = HittableList::new();

    // let ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // let left   = Arc::new(Dielectric::new(1.5));
    // let right  = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    // world.add(Box::new(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, ground)));
    // world.add(Box::new(Sphere::new(Point3::new( 0.0,    0.0, -1.0),   0.5, center)));
    // world.add(Box::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.5, left.clone())));
    // world.add(Box::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0), -0.45, left)));
    // world.add(Box::new(Sphere::new(Point3::new( 1.0,    0.0, -1.0),   0.5, right)));

    // let r = (PI / 4.0).cos();
    // let left  = Arc::new(Lambertian::new(Color::newi(0,0,1)));
    // let right = Arc::new(Lambertian::new(Color::newi(1,0,0)));

    // world.add(Box::new(Sphere::new(Point3::new(-r, 0.0, -1.0), r, left)));
    // world.add(Box::new(Sphere::new(Point3::new(r, 0.0, -1.0), r, right)));


    // camera
    let look_from = Point3::newi(13, 2, 3);
    let look_at = Point3::newi(0, 0, 0);
    let vup = Vector3::newi(0, 1, 0);
    let vfov = 20.0;
    let aperture = 0.1;
    let focus_dist = 10.0; //(look_from - look_at).magnitude();

    let cam = Camera::new(look_from, look_at, vup, vfov, ASPECT_RATIO, aperture, focus_dist);

   // random setup
    let mut rng = thread_rng();

    // render
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);


    for j in (0..HEIGHT-1).rev() {
        eprintln!("\rLines remaining - {}", j);
        for i in 0..WIDTH {

            let mut pixel_color = Color::zeros();

            for _s   in 0..100 {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / ((WIDTH - 1) as f64);
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / ((HEIGHT - 1) as f64);

                let r: Ray = cam.get_ray(u, v);
                pixel_color += coloray(r, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("Done!")

}
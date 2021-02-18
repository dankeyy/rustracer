mod vectors;
mod colors;
mod rays;
use crate::vectors::{Vector3, Color, Point3};
use crate::colors::write_color;
use crate::rays::Ray;



fn sphere_hit(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc: Vector3 = r.origin - center;

    let a: f64 = r.direction.magnitude_squared(); // remember vec dotted with itself is the equivalent to its length squared
    let half_b: f64 = Vector3::dot(oc, r.direction); // 2 cancelled out
    let c: f64 = oc.magnitude_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c; // here too

    if discriminant < 0.0 {
        -1.0
    } else {
        ( -half_b - discriminant.sqrt() ) / a
    }
}



fn coloray(r: Ray) -> Color {

    let t =  sphere_hit(Point3::newi(0, 0, -1), 0.5, r);
        if t > 0.0 {
            let n = (r.at(t) - Vector3::newi(0, 0, -1)).normalized();
            return Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5
        }

    let unit_direction: Vector3 = r.direction.normalized();
    let t = (unit_direction.y + 1.0) * 0.5;

    (1.0 - t) * Color::fromv(1.0) + t * Color::new(0.5, 0.7, 1.0)
}



fn main() {
    // image scaling
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
    
    // camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    let focal_length: f64 = 1.0;

    let origin = Point3::zeros();
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - Vector3::new(0.0, 0.0, focal_length);
    

    
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for j in (0..HEIGHT-1).rev() {
        eprintln!("\rLines remaining - {}", j);
        for i in 0..WIDTH {
            let u = i as f64 / (WIDTH - 1) as f64;
            let v = j as f64 / (HEIGHT - 1) as f64;

            let r: Ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color: Color = coloray(r);
            write_color(pixel_color);

        }
    }

    eprintln!("Done!")

}
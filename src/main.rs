mod vectors;
use vectors::Vector3;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

fn main() {

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);
    let fac: f32 = 255.999;

    for j in (0..HEIGHT-1).rev() {
        eprintln!("\rLines remaining - {}", j);
        for i in 0..WIDTH {

            let r = (i as f32 / (WIDTH - 1) as f32 * fac) as u32;
            let g = (j as f32 / (HEIGHT - 1) as f32 * fac) as u32;
            let b = (0.25 * fac) as u32;

            println!("{} {} {}", r, g, b);
        }
    eprintln!("Done!")

    }

}
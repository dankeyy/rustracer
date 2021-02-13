mod vectors;
mod colors;

use crate::vectors::Color;
use crate::colors::*;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

fn main() {

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for j in (0..HEIGHT-1).rev() {
        eprintln!("\rLines remaining - {}", j);
        for i in 0..WIDTH {

            let pixels = Color::new(i as f64 / (WIDTH - 1) as f64, 
                                    j as f64 / (HEIGHT - 1) as f64,
                                    0.25);
            write_color(pixels);

        }
    }

    eprintln!("Done!")

}
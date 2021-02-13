
use crate::vectors::Color;


pub fn write_color(pixels: Color) {
        let fac: f64 = 255.999;

        println!("{} {} {}", 
                (pixels.x * fac) as u32,
                (pixels.y * fac) as u32,
                (pixels.z * fac) as u32,
        ) 
        
    }
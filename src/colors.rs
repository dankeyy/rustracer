
use crate::vectors::Color;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
        if x < min {min} else if x > max {max} else {x}
}

pub fn write_color(pixels: Color, samples_per_pixel: u32) {
        let scale: f64 = 1.0 / (samples_per_pixel as f64);

        let r: f64 = (pixels.x * scale).sqrt();
        let b: f64 = (pixels.y * scale).sqrt();
        let g: f64 = (pixels.z * scale).sqrt();


        println!("{} {} {}", 
                 (256.0 * clamp(r, 0.0, 0.999)) as u8,
                 (256.0 * clamp(b, 0.0, 0.999)) as u8,
                 (256.0 * clamp(g, 0.0, 0.999)) as u8,
        )
        
}
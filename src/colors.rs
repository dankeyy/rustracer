
use crate::vectors::Color;

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
        if x < min {min} else if x > max {max} else {x}
}

pub fn write_color(pixels: Color, samples_per_pixel: u32) {
        let scale: f32 = 1.0 / (samples_per_pixel as f32);

        let r: f32 = (pixels.x * scale).sqrt();
        let b: f32 = (pixels.y * scale).sqrt();
        let g: f32 = (pixels.z * scale).sqrt();


        println!("{} {} {}", 
                 (256.0 * clamp(r, 0.0, 0.999)) as u8,
                 (256.0 * clamp(b, 0.0, 0.999)) as u8,
                 (256.0 * clamp(g, 0.0, 0.999)) as u8,
        )

}
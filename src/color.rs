use crate::vec3::Color;

pub fn write_color(pixel_color: Color, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = (256.0 * (pixel_color.x * scale).sqrt().clamp(0.0, 0.999)) as u8;
    let g = (256.0 * (pixel_color.y * scale).sqrt().clamp(0.0, 0.999)) as u8;
    let b = (256.0 * (pixel_color.z * scale).sqrt().clamp(0.0, 0.999)) as u8;

    println!("{r} {g} {b}");
}

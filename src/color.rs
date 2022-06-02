use crate::vec3::Color;

pub fn write_color(pixel_color: Color) {
    let r = (pixel_color.x * 255.999) as u8;
    let g = (pixel_color.y * 255.999) as u8;
    let b = (pixel_color.z * 255.999) as u8;

    println!("{r} {g} {b}");
}

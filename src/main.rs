pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod vec3;

use std::rc::Rc;

use self::camera::Camera;
use self::color::write_color;
use self::hittable::{HitRecord, Hittable};
use self::hittable_list::HittableList;
use self::ray::Ray;
use self::rtweekend::random_f64;
use self::sphere::Sphere;
use self::vec3::{Color, Point3, Vec3};

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::default();

    if world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
        let target = rec.p + Vec3::random_in_hemisphere(rec.normal);

        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    let mut world = HittableList::default();

    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let cam = Camera::default();

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {j}");

        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_f64()) / (image_width - 1) as f64;
                let v = (j as f64 + random_f64()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);

                pixel_color += ray_color(&r, &world, max_depth);
            }

            write_color(pixel_color, samples_per_pixel);
        }
    }

    eprintln!("Done.");
}

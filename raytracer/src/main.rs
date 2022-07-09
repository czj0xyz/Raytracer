use std::f64::INFINITY;
// use std::f64::consts::PI;
use console::style;
use image::{ImageBuffer, RgbImage};
use rand::prelude::*;
use std::{fs::File, process::exit};

mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{unit_vector, Color, Point3};

// fn degrees_to_radians(degrees: f64) -> f64{
//     degrees * PI / 180.0
// }

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: usize = 2000;
const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 100;
const QUALITY: u8 = 100;

fn random_double() -> f64 {
    let mut rng = rand::thread_rng();

    rng.gen()
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

fn write_color(
    pixel_color: Color,
    samples_per_pixel: usize,
    img: &mut RgbImage,
    x: usize,
    y: usize,
) {
    let pixel = (*img).get_pixel_mut(x as u32, (HEIGHT - y - 1) as u32);
    let scale = 1.0 / samples_per_pixel as f64;
    let r = pixel_color.x() * scale;
    let g = pixel_color.y() * scale;
    let b = pixel_color.z() * scale;
    let res: [u8; 3] = [
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    ];
    *pixel = image::Rgb(res);
}
//Image

fn ray_color(r: Ray, world: &impl Hittable) -> Color {
    let mut rec: HitRecord = Default::default();
    if (*world).hit(r, 0.0, INFINITY, &mut rec) {
        (rec.normal + Color { e: [1.0; 3] }) * 0.5
    } else {
        let unit_dir = unit_vector(r.get_dir());
        let t = 0.5 * (unit_dir.y() + 1.0);
        Color { e: [1.0; 3] } * (1.0 - t) + Color { e: [0.5, 0.7, 1.0] } * t
    }
}

fn main() {
    //Image
    let path = "output/output.jpg";
    let mut img: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    //World
    let mut world: HittableList = Default::default();
    world.add(Box::new(Sphere {
        center: Point3 {
            e: [0.0, 0.0, -1.0],
        },
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Point3 {
            e: [0.0, -100.5, -1.0],
        },
        radius: 100.0,
    }));

    //Camera
    let cam: Camera = Default::default();

    //Render

    for j in (0..HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..WIDTH {
            let mut pixel_color: Color = Color { e: [0.0; 3] };
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / ((WIDTH - 1) as f64);
                let v = (j as f64 + random_double()) / ((HEIGHT - 1) as f64);
                let r: Ray = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, &world);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL, &mut img, i, j);
        }
    }
    eprintln!("Done !");
    //output
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(QUALITY)) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}

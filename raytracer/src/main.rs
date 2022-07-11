use console::style;
use image::{ImageBuffer, RgbImage};
use std::f64::consts::PI;
use std::f64::INFINITY;
use std::sync::Arc;
use std::{fs::File, process::exit};

mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::Lambertian;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{random_double, unit_vector, Color, Point3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: usize = 400;
const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 100;
const QUALITY: u8 = 100;
const MAXDEPTH: isize = 50;

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
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
    let r_color = (pixel_color.x() * scale).sqrt();
    let g_color = (pixel_color.y() * scale).sqrt();
    let b_color = (pixel_color.z() * scale).sqrt();
    let res: [u8; 3] = [
        (256.0 * clamp(r_color, 0.0, 0.999)) as u8,
        (256.0 * clamp(g_color, 0.0, 0.999)) as u8,
        (256.0 * clamp(b_color, 0.0, 0.999)) as u8,
    ];
    *pixel = image::Rgb(res);
}
//Image

fn ray_color(r: Ray, world: &impl Hittable, depth: isize) -> Color {
    if depth <= 0 {
        Color { e: [0.0; 3] }
    } else {
        let mut rec: HitRecord = Default::default();
        if (*world).hit(r, 0.001, INFINITY, &mut rec) {
            let mut scattered: Ray = Default::default();
            let mut attenuation: Color = Default::default();
            match rec.clone().mat_ptr {
                Some(x) => {
                    if x.scatter(r, rec, &mut attenuation, &mut scattered) {
                        attenuation.mul(ray_color(scattered, world, depth - 1))
                    } else {
                        Color { e: [0.0; 3] }
                    }
                }
                None => Color { e: [0.0; 3] },
            }
        } else {
            let unit_dir = unit_vector(r.get_dir());
            let t = 0.5 * (unit_dir.y() + 1.0);
            Color { e: [1.0; 3] } * (1.0 - t) + Color { e: [0.5, 0.7, 1.0] } * t
        }
    }
}

fn main() {
    // Image
    let path = "output/output.jpg";
    let mut img: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    //World
    let r_ = (PI / 4.0).cos();
    let mut world: HittableList = Default::default();
    let material_left = Arc::new(Lambertian {
        albedo: Color { e: [0.0, 0.0, 1.0] },
    });
    let material_right = Arc::new(Lambertian {
        albedo: Color { e: [1.0, 0.0, 0.0] },
    });

    world.add(Box::new(Sphere {
        center: Point3 {
            e: [-r_, 0.0, -1.0],
        },
        radius: r_,
        mat_ptr: Some(material_left),
    }));
    world.add(Box::new(Sphere {
        center: Point3 { e: [r_, 0.0, -1.0] },
        radius: r_,
        mat_ptr: Some(material_right),
    }));
    //Camera
    let cam = Camera::creat(90.0, ASPECT_RATIO);

    //Render

    for j in (0..HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..WIDTH {
            let mut pixel_color: Color = Color { e: [0.0; 3] };
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / ((WIDTH - 1) as f64);
                let v = (j as f64 + random_double()) / ((HEIGHT - 1) as f64);
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAXDEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL, &mut img, i, j);
        }
        let output_image = image::DynamicImage::ImageRgb8(img.clone());
        let mut output_file = File::create(path).unwrap();
        match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(QUALITY)) {
            Ok(_) => {}
            // Err(_) => panic!("Outputting image fails."),
            Err(_) => println!("{}", style("Outputting image fails.").red()),
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

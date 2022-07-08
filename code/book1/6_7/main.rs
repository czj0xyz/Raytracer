use std::f64::INFINITY;
// use std::f64::consts::PI;
use console::style;
use image::{ImageBuffer, RgbImage};
use std::{fs::File, process::exit};

mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{unit_vector, Color, Point3, Vec3};

// fn degrees_to_radians(degrees: f64) -> f64{
//     degrees * PI / 180.0
// }

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: usize = 400;
const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;
const QUALITY: u8 = 100;

fn write_color(pixel_color: Color, img: &mut RgbImage, x: usize, y: usize) {
    let pixel = (*img).get_pixel_mut(x as u32, (HEIGHT - y - 1) as u32);
    let res: [u8; 3] = [
        (255.999 * pixel_color.x()) as u8,
        (255.999 * pixel_color.y()) as u8,
        (255.999 * pixel_color.z()) as u8,
    ];
    *pixel = image::Rgb(res);
    // println!("{} {} {}",(255.999*pixel_color.x()) as usize,
    //                     (255.999*pixel_color.y()) as usize,
    //                     (255.999*pixel_color.z()) as usize);
}
//Image

fn ray_color(r: Ray, world: &impl Hittable) -> Color {
    let mut rec: HitRecord = Default::default();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
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
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3 { e: [0.0; 3] };
    let horizontal = Vec3 {
        e: [viewport_width, 0.0, 0.0],
    };
    let vertical = Vec3 {
        e: [0.0, viewport_height, 0.0],
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3 {
            e: [0.0, 0.0, focal_length],
        };

    //Render
    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("255");

    for j in (0..HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..WIDTH {
            let u = (i as f64) / ((WIDTH - 1) as f64);
            let v = (j as f64) / ((HEIGHT - 1) as f64);
            let r = Ray {
                st: origin,
                dir: lower_left_corner + horizontal * u + vertical * v - origin,
            };
            let pixdel_color = ray_color(r, &world);
            write_color(pixdel_color, &mut img, i, j);
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

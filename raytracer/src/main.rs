use console::style;
use image::{ImageBuffer, RgbImage};
use std::f64::INFINITY;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{fs::File, process::exit};

mod camera;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::moving_sphere::MovingSphere;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{random_double, random_double_lr, unit_vector, Color, Point3, Vec3};

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

fn random_scene() -> HittableList {
    let mut world: HittableList = Default::default();

    let ground_material = Arc::new(Lambertian {
        albedo: Color { e: [0.5; 3] },
    });
    world.add(Arc::new(Sphere {
        center: Point3 {
            e: [0.0, -1000.0, 0.0],
        },
        radius: 1000.0,
        mat_ptr: Some(ground_material),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center_ = Point3 {
                e: [
                    a as f64 + 0.9 * random_double(),
                    0.2,
                    b as f64 + 0.9 * random_double(),
                ],
            };
            if (center_ - Point3 { e: [4.0, 0.2, 0.0] }).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    let albedo_ = Vec3::random().mul(Vec3::random());
                    Arc::new(Lambertian { albedo: albedo_ })
                } else if choose_mat < 0.95 {
                    let albedo_ = Vec3::random_lr(0.5, 1.0);
                    let fuzz_ = random_double_lr(0.0, 0.5);
                    Arc::new(Metal::creat(albedo_, fuzz_))
                } else {
                    Arc::new(Dielectric { ir: 1.5 })
                };
                if choose_mat < 0.8 {
                    let center2_ = center_
                        + Vec3 {
                            e: [0.0, random_double_lr(0.0, 0.5), 0.0],
                        };
                    world.add(Arc::new(MovingSphere {
                        center0: center_,
                        center1: center2_,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        mat_ptr: Some(sphere_material),
                    }));
                } else {
                    world.add(Arc::new(Sphere {
                        center: center_,
                        radius: 0.2,
                        mat_ptr: Some(sphere_material),
                    }));
                }
            }
        }
    }
    world.add(Arc::new(Sphere {
        center: Point3 { e: [0.0, 1.0, 0.0] },
        radius: 1.0,
        mat_ptr: Some(Arc::new(Dielectric { ir: 1.5 })),
    }));

    world.add(Arc::new(Sphere {
        center: Point3 {
            e: [-4.0, 1.0, 0.0],
        },
        radius: 1.0,
        mat_ptr: Some(Arc::new(Lambertian {
            albedo: Color { e: [0.4, 0.2, 0.1] },
        })),
    }));

    world.add(Arc::new(Sphere {
        center: Point3 { e: [4.0, 1.0, 0.0] },
        radius: 1.0,
        mat_ptr: Some(Arc::new(Metal::creat(Color { e: [0.7, 0.6, 0.5] }, 0.0))),
    }));

    world
}

fn solve(cam: &Camera, world: &HittableList, j: usize) -> (usize, Vec<Color>) {
    let mut ret: Vec<Color> = Default::default();
    for i in 0..WIDTH {
        let mut pixel_color: Color = Color { e: [0.0; 3] };
        for _ in 0..SAMPLES_PER_PIXEL {
            let u = (i as f64 + random_double()) / ((WIDTH - 1) as f64);
            let v = (j as f64 + random_double()) / ((HEIGHT - 1) as f64);
            let r: Ray = (*cam).get_ray(u, v);
            pixel_color += ray_color(r, world, MAXDEPTH);
        }
        ret.push(pixel_color);
    }
    (j, ret)
}

fn main() {
    // Image
    let path = "output/output.jpg";
    let mut img: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    //World

    let world: HittableList = random_scene();

    //Camera
    let lookfrom = Point3 {
        e: [13.0, 2.0, 3.0],
    };
    let lookat = Point3 { e: [0.0, 0.0, 0.0] };
    let vup = Vec3 { e: [0.0, 1.0, 0.0] };
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::creat(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    //Render

    let mut handles = vec![];

    #[allow(clippy::mutex_atomic)]
    let lines = Arc::new(Mutex::new(0));

    for _ in 0..20 {
        let counter = Arc::clone(&lines);
        let cam_ = cam;
        let world_ = world.clone();
        let handle = thread::spawn(move || -> Vec<(usize, Vec<Color>)> {
            let mut ret: Vec<(usize, Vec<Color>)> = Default::default();
            loop {
                let mut num = counter.lock().unwrap();
                eprintln!("Scanlines remaining: {}", *num);
                if (*num) == HEIGHT {
                    break ret;
                }
                let y = *num;
                *num += 1;
                std::mem::drop(num);
                ret.push(solve(&cam_, &world_, y));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let color_vec = handle.join().unwrap();
        for obj in color_vec {
            let y = obj.0;
            for x in 0..WIDTH {
                write_color(obj.1[x], SAMPLES_PER_PIXEL, &mut img, x, y);
            }
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

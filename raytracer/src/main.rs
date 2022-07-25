use console::style;
use image::{ImageBuffer, RgbImage};
use rand::prelude::*;
use std::f64::INFINITY;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{fs::File, process::exit};

pub mod basic;
pub mod bvh;
pub mod hittable;
pub mod material;
pub mod texture;

use basic::{
    camera::Camera,
    clamp,
    ray::Ray,
    vec3::{dot, random_double, random_double_lr, unit_vector, Color, Point3, Vec3},
};
use bvh::BvhNode;
use hittable::{
    aarect::{XyRect, XzRect, YzRect},
    constant_medium::ConstantMedium,
    flip_face::FlipFace,
    hittable_list::HittableList,
    moving_sphere::MovingSphere,
    mybox::MyBox,
    rotate_y::RotateY,
    sphere::Sphere,
    translate::Translate,
    Hittable,
};
use material::{Dielectric, DiffuseLight, Lambertian, Metal};

use texture::{ImageTexture, NoiseTexture};

const QUALITY: u8 = 100;
const MAXDEPTH: isize = 50;

fn write_color(
    pixel_color: Color,
    samples_per_pixel: usize,
    img: &mut RgbImage,
    x: usize,
    y: usize,
    image_height: usize,
) {
    let pixel = (*img).get_pixel_mut(x as u32, (image_height - y - 1) as u32);
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
#[allow(clippy::unnecessary_unwrap)]
fn ray_color(r: Ray, background: Color, world: &impl Hittable, depth: isize) -> Color {
    if depth <= 0 {
        Color { e: [0.0; 3] }
    } else {
        let rec = (*world).hit(r, 0.001, INFINITY);
        if rec.is_some() {
            let rec = rec.unwrap();
            let mut scattered: Ray = Default::default();
            // let mut attenuation: Color = Default::default();
            let emitted = rec.mat_ptr.emitted(r, rec.clone(), rec.u, rec.v, rec.p);
            let mut pdf: f64 = 0.0;
            let mut albedo: Color = Default::default();
            if !rec
                .mat_ptr
                .scatter(r, rec.clone(), &mut albedo, &mut scattered, &mut pdf)
            {
                emitted
            } else {
                let on_light = Point3 {
                    e: [
                        random_double_lr(213.0, 343.0),
                        554.0,
                        random_double_lr(227.0, 332.0),
                    ],
                };
                let mut to_light = on_light - rec.p;
                let distance_squared = to_light.length_squared();
                to_light = unit_vector(to_light);

                if dot(to_light, rec.normal) < 0.0 {
                    return emitted;
                };

                let light_area = (343.0 - 213.0) * (332.0 - 227.0);
                let light_cosine = to_light.y().abs();

                if light_cosine < 0.000001 {
                    return emitted;
                };

                pdf = distance_squared / (light_cosine * light_area);
                scattered = Ray {
                    st: rec.p,
                    dir: to_light,
                    tm: r.get_time(),
                };

                emitted
                    + (albedo * rec.mat_ptr.scattering_pdf(r, rec, scattered)).mul(ray_color(
                        scattered,
                        background,
                        world,
                        depth - 1,
                    )) / pdf
            }
        } else {
            background
        }
    }
}

#[allow(dead_code)]
fn final_scene() -> HittableList {
    let mut rng = StdRng::seed_from_u64(19260817);
    let mut boxes1: HittableList = Default::default();
    let ground = Lambertian::creat(Color {
        e: [0.48, 0.83, 0.53],
    });

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 100.0 * rng.gen::<f64>() + 1.0; //random_double_lr(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Box::new(MyBox::creat(
                Point3 { e: [x0, y0, z0] },
                Point3 { e: [x1, y1, z1] },
                ground.clone(),
            )));
        }
    }

    let mut objects: HittableList = Default::default();
    objects.add(Box::new(BvhNode::creat(boxes1.objects, 0.0, 1.0)));

    let light = DiffuseLight::creat_color(Color { e: [7.0; 3] });
    objects.add(Box::new(XzRect {
        x0: 123.0,
        x1: 423.0,
        z0: 147.0,
        z1: 412.0,
        k: 554.0,
        mp: light,
    }));

    let center1_ = Point3 {
        e: [400.0, 400.0, 200.0],
    };
    let center2_ = center1_
        + Vec3 {
            e: [30.0, 0.0, 0.0],
        };
    let moving_sphere_material = Lambertian::creat(Color { e: [0.7, 0.3, 0.1] });
    objects.add(Box::new(MovingSphere {
        center0: center1_,
        center1: center2_,
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
        mat_ptr: moving_sphere_material,
    }));

    objects.add(Box::new(Sphere {
        center: Point3 {
            e: [260.0, 150.0, 45.0],
        },
        radius: 50.0,
        mat_ptr: Dielectric { ir: 1.5 },
    }));
    objects.add(Box::new(Sphere {
        center: Point3 {
            e: [0.0, 150.0, 145.0],
        },
        radius: 50.0,
        mat_ptr: Metal::creat(Color { e: [0.8, 0.8, 0.9] }, 1.0),
    }));

    let boundary = Sphere {
        center: Point3 {
            e: [360.0, 150.0, 145.0],
        },
        radius: 70.0,
        mat_ptr: Dielectric { ir: 1.5 },
    };
    objects.add(Box::new(boundary.clone()));
    objects.add(Box::new(ConstantMedium::creat2(
        boundary,
        0.2,
        Color { e: [0.2, 0.4, 0.9] },
    )));

    let boundary = Sphere {
        center: Point3 { e: [0.0; 3] },
        radius: 5000.0,
        mat_ptr: Dielectric { ir: 1.5 },
    };
    objects.add(Box::new(ConstantMedium::creat2(
        boundary,
        0.0001,
        Color { e: [1.0; 3] },
    )));

    let earth_texture = ImageTexture::creat("raytracer/src/picture/earthmap.jpg");
    let emt = Lambertian {
        albedo: earth_texture,
    };
    objects.add(Box::new(Sphere {
        center: Point3 {
            e: [400.0, 200.0, 400.0],
        },
        radius: 100.0,
        mat_ptr: emt,
    }));

    let pertext = NoiseTexture::creat(0.1);
    objects.add(Box::new(Sphere {
        center: Point3 {
            e: [220.0, 280.0, 300.0],
        },
        radius: 80.0,
        mat_ptr: Lambertian { albedo: pertext },
    }));

    let mut boxes2: HittableList = Default::default();
    let white = Lambertian::creat(Color {
        e: [0.73, 0.73, 0.73],
    });

    let ns: usize = 1000;

    for _ in 0..ns {
        boxes2.add(Box::new(Sphere {
            center: Vec3 {
                e: [
                    165.0 * rng.gen::<f64>(),
                    165.0 * rng.gen::<f64>(),
                    165.0 * rng.gen::<f64>(),
                ],
            }, //Vec3::random_lr(0.0,165.0)
            radius: 10.0,
            mat_ptr: white.clone(),
        }));
    }

    objects.add(Box::new(Translate {
        ptr: RotateY::creat(BvhNode::creat(boxes2.objects, 0.0, 1.0), 15.0),
        offset: Vec3 {
            e: [-100.0, 270.0, 395.0],
        },
    }));

    objects
}

#[allow(dead_code)]
fn cornell_box() -> HittableList {
    let mut objects: HittableList = Default::default();

    let red = Lambertian::creat(Color {
        e: [0.65, 0.05, 0.05],
    });
    let white = Lambertian::creat(Color {
        e: [0.73, 0.73, 0.73],
    });
    let green = Lambertian::creat(Color {
        e: [0.12, 0.45, 0.15],
    });

    let light = DiffuseLight::creat_color(Color { e: [15.0; 3] });

    objects.add(Box::new(YzRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        mp: green,
    }));

    objects.add(Box::new(YzRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mp: red,
    }));

    objects.add(Box::new(FlipFace {
        ptr: XzRect {
            x0: 213.0,
            x1: 343.0,
            z0: 227.0,
            z1: 332.0,
            k: 554.0,
            mp: light,
        },
    }));

    objects.add(Box::new(XzRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        mp: white.clone(),
    }));

    objects.add(Box::new(XzRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mp: white.clone(),
    }));

    objects.add(Box::new(XyRect {
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
        mp: white.clone(),
    }));

    let box1 = MyBox::creat(
        Point3 { e: [0.0; 3] },
        Point3 {
            e: [165.0, 330.0, 165.0],
        },
        white.clone(),
    );
    let box1 = RotateY::creat(box1, 15.0);
    let box1 = Translate {
        ptr: box1,
        offset: Vec3 {
            e: [265.0, 0.0, 295.0],
        },
    };
    objects.add(Box::new(box1));

    let box2 = MyBox::creat(Point3 { e: [0.0; 3] }, Point3 { e: [165.0; 3] }, white);
    let box2 = RotateY::creat(box2, -18.0);
    let box2 = Translate {
        ptr: box2,
        offset: Vec3 {
            e: [130.0, 0.0, 65.0],
        },
    };
    objects.add(Box::new(box2));

    objects
}

fn solve(
    cam: &Camera,
    world: &HittableList,
    j: usize,
    background: Color,
    samples_per_pixel: usize,
    image_width: usize,
    image_height: usize,
) -> (usize, Vec<Color>) {
    let mut ret: Vec<Color> = Default::default();
    for i in 0..image_width {
        let mut pixel_color: Color = Color { e: [0.0; 3] };
        for _ in 0..samples_per_pixel {
            let u = (i as f64 + random_double()) / ((image_width - 1) as f64);
            let v = (j as f64 + random_double()) / ((image_height - 1) as f64);
            let r: Ray = (*cam).get_ray(u, v);
            pixel_color += ray_color(r, background, world, MAXDEPTH);
        }
        ret.push(pixel_color);
    }
    (j, ret)
}

fn main() {
    // Image
    let aspect_ratio = 1.0;
    let image_width: usize = 600;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 10;

    //world
    let background: Color = Color { e: [0.0; 3] };

    //Camera
    let lookfrom = Point3 {
        e: [278.0, 278.0, -800.0],
    };
    let lookat = Point3 {
        e: [278.0, 278.0, 0.0],
    };
    let vup: Vec3 = Vec3 { e: [0.0, 1.0, 0.0] };
    let dist_to_focus = 10.0;
    let vfov = 40.0;
    let time0 = 0.0;
    let time1 = 1.0;
    let aperture = 0.0;
    let cam = Camera::creat(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        time0,
        time1,
    );
    let path = "output/output.jpg";
    let mut img: RgbImage = ImageBuffer::new(image_width as u32, image_height as u32);

    //Render

    let mut handles = vec![];

    #[allow(clippy::mutex_atomic)]
    let lines = Arc::new(Mutex::new(0));

    for _ in 0..32 {
        let counter = Arc::clone(&lines);
        let cam_ = cam;
        let world_ = cornell_box(); //world.clone();
        let background_ = background;
        let handle = thread::spawn(move || -> Vec<(usize, Vec<Color>)> {
            let mut ret: Vec<(usize, Vec<Color>)> = Default::default();
            loop {
                let mut num = counter.lock().unwrap();
                if (*num) == image_height {
                    break ret;
                }
                eprintln!("Scanlines remaining: {}", *num);
                let y = *num;
                *num += 1;
                std::mem::drop(num);
                ret.push(solve(
                    &cam_,
                    &world_,
                    y,
                    background_,
                    samples_per_pixel,
                    image_width,
                    image_height,
                ));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let color_vec = handle.join().unwrap();
        for obj in color_vec {
            let y = obj.0;
            for x in 0..image_width {
                write_color(obj.1[x], samples_per_pixel, &mut img, x, y, image_height);
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

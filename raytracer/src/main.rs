use console::style;
use image::{ImageBuffer, RgbImage};
use std::f64::INFINITY;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{fs::File, process::exit};

mod aabb;
mod aarect;
mod bvh;
mod camera;
mod constant_medium;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod mybox;
mod perlin;
mod ray;
mod sphere;
mod texture;
mod vec3;

use crate::aarect::{XyRect, XzRect, YzRect};
use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::constant_medium::ConstantMedium;
use crate::hittable::{HitRecord, Hittable, RotateY, Translate};
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::moving_sphere::MovingSphere;
use crate::mybox::Box;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::vec3::{clamp, random_double, random_double_lr, Color, Point3, Vec3};

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

fn ray_color(r: Ray, background: Color, world: &impl Hittable, depth: isize) -> Color {
    if depth <= 0 {
        Color { e: [0.0; 3] }
    } else {
        let mut rec: HitRecord = Default::default();
        if (*world).hit(r, 0.001, INFINITY, &mut rec) {
            let mut scattered: Ray = Default::default();
            let mut attenuation: Color = Default::default();
            let emitted = match rec.clone().mat_ptr {
                Some(x) => x.emitted(rec.u, rec.v, rec.p),
                None => Color { e: [0.0; 3] },
            };

            match rec.clone().mat_ptr {
                Some(x) => {
                    if !x.scatter(r, rec, &mut attenuation, &mut scattered) {
                        emitted
                    } else {
                        emitted
                            + attenuation.mul(ray_color(scattered, background, world, depth - 1))
                    }
                }
                None => Color { e: [0.0; 3] },
            }
        } else {
            background
        }
    }
}

fn random_scene() -> HittableList {
    let mut world: HittableList = Default::default();

    let checker = Arc::new(CheckerTexture::creat(
        Color { e: [0.2, 0.3, 0.1] },
        Color { e: [0.9; 3] },
    ));
    world.add(Arc::new(Sphere {
        center: Point3 {
            e: [0.0, -1000.0, 0.0],
        },
        radius: 1000.0,
        mat_ptr: Some(Arc::new(Lambertian {
            albedo: Some(checker),
        })),
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
                    Arc::new(Lambertian::creat(albedo_))
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
        mat_ptr: Some(Arc::new(Lambertian::creat(Color { e: [0.4, 0.2, 0.1] }))),
    }));

    world.add(Arc::new(Sphere {
        center: Point3 { e: [4.0, 1.0, 0.0] },
        radius: 1.0,
        mat_ptr: Some(Arc::new(Metal::creat(Color { e: [0.7, 0.6, 0.5] }, 0.0))),
    }));

    world
}

fn two_spheres() -> HittableList {
    let mut objects: HittableList = Default::default();
    let checker = Arc::new(CheckerTexture::creat(
        Color { e: [0.2, 0.3, 0.1] },
        Color { e: [0.9; 3] },
    ));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: [0.0, -10.0, 0.0],
        },
        radius: 10.0,
        mat_ptr: Some(Arc::new(Lambertian {
            albedo: Some(checker.clone()),
        })),
    }));

    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: [0.0, 10.0, 0.0],
        },
        radius: 10.0,
        mat_ptr: Some(Arc::new(Lambertian {
            albedo: Some(checker),
        })),
    }));

    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects: HittableList = Default::default();
    let pertext = Arc::new(NoiseTexture::creat(4.0));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: [0.0, -1000.0, 0.0],
        },
        radius: 1000.0,
        mat_ptr: Some(Arc::new(Lambertian {
            albedo: Some(pertext.clone()),
        })),
    }));

    objects.add(Arc::new(Sphere {
        center: Point3 { e: [0.0, 2.0, 0.0] },
        radius: 2.0,
        mat_ptr: Some(Arc::new(Lambertian {
            albedo: Some(pertext),
        })),
    }));

    objects
}

fn earth() -> HittableList {
    let mut objects: HittableList = Default::default();

    let earth_texture = Arc::new(ImageTexture::creat("raytracer/src/picture/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian {
        albedo: Some(earth_texture),
    });
    let globe = Arc::new(Sphere {
        center: Point3 { e: [0.0; 3] },
        radius: 2.0,
        mat_ptr: Some(earth_surface),
    });

    objects.add(globe);

    objects
}

fn simple_light() -> HittableList {
    let mut objects: HittableList = Default::default();
    let pertext = Arc::new(NoiseTexture::creat(4.0));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: [0.0, -1000.0, 0.0],
        },
        radius: 1000.0,
        mat_ptr: Some(Arc::new(Lambertian {
            albedo: Some(pertext.clone()),
        })),
    }));

    objects.add(Arc::new(Sphere {
        center: Point3 { e: [0.0, 2.0, 0.0] },
        radius: 2.0,
        mat_ptr: Some(Arc::new(Lambertian {
            albedo: Some(pertext),
        })),
    }));

    let difflight = Arc::new(DiffuseLight::creat_color(Color { e: [4.0; 3] }));

    objects.add(Arc::new(XyRect {
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0,
        mp: difflight.clone(),
    }));

    objects.add(Arc::new(Sphere {
        center: Point3 { e: [0.0, 7.0, 0.0] },
        radius: 2.0,
        mat_ptr: Some(difflight),
    }));

    objects
}

fn cornell_box() -> HittableList {
    let mut objects: HittableList = Default::default();

    let red = Arc::new(Lambertian::creat(Color {
        e: [0.65, 0.05, 0.05],
    }));
    let white = Arc::new(Lambertian::creat(Color {
        e: [0.73, 0.73, 0.73],
    }));
    let green = Arc::new(Lambertian::creat(Color {
        e: [0.12, 0.45, 0.15],
    }));

    let light = Arc::new(DiffuseLight::creat_color(Color { e: [15.0; 3] }));

    objects.add(Arc::new(YzRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        mp: green,
    }));

    objects.add(Arc::new(YzRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mp: red,
    }));

    objects.add(Arc::new(XzRect {
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 554.0,
        mp: light,
    }));

    objects.add(Arc::new(XzRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mp: white.clone(),
    }));

    objects.add(Arc::new(XzRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        mp: white.clone(),
    }));

    objects.add(Arc::new(XyRect {
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
        mp: white.clone(),
    }));

    let box1 = Arc::new(Box::creat(
        Point3 { e: [0.0; 3] },
        Point3 {
            e: [165.0, 330.0, 165.0],
        },
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::creat(box1, 15.0));
    let box1 = Arc::new(Translate {
        ptr: box1,
        offset: Vec3 {
            e: [265.0, 0.0, 295.0],
        },
    });
    objects.add(box1);

    let box2 = Arc::new(Box::creat(
        Point3 { e: [0.0; 3] },
        Point3 { e: [165.0; 3] },
        white,
    ));
    let box2 = Arc::new(RotateY::creat(box2, -18.0));
    let box2 = Arc::new(Translate {
        ptr: box2,
        offset: Vec3 {
            e: [130.0, 0.0, 65.0],
        },
    });
    objects.add(box2);

    objects
}

fn cornell_smoke() -> HittableList {
    let mut objects: HittableList = Default::default();

    let red = Arc::new(Lambertian::creat(Color {
        e: [0.65, 0.05, 0.05],
    }));
    let white = Arc::new(Lambertian::creat(Color {
        e: [0.73, 0.73, 0.73],
    }));
    let green = Arc::new(Lambertian::creat(Color {
        e: [0.12, 0.45, 0.15],
    }));

    let light = Arc::new(DiffuseLight::creat_color(Color { e: [7.0; 3] }));

    objects.add(Arc::new(YzRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        mp: green,
    }));

    objects.add(Arc::new(YzRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mp: red,
    }));

    objects.add(Arc::new(XzRect {
        x0: 113.0,
        x1: 443.0,
        z0: 127.0,
        z1: 432.0,
        k: 554.0,
        mp: light,
    }));

    objects.add(Arc::new(XzRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        mp: white.clone(),
    }));

    objects.add(Arc::new(XzRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mp: white.clone(),
    }));

    objects.add(Arc::new(XyRect {
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
        mp: white.clone(),
    }));

    let box1 = Arc::new(Box::creat(
        Point3 { e: [0.0; 3] },
        Point3 {
            e: [165.0, 330.0, 165.0],
        },
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::creat(box1, 15.0));
    let box1 = Arc::new(Translate {
        ptr: box1,
        offset: Vec3 {
            e: [265.0, 0.0, 295.0],
        },
    });
    objects.add(Arc::new(ConstantMedium::creat2(
        box1,
        0.01,
        Color { e: [0.0; 3] },
    )));

    let box2 = Arc::new(Box::creat(
        Point3 { e: [0.0; 3] },
        Point3 { e: [165.0; 3] },
        white,
    ));
    let box2 = Arc::new(RotateY::creat(box2, -18.0));
    let box2 = Arc::new(Translate {
        ptr: box2,
        offset: Vec3 {
            e: [130.0, 0.0, 65.0],
        },
    });
    objects.add(Arc::new(ConstantMedium::creat2(
        box2,
        0.01,
        Color { e: [1.0; 3] },
    )));

    objects
}

fn final_scene() -> HittableList {
    let mut boxes1: HittableList = Default::default();
    let ground = Arc::new(Lambertian::creat(Color {
        e: [0.48, 0.83, 0.53],
    }));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_lr(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(Box::creat(
                Point3 { e: [x0, y0, z0] },
                Point3 { e: [x1, y1, z1] },
                ground.clone(),
            )));
        }
    }

    let mut objects: HittableList = Default::default();
    objects.add(Arc::new(BvhNode::creat(
        &boxes1.objects,
        0,
        boxes1.objects.len(),
        0.0,
        1.0,
    )));

    let light = Arc::new(DiffuseLight::creat_color(Color { e: [7.0; 3] }));
    objects.add(Arc::new(XzRect {
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
    let moving_sphere_material = Arc::new(Lambertian::creat(Color { e: [0.7, 0.3, 0.1] }));
    objects.add(Arc::new(MovingSphere {
        center0: center1_,
        center1: center2_,
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
        mat_ptr: Some(moving_sphere_material),
    }));

    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: [260.0, 150.0, 45.0],
        },
        radius: 50.0,
        mat_ptr: Some(Arc::new(Dielectric { ir: 1.5 })),
    }));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: [0.0, 150.0, 145.0],
        },
        radius: 50.0,
        mat_ptr: Some(Arc::new(Metal::creat(Color { e: [0.8, 0.8, 0.9] }, 1.0))),
    }));

    let boundary = Arc::new(Sphere {
        center: Point3 {
            e: [360.0, 150.0, 145.0],
        },
        radius: 70.0,
        mat_ptr: Some(Arc::new(Dielectric { ir: 1.5 })),
    });
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::creat2(
        boundary,
        0.2,
        Color { e: [0.2, 0.4, 0.9] },
    )));

    let boundary = Arc::new(Sphere {
        center: Point3 { e: [0.0; 3] },
        radius: 5000.0,
        mat_ptr: Some(Arc::new(Dielectric { ir: 1.5 })),
    });
    objects.add(Arc::new(ConstantMedium::creat2(
        boundary,
        0.0001,
        Color { e: [1.0; 3] },
    )));

    let earth_texture = Arc::new(ImageTexture::creat("raytracer/src/picture/earthmap.jpg"));
    let emt = Arc::new(Lambertian {
        albedo: Some(earth_texture),
    });
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: [400.0, 200.0, 400.0],
        },
        radius: 100.0,
        mat_ptr: Some(emt),
    }));

    let pertext = Arc::new(NoiseTexture::creat(0.1));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: [220.0, 280.0, 300.0],
        },
        radius: 80.0,
        mat_ptr: Some(Arc::new(Lambertian {
            albedo: Some(pertext),
        })),
    }));

    let mut boxes2: HittableList = Default::default();
    let white = Arc::new(Lambertian::creat(Color {
        e: [0.73, 0.73, 0.73],
    }));

    let ns: usize = 1000;

    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere {
            center: Vec3::random_lr(0.0, 165.0),
            radius: 10.0,
            mat_ptr: Some(white.clone()),
        }));
    }

    objects.add(Arc::new(Translate {
        ptr: Arc::new(RotateY::creat(
            Arc::new(BvhNode::creat(
                &boxes2.objects,
                0,
                boxes2.objects.len(),
                0.0,
                1.0,
            )),
            15.0,
        )),
        offset: Vec3 {
            e: [-100.0, 270.0, 395.0],
        },
    }));

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
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width: usize = 400;
    let mut image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let mut samples_per_pixel = 100;
    //World
    let world: HittableList;
    let lookfrom: Point3;
    let lookat: Point3;
    let vfov;
    let mut aperture = 0.0;
    let background: Color;

    let opt = 0;
    match opt {
        1 => {
            world = random_scene();
            background = Color { e: [0.7, 0.8, 1.0] };
            lookfrom = Point3 {
                e: [13.0, 2.0, 3.0],
            };
            lookat = Point3 { e: [0.0; 3] };
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world = two_spheres();
            background = Color { e: [0.7, 0.8, 1.0] };
            lookfrom = Point3 {
                e: [13.0, 2.0, 3.0],
            };
            lookat = Point3 { e: [0.0; 3] };
            vfov = 20.0;
        }
        3 => {
            world = two_perlin_spheres();
            background = Color { e: [0.7, 0.8, 1.0] };
            lookfrom = Point3 {
                e: [13.0, 2.0, 3.0],
            };
            lookat = Point3 { e: [0.0; 3] };
            vfov = 20.0;
        }
        4 => {
            world = earth();
            background = Color { e: [0.7, 0.8, 1.0] };
            lookfrom = Point3 {
                e: [13.0, 2.0, 3.0],
            };
            lookat = Point3 { e: [0.0; 3] };
            vfov = 20.0;
        }
        5 => {
            world = simple_light();
            background = Color { e: [0.0, 0.0, 0.0] };
            lookfrom = Point3 {
                e: [26.0, 3.0, 6.0],
            };
            lookat = Point3 { e: [0.0, 2.0, 0.0] };
            samples_per_pixel = 400;
            vfov = 20.0;
        }
        6 => {
            aspect_ratio = 1.0;
            image_width = 600;
            image_height = (image_width as f64 / aspect_ratio) as usize;
            world = cornell_box();
            background = Color { e: [0.0, 0.0, 0.0] };
            lookfrom = Point3 {
                e: [278.0, 278.0, -800.0],
            };
            lookat = Point3 {
                e: [278.0, 278.0, 0.0],
            };
            samples_per_pixel = 200;
            vfov = 40.0;
        }
        7 => {
            world = cornell_smoke();
            aspect_ratio = 1.0;
            image_width = 600;
            image_height = (image_width as f64 / aspect_ratio) as usize;

            background = Color { e: [0.0, 0.0, 0.0] };
            lookfrom = Point3 {
                e: [278.0, 278.0, -800.0],
            };
            lookat = Point3 {
                e: [278.0, 278.0, 0.0],
            };
            samples_per_pixel = 200;
            vfov = 40.0;
        }
        _ => {
            world = final_scene();
            aspect_ratio = 1.0;
            image_width = 600;
            image_height = (image_width as f64 / aspect_ratio) as usize;
            samples_per_pixel = 1000;
            background = Color { e: [0.0, 0.0, 0.0] };
            lookfrom = Point3 {
                e: [478.0, 278.0, -600.0],
            };
            lookat = Point3 {
                e: [278.0, 278.0, 0.0],
            };
            vfov = 40.0;
        }
    }

    //Camera
    let vup = Vec3 { e: [0.0, 1.0, 0.0] };
    let dist_to_focus = 10.0;
    let cam = Camera::creat(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
    let path = "output/output.jpg";
    let mut img: RgbImage = ImageBuffer::new(image_width as u32, image_height as u32);

    //Render

    let mut handles = vec![];

    #[allow(clippy::mutex_atomic)]
    let lines = Arc::new(Mutex::new(0));

    for _ in 0..16 {
        let counter = Arc::clone(&lines);
        let cam_ = cam;
        let world_ = world.clone();
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

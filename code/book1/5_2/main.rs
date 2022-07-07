mod vec3;
mod ray;

use crate::vec3::{Vec3,Color,Point3,dot,cross,unit_vector};
use crate::ray::{Ray};

// use ray::Ray;
fn write_color(pixel_color : Color){
    println!("{} {} {}",(255.999*pixel_color.x()) as usize,
                        (255.999*pixel_color.y()) as usize,
                        (255.999*pixel_color.z()) as usize);
}
//Image
const ASPECT_RATIO : f64 = 16.0/9.0;
const WIDTH: usize = 400;
const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> bool{
    let oc = r.get_start() - center;
    let a = dot(r.get_dir(),r.get_dir());
    let b = 2.0 * dot(oc , r.get_dir());
    let c = dot(oc,oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn ray_color(r : Ray) -> Color{
    if hit_sphere(Point3{e:[0.0,0.0,-1.0]},0.5,r) {
        Color{e:[1.0,0.0,0.0]} 
    }else{
        let unit_dir = unit_vector(r.get_dir());
        let t = 0.5*(unit_dir.y() + 1.0);
        Color{e:[1.0;3]} * (1.0-t) + Color{e:[0.5,0.7,1.0]} *t
    }
}

fn main(){
    // //Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3{e:[0.0;3]};
    let horizontal =  Vec3{e:[viewport_width,0.0,0.0]};
    let vertical =  Vec3{e:[0.0,viewport_height,0.0]};
    let lower_left_corner = origin - horizontal/2.0- vertical/2.0 - Vec3{e:[0.0,0.0,focal_length]};


    // //Render
    println!("P3");
    println!("{} {}",WIDTH,HEIGHT);
    println!("255");

    for j in (0..HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}",j);

        for i in 0..WIDTH {
            let u = (i as f64) / ((WIDTH-1) as f64);
            let v = (j as f64) / ((HEIGHT-1) as f64);
            let r = Ray{ st: origin , dir: lower_left_corner + horizontal*u + vertical*v - origin};
            let color = ray_color(r);
            write_color(color);
        }

    }
    eprintln!("Done !");
}
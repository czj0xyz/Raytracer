mod vec3;
// mod ray;

use vec3::{Vec3,Color,Point3,dot,cross,unit_vector,Ray};

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

fn ray_color(r : Ray) -> Color{
    let unit_dir = unit_vector(r.get_dir());
    let t = 0.5*(unit_dir.y() + 1.0);
    Color{e:[1.0;3]} * (1.0-t) + Color{e:[0.5,0.7,1.0]} *t
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
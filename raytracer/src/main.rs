use std::ops::{Add, Mul, Sub, Div, AddAssign, SubAssign, MulAssign ,DivAssign};

// use std::fs::OpenOptions;
// use std::io::Write;


#[derive(Default,Copy,Clone,Debug)]
pub struct Vec3{
   pub e : [f64;3],
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self , other:Vec3) -> Vec3 {
        Vec3 { e:[self.e[0]+other.e[0],
               self.e[1]+other.e[1],
               self.e[2]+other.e[2]] }
    }
}

impl AddAssign for Vec3{
    fn add_assign(&mut self, other: Vec3){
        *self = Vec3 {
            e:[self.e[0]+other.e[0],
            self.e[1]+other.e[1],
            self.e[2]+other.e[2],]
        }
    }
}

impl Mul for Vec3 {
    type Output = f64;
    fn mul(self , other: Vec3) -> f64{
        self.e[0]*other.e[0]+self.e[1]*other.e[1]+self.e[2]*other.e[2]
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self , x: f64) -> Vec3{
        Vec3 { e:[self.e[0]*x,
               self.e[1]*x,
               self.e[2]*x] }
    }
}

impl MulAssign<f64> for Vec3{
    fn mul_assign(&mut self, other: f64){
        *self = Vec3 {
            e:[self.e[0]*other,
            self.e[1]*other,
            self.e[2]*other,]
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self , other:Vec3) -> Vec3 {
        Vec3 { e:[self.e[0]-other.e[0],
               self.e[1]-other.e[1],
               self.e[2]-other.e[2]] }
    }
}

impl SubAssign for Vec3{
    fn sub_assign(&mut self, other: Vec3){
        *self = Vec3 {
            e:[self.e[0]-other.e[0],
            self.e[1]-other.e[1],
            self.e[2]-other.e[2],]
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self , x: f64) -> Vec3{
       self * (1.0/x)
    }
}

impl DivAssign<f64> for Vec3{
    fn div_assign(&mut self, other: f64){
        *self = Vec3 {
            e:[self.e[0]/other,
            self.e[1]/other,
            self.e[2]/other,]
        }
    }
}

impl Vec3{
    pub fn length(&self) -> f64{
        (self.e[0]*self.e[0]+self.e[1]*self.e[1]+self.e[2]*self.e[2]).sqrt()
    }

    pub fn length_squared(&self) -> f64{
        self.e[0]*self.e[0]+self.e[1]*self.e[1]+self.e[2]*self.e[2]
    }

    pub fn x(&self) -> f64{
        self.e[0]
    }
    
    pub fn y(&self) -> f64{
        self.e[1]
    }
    
    pub fn z(&self) -> f64{
        self.e[2]
    }
}

pub fn dot(a: Vec3,b: Vec3) -> f64{
    a.e[0]*b.e[0]+a.e[1]*b.e[1]+a.e[2]*b.e[2]
}

pub fn cross(u: Vec3,v: Vec3) -> Vec3{
    Vec3 {e:[u.e[1]*v.e[2] - u.e[2]*v.e[1],
          u.e[2]*v.e[0] - u.e[0]*v.e[2],
          u.e[0]*v.e[1] - u.e[1]*v.e[0]] }
}

pub fn unit_vector(v: Vec3) -> Vec3{
    v/ v.length()
}

pub type Color = Vec3;  

pub type Point3 = Vec3;

#[derive(Default,Copy,Clone,Debug)]
pub struct Ray{
    pub st:Point3,
    pub dir:Vec3,
}

impl Ray{
    pub fn get_start(&self) -> Point3{
        self.st
    }
    pub fn get_dir(&self) -> Vec3{
        self.dir
    }
    pub fn at(&self,t: f64) -> Point3{
        self.st+self.dir*t
    }
}



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


use std::ops::{Add, Mul, Sub, Div, AddAssign, SubAssign, MulAssign ,DivAssign};
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


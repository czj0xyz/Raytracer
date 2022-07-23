pub mod aabb;

use crate::basic::{ray::Ray, vec3::random_int_lr};
use crate::hittable::{HitRecord, Hittable};
use aabb::{surrounding_box, Aabb};

use std::cmp::Ordering;
use std::process::exit;

#[derive(Default)]
pub struct BvhNode {
    pub left: Option<Box<dyn Hittable>>,
    pub right: Option<Box<dyn Hittable>>,
    pub box_: Aabb,
}
impl Hittable for BvhNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !(*self).box_.hit(r, t_min, t_max) {
            // eprintln!("OK");
            None
        } else {
            let hit_left = if (*self).left.is_some() {
                (*self).left.as_ref().unwrap().hit(r, t_min, t_max)
            } else {
                None
            };

            let hit_right = if (*self).right.is_some() {
                (*self).right.as_ref().unwrap().hit(
                    r,
                    t_min,
                    if hit_left.is_some() {
                        hit_left.as_ref().unwrap().t
                    } else {
                        t_max
                    },
                )
            } else {
                None
            };

            if hit_right.is_some() {
                hit_right
            } else if hit_left.is_some() {
                hit_left
            } else {
                None
            }
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut Aabb) -> bool {
        *output_box = (*self).box_;
        true
    }
}

fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> bool {
    let mut box_a: Aabb = Default::default();
    let mut box_b: Aabb = Default::default();
    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        eprintln!("No bounding box in bvh_node constructor.");
        exit(0);
    }
    box_a.min().e[axis] < box_b.min().e[axis]
}
#[allow(dead_code)]
fn box_x_compare(a: &dyn Hittable, b: &dyn Hittable) -> bool {
    box_compare(a, b, 0)
}
#[allow(dead_code)]
fn box_y_compare(a: &dyn Hittable, b: &dyn Hittable) -> bool {
    box_compare(a, b, 1)
}
#[allow(dead_code)]
fn box_z_compare(a: &dyn Hittable, b: &dyn Hittable) -> bool {
    box_compare(a, b, 2)
}

impl BvhNode {
    #[allow(dead_code)]
    #[allow(clippy::ptr_arg)]
    pub fn creat(src_objects: Vec<Box<dyn Hittable>>, t0: f64, t1: f64) -> BvhNode {
        let mut objects = src_objects;
        let axis = random_int_lr(0, 2);
        let cmp = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };
        let object_span = objects.len();
        let mut ret: BvhNode = Default::default();
        if object_span == 1 {
            let a = objects.pop().unwrap();
            ret.left = Some(a);
            ret.right = None;
        } else if object_span == 2 {
            let b = objects.pop().unwrap();
            let a = objects.pop().unwrap();
            if cmp(&(*a), &(*b)) {
                ret.left = Some(a);
                ret.right = Some(b);
            } else {
                ret.left = Some(b);
                ret.right = Some(a);
            }
        } else {
            //sort()
            objects.sort_by(|a, b| -> Ordering {
                if cmp(&(**a), &(**b)) {
                    Ordering::Less
                } else if cmp(&(**b), &(**a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            let mid = object_span >> 1;
            let mut left_vec = objects;
            let right_vec = left_vec.split_off(mid);

            ret.left = Some(Box::new(BvhNode::creat(left_vec, t0, t1)));
            ret.right = Some(Box::new(BvhNode::creat(right_vec, t0, t1)));
        }
        let mut box_left: Aabb = Default::default();
        let mut box_right: Aabb = Default::default();
        let fl = match ret.left.as_ref() {
            Some(x) => x.bounding_box(t0, t1, &mut box_left),
            None => false,
        };
        let fr = match ret.right.as_ref() {
            Some(x) => x.bounding_box(t0, t1, &mut box_right),
            None => false,
        };
        if !fl && !fr {
            eprintln!("No bounding box in bvh_node constructor.");
            exit(0);
        }
        ret.box_ = if !fl {
            surrounding_box(box_right, box_right)
        } else if !fr {
            surrounding_box(box_left, box_left)
        } else {
            surrounding_box(box_left, box_right)
        };
        ret
    }
}

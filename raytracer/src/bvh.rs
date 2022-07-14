use crate::aabb::{surrounding_box, Aabb};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::random_int_lr;
use std::cmp::Ordering;
use std::process::exit;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct BvhNode {
    pub left: Option<Arc<dyn Hittable>>,
    pub right: Option<Arc<dyn Hittable>>,
    pub box_: Aabb,
}
impl Hittable for BvhNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !(*self).box_.hit(r, t_min, t_max) {
            false
        } else {
            let hit_left: bool = match (*self).left.clone() {
                Some(res) => res.hit(r, t_min, t_max, rec),
                None => false,
            };
            let hit_right: bool = match (*self).right.clone() {
                Some(res) => res.hit(r, t_min, if hit_left { rec.t } else { t_max }, rec),
                None => false,
            };
            hit_left || hit_right
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut Aabb) -> bool {
        *output_box = (*self).box_;
        true
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> bool {
    let mut box_a: Aabb = Default::default();
    let mut box_b: Aabb = Default::default();
    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        eprintln!("No bounding box in bvh_node constructor.");
        exit(0);
    }
    box_a.min().e[axis] < box_b.min().e[axis]
}
#[allow(dead_code)]
fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
    box_compare(a, b, 0)
}
#[allow(dead_code)]
fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
    box_compare(a, b, 1)
}
#[allow(dead_code)]
fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
    box_compare(a, b, 2)
}

impl BvhNode {
    #[allow(dead_code)]
    #[allow(clippy::ptr_arg)]
    pub fn creat(
        src_objects: &Vec<Arc<dyn Hittable>>,
        st: usize,
        ed: usize,
        t0: f64,
        t1: f64,
    ) -> BvhNode {
        let mut objects = (*src_objects).clone();
        let axis = random_int_lr(0, 2);
        let cmp = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };
        let object_span = ed - st;
        let mut ret: BvhNode = Default::default();
        if object_span == 1 {
            ret.left = Some(objects[st].clone());
            ret.right = Some(objects[st].clone());
        } else if object_span == 2 {
            if cmp(&objects[st], &objects[st + 1]) {
                ret.left = Some(objects[st].clone());
                ret.right = Some(objects[st + 1].clone());
            } else {
                ret.left = Some(objects[st + 1].clone());
                ret.right = Some(objects[st].clone());
            }
        } else {
            //sort()
            let temp = &mut objects[st..ed];
            temp.sort_by(|a, b| -> Ordering {
                if cmp(a, b) {
                    Ordering::Less
                } else if cmp(b, a) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            let mid = st + object_span / 2;
            ret.left = Some(Arc::new(BvhNode::creat(&objects, st, mid, t0, t1)));
            ret.right = Some(Arc::new(BvhNode::creat(&objects, mid, ed, t0, t1)));
        }
        let mut box_left: Aabb = Default::default();
        let mut box_right: Aabb = Default::default();
        let fl = match ret.left.clone() {
            Some(x) => x.bounding_box(t0, t1, &mut box_left),
            None => false,
        };
        let fr = match ret.right.clone() {
            Some(x) => x.bounding_box(t0, t1, &mut box_right),
            None => false,
        };
        if !fl || !fr {
            eprintln!("No bounding box in bvh_node constructor.");
            exit(0);
        }
        ret.box_ = surrounding_box(box_left, box_right);
        ret
    }
}

use super::aarect::{XyRect, XzRect, YzRect};
use super::hittable_list::HittableList;
use crate::basic::{ray::Ray, vec3::Point3};
use crate::bvh::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

pub struct MyBox {
    pub box_min: Point3,
    pub box_max: Point3,
    pub sides: HittableList,
}

impl MyBox {
    pub fn creat<T: Material + Clone + 'static>(p0: Point3, p1: Point3, ptr: T) -> MyBox {
        let box_min_ = p0;
        let box_max_ = p1;
        let mut ret: HittableList = Default::default();

        ret.add(Box::new(XyRect {
            x0: p0.x(),
            x1: p1.x(),
            y0: p0.y(),
            y1: p1.y(),
            k: p1.z(),
            mp: ptr.clone(),
        }));
        ret.add(Box::new(XyRect {
            x0: p0.x(),
            x1: p1.x(),
            y0: p0.y(),
            y1: p1.y(),
            k: p0.z(),
            mp: ptr.clone(),
        }));

        ret.add(Box::new(XzRect {
            x0: p0.x(),
            x1: p1.x(),
            z0: p0.z(),
            z1: p1.z(),
            k: p1.y(),
            mp: ptr.clone(),
        }));
        ret.add(Box::new(XzRect {
            x0: p0.x(),
            x1: p1.x(),
            z0: p0.z(),
            z1: p1.z(),
            k: p0.y(),
            mp: ptr.clone(),
        }));

        ret.add(Box::new(YzRect {
            y0: p0.y(),
            y1: p1.y(),
            z0: p0.z(),
            z1: p1.z(),
            k: p1.x(),
            mp: ptr.clone(),
        }));
        ret.add(Box::new(YzRect {
            y0: p0.y(),
            y1: p1.y(),
            z0: p0.z(),
            z1: p1.z(),
            k: p0.x(),
            mp: ptr,
        }));

        MyBox {
            box_min: box_min_,
            box_max: box_max_,
            sides: ret,
        }
    }
}

impl Hittable for MyBox {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        (*self).sides.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb {
            min: (*self).box_min,
            max: (*self).box_max,
        };
        true
    }
}

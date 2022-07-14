use crate::vec3::{random_double, random_int_lr, Point3};

pub struct Perlin {
    pub ranfloat: Vec<f64>,
    pub perm_x: Vec<usize>,
    pub perm_y: Vec<usize>,
    pub perm_z: Vec<usize>,
}

const POINT_COUNT: usize = 256;

impl Perlin {
    fn permute(p: &mut Vec<usize>) {
        for i in (0..p.len()).rev() {
            let id = random_int_lr(0, i as isize) as usize;
            let mut a = p[i];
            let mut b = p[id];
            std::mem::swap(&mut a, &mut b);
            p[i] = a;
            p[id] = b;
        }
    }

    fn perlin_generate_perm() -> Vec<usize> {
        let mut ret: Vec<usize> = Default::default();
        for i in 0..POINT_COUNT {
            ret.push(i);
        }
        Perlin::permute(&mut ret);
        ret
    }
    pub fn noise(&self, p: Point3) -> f64 {
        // if p.x()<0.0 || p.y()<0.0 || p.z()<0.0 {
        //     eprintln!("????");
        // }
        let i = (((4.0 * p.x()) as isize) & 255) as usize;
        let j = (((4.0 * p.y()) as isize) & 255) as usize;
        let k = (((4.0 * p.z()) as isize) & 255) as usize;

        (*self).ranfloat[(*self).perm_x[i] ^ (*self).perm_y[j] ^ (*self).perm_z[k]]
    }
}

impl Default for Perlin {
    fn default() -> Perlin {
        let mut ret: Vec<f64> = Default::default();

        for _ in 0..POINT_COUNT {
            ret.push(random_double());
        }

        Perlin {
            ranfloat: ret,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }
}

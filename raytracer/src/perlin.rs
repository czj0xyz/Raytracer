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

    fn trilinear_interp(c: Vec<f64>, u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += ((i as f64) * u + ((1 - i) as f64) * (1.0 - u))
                        * ((j as f64) * v + ((1 - j) as f64) * (1.0 - v))
                        * ((k as f64) * w + ((1 - k) as f64) * (1.0 - w))
                        * c[i << 2 | j << 1 | k];
                }
            }
        }
        accum
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let mut u_fl = p.x() - p.x().floor();
        let mut v_fl = p.y() - p.y().floor();
        let mut w_fl = p.z() - p.z().floor();
        u_fl = u_fl.powi(2) * (3.0 - 2.0 * u_fl);
        v_fl = v_fl.powi(2) * (3.0 - 2.0 * v_fl);
        w_fl = w_fl.powi(2) * (3.0 - 2.0 * w_fl);

        let i = p.x().floor() as isize;
        let j = p.y().floor() as isize;
        let k = p.z().floor() as isize;
        let mut ret: Vec<f64> = Default::default();
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    ret.push(
                        (*self).ranfloat[(*self).perm_x[((i + di) & 255) as usize]
                            ^ (*self).perm_y[((j + dj) & 255) as usize]
                            ^ (*self).perm_z[((k + dk) & 255) as usize]],
                    );
                }
            }
        }
        Perlin::trilinear_interp(ret, u_fl, v_fl, w_fl)
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

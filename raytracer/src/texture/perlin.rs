use crate::basic::vec3::{dot, random_int_lr, unit_vector, Point3, Vec3};

#[derive(Clone)]
pub struct Perlin {
    pub ranvec: Vec<Vec3>,
    pub perm_x: Vec<usize>,
    pub perm_y: Vec<usize>,
    pub perm_z: Vec<usize>,
}

const POINT_COUNT: usize = 256;

impl Perlin {
    pub fn turb(&self, p: Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * (*self).noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }

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

    fn perlin_interp(c: Vec<Vec3>, u: f64, v: f64, w: f64) -> f64 {
        let uu = u.powi(2) * (3.0 - 2.0 * u);
        let vv = v.powi(2) * (3.0 - 2.0 * v);
        let ww = w.powi(2) * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += ((i as f64) * uu + ((1 - i) as f64) * (1.0 - uu))
                        * ((j as f64) * vv + ((1 - j) as f64) * (1.0 - vv))
                        * ((k as f64) * ww + ((1 - k) as f64) * (1.0 - ww))
                        * dot(
                            c[(i << 2) | (j << 1) | k],
                            Vec3 {
                                e: [u - i as f64, v - j as f64, w - k as f64],
                            },
                        );
                }
            }
        }
        accum
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u_fl = p.x() - p.x().floor();
        let v_fl = p.y() - p.y().floor();
        let w_fl = p.z() - p.z().floor();

        let i = p.x().floor() as isize;
        let j = p.y().floor() as isize;
        let k = p.z().floor() as isize;
        let mut ret: Vec<Vec3> = Default::default();
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    ret.push(
                        (*self).ranvec[(*self).perm_x[((i + di) & 255) as usize]
                            ^ (*self).perm_y[((j + dj) & 255) as usize]
                            ^ (*self).perm_z[((k + dk) & 255) as usize]],
                    );
                }
            }
        }
        Perlin::perlin_interp(ret, u_fl, v_fl, w_fl)
    }
}

impl Default for Perlin {
    fn default() -> Perlin {
        let mut ret: Vec<Vec3> = Default::default();

        for _ in 0..POINT_COUNT {
            ret.push(unit_vector(Vec3::random_lr(-1.0, 1.0)));
        }

        Perlin {
            ranvec: ret,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }
}

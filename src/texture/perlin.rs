use crate::{
    linalg::{Point3, Vec3},
    random::random_unit_vector,
};

const POINT_COUNT: usize = 256;

#[derive(Debug)]
pub(crate) struct Perlin {
    rand: [Vec3; POINT_COUNT],
    perm_x: [u8; POINT_COUNT],
    perm_y: [u8; POINT_COUNT],
    perm_z: [u8; POINT_COUNT],
}

impl Perlin {
    fn new() -> Self {
        let mut res = Self {
            rand: [Vec3::ZERO; POINT_COUNT],
            perm_x: [0; POINT_COUNT],
            perm_y: [0; POINT_COUNT],
            perm_z: [0; POINT_COUNT],
        };
        for v in res.rand.iter_mut() {
            *v = random_unit_vector();
        }
        Self::generate(&mut res.perm_x);
        Self::generate(&mut res.perm_y);
        Self::generate(&mut res.perm_z);
        res
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as isize;
        let j = p.y.floor() as isize;
        let k = p.z.floor() as isize;

        let mut c = [[[Vec3::ZERO; 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cij) in ci.iter_mut().enumerate() {
                for (dk, cijk) in cij.iter_mut().enumerate() {
                    *cijk = self.rand[(self.perm_x[((i + di as isize) & 0xff) as usize]
                        ^ self.perm_y[((j + dj as isize) & 0xff) as usize]
                        ^ self.perm_z[((k + dk as isize) & 0xff) as usize])
                        as usize];
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    pub fn turbulence(&self, p: Point3, depth: usize) -> f64 {
        let mut acc = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        acc.abs()
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut acc = 0.0;
        for (i, ci) in c.iter().enumerate() {
            for (j, cij) in ci.iter().enumerate() {
                for (k, cijk) in cij.iter().enumerate() {
                    let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    acc += ((i as f64) * uu + (1.0 - (i as f64)) * (1.0 - uu))
                        * ((j as f64) * vv + (1.0 - (j as f64)) * (1.0 - vv))
                        * ((k as f64) * ww + (1.0 - (k as f64)) * (1.0 - ww))
                        * cijk.dot(&weight);
                }
            }
        }
        acc
    }

    fn generate(p: &mut [u8]) {
        for (i, v) in p.iter_mut().enumerate() {
            *v = i as u8;
        }
        Self::permute(p);
    }

    fn permute(p: &mut [u8]) {
        for i in (1..POINT_COUNT).rev() {
            let tgt = fastrand::u8(0..i as u8);
            p.swap(i, tgt as usize);
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

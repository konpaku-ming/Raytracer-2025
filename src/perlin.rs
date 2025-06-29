use crate::random::{random_double, random_int_range};
use crate::vec3::Point3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    rand_float: [f64; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rand_float = [0.0; POINT_COUNT];
        for val in rand_float.iter_mut().take(POINT_COUNT) {
            *val = random_double();
        }

        Self {
            rand_float,
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[0.0f64; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idx = self.perm_x[((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize];
                    c[di as usize][dj as usize][dk as usize] = self.rand_float[idx as usize];
                }
            }
        }
        Perlin::trilinear(c, u, v, w)
    }

    fn generate_perm() -> [i32; POINT_COUNT] {
        let mut p: [i32; POINT_COUNT] = [0; POINT_COUNT];
        for (i, val) in p.iter_mut().enumerate() {
            *val = i as i32;
        }
        Perlin::permute(&mut p);
        p
    }

    fn permute(p: &mut [i32; POINT_COUNT]) {
        for i in (1..POINT_COUNT).rev() {
            let target = random_int_range(0, i as i32);
            p.swap(i, target as usize);
        }
    }

    fn trilinear(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for (i, plane) in c.iter().enumerate() {
            let weight_u = if i == 1 { u } else { 1.0 - u };
            for (j, row) in plane.iter().enumerate() {
                let weight_v = if j == 1 { v } else { 1.0 - v };
                for (k, value) in row.iter().enumerate() {
                    let weight_w = if k == 1 { w } else { 1.0 - w };
                    accum += weight_u * weight_v * weight_w * value;
                }
            }
        }
        accum
    }
}
impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

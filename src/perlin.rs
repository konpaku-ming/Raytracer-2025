use crate::random::random_int_range;
use crate::vec3::{Point3, Vec3, dot, unit_vector};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    rand_vec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rand_vec = [Vec3::default(); POINT_COUNT];
        for val in rand_vec.iter_mut().take(POINT_COUNT) {
            *val = unit_vector(&Vec3::random_range(-1.0, 1.0));
        }

        Self {
            rand_vec,
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idx = self.perm_x[((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize];
                    c[di as usize][dj as usize][dk as usize] = self.rand_vec[idx as usize];
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

    fn trilinear(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for (i, plane) in c.iter().enumerate() {
            let weight_u = if i == 1 { uu } else { 1.0 - uu };
            for (j, row) in plane.iter().enumerate() {
                let weight_v = if j == 1 { vv } else { 1.0 - vv };
                for (k, value) in row.iter().enumerate() {
                    let weight_w = if k == 1 { ww } else { 1.0 - ww };
                    let weight_vec = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += weight_u * weight_v * weight_w * dot(value, &weight_vec);
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

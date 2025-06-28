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
        let i = ((4.0 * p.x()) as i32 & 255) as usize;
        let j = ((4.0 * p.y()) as i32 & 255) as usize;
        let k = ((4.0 * p.z()) as i32 & 255) as usize;

        let idx = (self.perm_x[i] ^ self.perm_y[j]) ^ self.perm_z[k];
        self.rand_float[idx as usize]
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
}
impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

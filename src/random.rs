use crate::vec3::{Vec3, dot, unit_vector};
use once_cell::sync::Lazy;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use std::sync::Mutex;

static RNG: Lazy<Mutex<SmallRng>> = Lazy::new(|| Mutex::new(SmallRng::seed_from_u64(42)));

pub fn random_double() -> f64 {
    RNG.lock().unwrap().random_range(0.0..1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn random_int_range(min: i32, max: i32) -> i32 {
    random_double_range(min as f64, (max + 1) as f64) as i32
}

pub fn random() -> Vec3 {
    Vec3 {
        e: [random_double(), random_double(), random_double()],
    }
}

pub fn random_range(min: f64, max: f64) -> Vec3 {
    Vec3 {
        e: [
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        ],
    }
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = random_range(-1.0, 1.0);
        let len_sq = p.length_squared();
        if len_sq > 1e-160 && len_sq <= 1.0 {
            return unit_vector(&p);
        }
    }
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(&on_unit_sphere, &normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_double_range(-1.0, 1.0),
            random_double_range(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

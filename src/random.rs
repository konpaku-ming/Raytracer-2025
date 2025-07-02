use crate::vec3::{Vec3, dot, unit_vector};
use rand::{Rng, rng};
use std::f64::consts::PI;

pub fn random_double() -> f64 {
    rng().random_range(0.0..1.0)
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

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_double();
    let r2 = random_double();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    let z = (1.0 - r2).sqrt();

    Vec3::new(x, y, z)
}

use rand::{Rng, rng};

pub fn random_double() -> f64 {
    let mut rng = rng();
    rng.random_range(0.0..1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

use crate::onb::ONB;
use crate::random::{random_cosine_direction, random_unit_vector};
use crate::vec3::{Vec3, dot, unit_vector};
use std::f64::consts::PI;

pub trait Pdf {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct SpherePdf;

impl Default for SpherePdf {
    fn default() -> Self {
        Self::new()
    }
}

impl SpherePdf {
    pub fn new() -> Self {
        SpherePdf
    }
}

impl Pdf for SpherePdf {
    fn value(&self, _direction: Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        random_unit_vector()
    }
}

pub struct CosinePdf {
    uvw: ONB,
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        CosinePdf { uvw: ONB::new(w) }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine_theta = dot(&unit_vector(&direction), &self.uvw.w());
        (cosine_theta / PI).max(0.0)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(random_cosine_direction())
    }
}

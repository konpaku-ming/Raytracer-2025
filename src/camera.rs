use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    ctr: Point3,
    vul: Point3,
    v_u: Vec3,
    v_v: Vec3,
}

impl Camera {
    pub fn new(
        center: Point3,
        viewport_upper_left: Point3,
        viewport_u: Vec3,
        viewport_v: Vec3,
    ) -> Self {
        Self {
            ctr: center,
            vul: viewport_upper_left,
            v_u: viewport_u,
            v_v: viewport_v,
        }
    }

    pub fn center(&self) -> Point3 {
        self.ctr
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        Ray::new(self.ctr, self.vul + self.v_u * x + self.v_v * y - self.ctr)
    }
}

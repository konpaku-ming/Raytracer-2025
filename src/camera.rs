use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    cen: Point3,
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
            cen: center,
            vul: viewport_upper_left,
            v_u: viewport_u,
            v_v: viewport_v,
        }
    }
}

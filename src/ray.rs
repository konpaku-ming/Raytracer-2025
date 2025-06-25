use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::vec3::unit_vector;
use crate::vec3color::Color;

pub struct Ray {
    orig: Point3, //原点
    dir: Vec3,    //方向
}

impl Ray {
    //构造
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + (self.dir * t)
    }

    pub fn ray_color(&self) -> Color {
        let a = 0.5 * (unit_vector(self.direction()).y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}

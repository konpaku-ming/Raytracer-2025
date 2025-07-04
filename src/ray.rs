use crate::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    orig: Point3, //原点
    dir: Vec3,    //方向
    tm: f64,      //时间
}

impl Ray {
    //构造
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
            tm: 0.0,
        }
    }

    pub fn new_with_time(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            orig: origin,
            dir: direction,
            tm: time,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + (self.dir * t)
    }
}

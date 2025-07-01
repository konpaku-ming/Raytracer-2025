use crate::random::{random_double, random_in_unit_disk};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    ctr: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        center: Point3,
        pixel_delta_u: Vec3,
        pixel_delta_v: Vec3,
        pixel00_loc: Vec3,
        (defocus_angle, defocus_disk_u, defocus_disk_v): (f64, Vec3, Vec3),
    ) -> Self {
        Self {
            ctr: center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn center(&self) -> Point3 {
        self.ctr
    }

    pub fn sample_square(&mut self) -> Vec3 {
        //二维随机向量
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    pub fn get_ray(&mut self, i: u32, j: u32) -> Ray {
        //向指定位置发射一条光线
        let offset = self.sample_square(); //小范围内随机取样
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center()
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double(); //0-1之间随机时间
        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    pub fn defocus_disk_sample(&mut self) -> Point3 {
        let p = random_in_unit_disk();
        self.center() + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }
}

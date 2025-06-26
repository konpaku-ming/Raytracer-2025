use crate::random::random_double;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    ctr: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    samples_per_pixel: i32,
}

impl Camera {
    pub fn new(
        center: Point3,
        pixel_delta_u: Vec3,
        pixel_delta_v: Vec3,
        pixel00_loc: Vec3,
        samples_per_pixel: i32,
    ) -> Self {
        Self {
            ctr: center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel,
        }
    }

    pub fn center(&self) -> Point3 {
        self.ctr
    }

    pub fn sample_square(&self) -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_direction = pixel_sample - self.center();
        let ray_origin = self.center();
        Ray::new(ray_origin, ray_direction)
    }

    pub fn get_ray_samples(&self, i: u32, j: u32) -> Vec<Ray> {
        let mut  ray_samples= Vec::new();
        for _sample in 0..self.samples_per_pixel {
            let ray=self.get_ray(i,j);
            ray_samples.push(ray);
        }
        ray_samples
    }
}

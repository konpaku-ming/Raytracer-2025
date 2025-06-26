use crate::camera::Camera;
use crate::hit_checker::{HitRecord, Hittable, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::sketchpad::Sketchpad;
use crate::vec3::{Point3, Vec3, unit_vector};
use crate::vec3color::Color;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub struct RayTracer {
    sketchpad: Sketchpad,
    camera: Camera,
    //aspect_ratio: f64,
    width: u32,
    height: u32,
    //focal_length: f64,
    hittable_list: HittableList,
    pixel_samples_scale: f64,
    max_depth: i32,
}

impl RayTracer {
    pub fn new(
        aspect_ratio: f64,
        width: u32,
        //viewport_height: f64,
        focal_length: f64,
        hittable_list: HittableList,
        samples_per_pixel: i32,
        max_depth: i32,
        v_fov: f64,
    ) -> Self {
        let height = (width as f64 / aspect_ratio) as u32;
        let height = if height < 1 { 1 } else { height };
        let theta = degrees_to_radians(v_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (width as f64 / height as f64);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let center = Point3::new(0.0, 0.0, 0.0);
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_delta_u = viewport_u / width as f64;
        let pixel_delta_v = viewport_v / height as f64;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
        let camera = Camera::new(
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel,
        );
        let sketchpad = Sketchpad::new(width, aspect_ratio);

        Self {
            sketchpad,
            camera,
            //aspect_ratio,
            width,
            height,
            //focal_length,
            hittable_list,
            pixel_samples_scale,
            max_depth,
        }
    }

    pub fn ray_color(&self, ray: &Ray, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::default();
        if self
            .hittable_list
            .hit(ray, Interval::new(0.001, f64::INFINITY), &mut rec)
        {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            return if rec.mat.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                attenuation * self.ray_color(&scattered, depth - 1)
            } else {
                Color::new(0.0, 0.0, 0.0)
            };
        }
        let a = 0.5 * (unit_vector(ray.direction()).y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render(&mut self) {
        for j in 0..self.height {
            for i in 0..self.width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                let ray_samples = self.camera.get_ray_samples(i, j);
                for r in ray_samples {
                    pixel_color += self.ray_color(&r, self.max_depth);
                }
                self.sketchpad
                    .draw(i, j, pixel_color * self.pixel_samples_scale);
            }
        }
        self.sketchpad.save();
    }
}

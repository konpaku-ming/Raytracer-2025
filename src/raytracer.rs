use crate::camera::Camera;
use crate::ray::Ray;
use crate::sketchpad::Sketchpad;
use crate::vec3::{Point3, Vec3};

pub struct RayTracer {
    sketchpad: Sketchpad,
    camera: Camera,
    aspect_ratio: f64,
    width: u32,
    height: u32,
    focal_length: f64,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
}

impl RayTracer {
    pub fn new(aspect_ratio: f64, width: u32, viewport_height: f64, focal_length: f64) -> Self {
        let height = (width as f64 / aspect_ratio) as u32;
        let height = if height < 1 { 1 } else { height };
        let viewport_width = viewport_height * (width as f64 / height as f64);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let center = Point3::new(0.0, 0.0, 0.0);
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let camera = Camera::new(center, viewport_upper_left, viewport_u, viewport_v);
        let pixel_delta_u = viewport_u / width as f64;
        let pixel_delta_v = viewport_v / height as f64;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        let sketchpad = Sketchpad::new(width, aspect_ratio);

        Self {
            sketchpad,
            camera,
            aspect_ratio,
            width,
            height,
            focal_length,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
        }
    }

    pub fn render(&mut self) {
        for j in 0..self.height {
            for i in 0..self.width {
                let pixel_center = self.pixel00_loc
                    + ((i as f64) * self.pixel_delta_u)
                    + ((j as f64) * self.pixel_delta_v);
                let ray_direction = pixel_center - self.camera.center();
                let r = Ray::new(self.camera.center(), ray_direction);
                let pixel_color = r.ray_color();
                self.sketchpad.draw(i, j, pixel_color);
            }
        }
        self.sketchpad.save();
    }
}

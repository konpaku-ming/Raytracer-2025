use crate::camera::Camera;
use crate::hit_checker::{HitRecord, Hittable, HittableList, degrees_to_radians};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::sketchpad::Sketchpad;
use crate::vec3::{Point3, Vec3, cross, unit_vector};
use crate::vec3color::Color;
use indicatif::ProgressBar;

pub struct RayTracer {
    sketchpad: Sketchpad,
    camera: Camera,
    width: u32,
    height: u32,
    hittable_list: HittableList,
    pixel_samples_scale: f64,
    max_depth: i32,
    background: Color,
}

impl RayTracer {
    pub fn new(
        (aspect_ratio, width): (f64, u32),
        (look_from, look_at, vup, v_fov): (Point3, Point3, Vec3, f64),
        hittable_list: HittableList,
        samples_per_pixel: i32,
        max_depth: i32,
        (defocus_angle, focus_dist): (f64, f64),
        background: Color,
    ) -> Self {
        let height = (width as f64 / aspect_ratio) as u32;
        let height = if height < 1 { 1 } else { height };
        let theta = degrees_to_radians(v_fov);
        let h = (theta / 2.0).tan();

        let w = unit_vector(&(look_from - look_at));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (width as f64 / height as f64);
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        let center = look_from;
        let viewport_upper_left = center - focus_dist * w - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel_delta_u = viewport_u / width as f64;
        let pixel_delta_v = viewport_v / height as f64;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let defocus_radius = focus_dist * degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let camera = Camera::new(
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel,
            (defocus_angle, defocus_disk_u, defocus_disk_v),
        );
        let sketchpad = Sketchpad::new(width, aspect_ratio);

        Self {
            sketchpad,
            camera,
            width,
            height,
            hittable_list,
            pixel_samples_scale,
            max_depth,
            background,
        }
    }

    pub fn ray_color(&self, ray: &Ray, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::default();
        //没击中物体返回背景色，击中不散射返回发光颜色（不发光材料=发黑色光）
        if self
            .hittable_list
            .hit(ray, Interval::new(0.001, f64::INFINITY), &mut rec)
        {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            let color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.pos);
            return if rec.mat.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                attenuation * self.ray_color(&scattered, depth - 1) + color_from_emission
            } else {
                color_from_emission
            };
        }
        self.background
    }

    pub fn render(&mut self) {
        let progress = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new(self.height as u64)
        };

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
            progress.inc(1);
        }
        progress.finish();
        self.sketchpad.save();
    }
}

use crate::camera::Camera;
use crate::hit_checker::{HitRecord, Hittable, HittableList, degrees_to_radians};
use crate::interval::Interval;
use crate::material::ScatterRecord;
use crate::pdf::{HittablePdf, MixturePdf, Pdf};
use crate::ray::Ray;
use crate::sketchpad::Sketchpad;
use crate::vec3::{Point3, Vec3, cross, unit_vector};
use crate::vec3color::Color;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct RayTracer {
    sketchpad: Sketchpad,
    camera: Camera,
    width: u32,
    height: u32,
    hittable_list: HittableList,
    samples_per_pixel: i32,
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

        let defocus_radius = focus_dist * degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let sqrt_spp = (samples_per_pixel as f64).sqrt() as u32;
        let recip_sqrt_spp = 1.0 / sqrt_spp as f64;

        let camera = Camera::new(
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            (defocus_angle, defocus_disk_u, defocus_disk_v),
            recip_sqrt_spp,
        );
        let sketchpad = Sketchpad::new(width, aspect_ratio);

        Self {
            sketchpad,
            camera,
            width,
            height,
            hittable_list,
            samples_per_pixel,
            max_depth,
            background,
        }
    }

    pub fn ray_color(&self, ray: &Ray, depth: i32, lights: Arc<dyn Hittable>) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::default();
        //没击中物体返回背景色，击中不散射返回发光颜色（不发光材料=发黑色光）
        if self
            .hittable_list
            .hit(ray, Interval::new(0.001, f64::INFINITY), &mut rec)
        {
            let mut s_rec = ScatterRecord::default();
            let color_from_emission = rec.mat.emitted(ray, &rec, rec.u, rec.v, &rec.pos);
            if !rec.mat.scatter(ray, &rec, &mut s_rec) {
                return color_from_emission;
            }

            if s_rec.skip_pdf {
                return s_rec.attenuation
                    * self.ray_color(&s_rec.skip_pdf_ray, depth - 1, lights.clone());
            }

            let light_ptr = Arc::new(HittablePdf::new(lights.clone(), rec.pos));
            let mixed_pdf = MixturePdf::new(light_ptr, s_rec.pdf_ptr);

            let scattered = Ray::new_with_time(rec.pos, mixed_pdf.generate(), ray.time());
            let pdf_value = mixed_pdf.value(*scattered.direction());

            let scattering_pdf = rec.mat.scattering_pdf(ray, &rec, &scattered);

            let color_from_scatter = (s_rec.attenuation
                * scattering_pdf
                * self.ray_color(&scattered, depth - 1, lights.clone()))
                / pdf_value;
            return color_from_emission + color_from_scatter;
        }
        self.background
    }

    pub fn render(&mut self, lights: Arc<dyn Hittable>) {
        let width = self.width;
        let height = self.height;
        let samples_per_pixel = self.samples_per_pixel;
        let max_depth = self.max_depth;

        let sqrt_spp = (samples_per_pixel as f64).sqrt() as u32;
        let pixel_samples_scale = 1.0 / (sqrt_spp * sqrt_spp) as f64;

        let total_pixels = width * height;
        let progress = Arc::new(AtomicUsize::new(0));
        let mut pixels = vec![Color::new(0.0, 0.0, 0.0); total_pixels as usize];

        // 初始化进度条
        let pb = ProgressBar::new(total_pixels as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("渲染中: [{bar:40.cyan/blue}] {percent}% | {pos}/{len} 像素")
                .unwrap()
                .progress_chars("##-"),
        );

        pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, pixel)| {
                let x = index as u32 % width;
                let y = index as u32 / width;

                let mut color = Color::new(0.0, 0.0, 0.0);

                for s_j in 0..sqrt_spp {
                    for s_i in 0..sqrt_spp {
                        let r = self.camera.get_ray(x, y, s_i, s_j);
                        color += self.ray_color(&r, max_depth, lights.clone());
                    }
                }
                *pixel = color * pixel_samples_scale;

                progress.fetch_add(1, Ordering::Relaxed);
                pb.set_position(progress.load(Ordering::Relaxed) as u64);
            });

        pb.finish_with_message("渲染完成！");

        // 将结果写入 sketchpad
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                self.sketchpad.draw(x, y, pixels[idx as usize]);
            }
        }

        self.sketchpad.save();
    }
}

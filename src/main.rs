use raytracer::camera::Camera;
use raytracer::ray::Ray;
use raytracer::sketchpad::Sketchpad;
use raytracer::vec3::{Point3, Vec3};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let focal_length = 1.0;
    let camera_center = Point3::new(0.0, 0.0, 0.0);
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let camera = Camera::new(camera_center, viewport_upper_left, viewport_u, viewport_v);
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    let mut sketchpad = Sketchpad::new(image_width as u32, image_height as u32);
    for j in (0..image_height) {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + ((i as f64) * pixel_delta_u) + ((j as f64) * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = r.ray_color();
            sketchpad.draw(i as u32, j as u32, pixel_color);
        }
    }
    sketchpad.save();
}

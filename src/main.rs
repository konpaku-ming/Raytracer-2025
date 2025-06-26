use raytracer::hit_checker::HittableList;
use raytracer::hit_checker::Sphere;
use raytracer::raytracer::RayTracer;
use raytracer::vec3::Vec3;
use std::rc::Rc;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let viewport_height = 2.0;
    let focal_length = 1.0;
    let samples_per_pixel = 100;
    let mut hittable_list = HittableList::default();
    hittable_list.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    hittable_list.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let mut raytracer = RayTracer::new(
        aspect_ratio,
        image_width,
        viewport_height,
        focal_length,
        hittable_list,
        samples_per_pixel,
    );
    raytracer.render();
}

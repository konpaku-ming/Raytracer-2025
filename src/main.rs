use raytracer::hit_checker::HittableList;
use raytracer::hit_checker::Sphere;
use raytracer::material::Dielectric;
use raytracer::material::Lambertian;
use raytracer::material::Metal;
use raytracer::raytracer::RayTracer;
use raytracer::vec3::Point3;
use raytracer::vec3color::Color;
use std::rc::Rc;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let viewport_height = 2.0;
    let focal_length = 1.0;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let mut hittable_list = HittableList::default();
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.00 / 1.33));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    hittable_list.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    hittable_list.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    hittable_list.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    hittable_list.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    let mut raytracer = RayTracer::new(
        aspect_ratio,
        image_width,
        viewport_height,
        focal_length,
        hittable_list,
        samples_per_pixel,
        max_depth,
    );
    raytracer.render();
}

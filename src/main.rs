use raytracer::hit_checker::HittableList;
use raytracer::hit_checker::Sphere;
//use raytracer::material::Dielectric;
use raytracer::material::Lambertian;
//use raytracer::material::Metal;
use raytracer::raytracer::RayTracer;
use raytracer::vec3::Point3;
use raytracer::vec3color::Color;
use std::rc::Rc;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    //let viewport_height = 2.0;
    let focal_length = 1.0;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let v_fov = 90.0;
    let mut hittable_list = HittableList::default();
    let r = (std::f64::consts::PI / 4.0).cos();

    let material_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));
    hittable_list.add(Rc::new(Sphere::new(
        Point3::new(-r, 0.0, -1.0),
        r,
        material_left,
    )));
    hittable_list.add(Rc::new(Sphere::new(
        Point3::new(r, 0.0, -1.0),
        r,
        material_right,
    )));
    let mut raytracer = RayTracer::new(
        aspect_ratio,
        image_width,
        //viewport_height,
        focal_length,
        hittable_list,
        samples_per_pixel,
        max_depth,
        v_fov,
    );
    raytracer.render();
}

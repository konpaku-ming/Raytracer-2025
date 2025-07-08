use raytracer::bvh::BvhNode;
use raytracer::hit_checker::HittableList;
use raytracer::material::{Dielectric, DiffuseLight, DummyMaterial, Lambertian};
use raytracer::modeling::{Quad, RotateY, Sphere, Translate, make_box};
use raytracer::obj::create_model;
use raytracer::raytracer::RayTracer;
use raytracer::vec3::{Point3, Vec3};
use raytracer::vec3color::Color;
use std::sync::Arc;

fn main() {
    let op = 3;
    match op {
        1 => cornell_box(),
        2 => sphere(),
        _ => koishi(),
    }
}

fn cornell_box() {
    let aspect_ratio = 1.0;
    let image_width = 600;
    let samples_per_pixel = 1000;
    let max_depth = 50;
    let v_fov = 40.0;
    let look_from = Point3::new(278.0, 278.0, -800.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let background = Color::new(0.0, 0.0, 0.0);

    let mut world = HittableList::default();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));
    //let aluminum = Arc::new(Metal::new(Color::new(0.8, 0.85, 0.88), 0.0));
    let glass = Arc::new(Dielectric::new(1.5));

    let empty_material = Arc::new(DummyMaterial);
    let mut lights = HittableList::default();
    lights.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        empty_material.clone(),
    )));
    lights.add(Arc::new(Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        empty_material.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(550.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    world.add(Arc::new(Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        glass,
    )));

    let bvh = BvhNode::from_list(&mut world);
    let mut world = HittableList::default();
    world.add(Arc::new(bvh));

    let mut raytracer = RayTracer::new(
        (aspect_ratio, image_width),
        (look_from, look_at, vup, v_fov),
        world,
        samples_per_pixel,
        max_depth,
        (defocus_angle, focus_dist),
        background,
    );
    raytracer.render(Arc::new(lights));
}

fn koishi() {
    let aspect_ratio = 1.0;
    let image_width = 600;
    let samples_per_pixel = 10000;
    let max_depth = 50;
    let v_fov = 40.0;
    let look_from = Point3::new(0.0, 165.0, 100.0);
    let look_at = Point3::new(0.0, 120.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let background = Color::new(1.0, 1.0, 1.0);

    let mut world = HittableList::default();
    let mut lights = HittableList::default();
    let empty_material = Arc::new(DummyMaterial);

    lights.add(Arc::new(Quad::new(
        Point3::new(-100.0, 300.0, -100.0),
        Vec3::new(200.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 200.0),
        empty_material,
    )));

    create_model("assets/koishi.obj", "assets/koishi.mtl", &mut world);

    let mut raytracer = RayTracer::new(
        (aspect_ratio, image_width),
        (look_from, look_at, vup, v_fov),
        world,
        samples_per_pixel,
        max_depth,
        (defocus_angle, focus_dist),
        background,
    );
    raytracer.render(Arc::new(lights));
}

fn sphere() {
    let aspect_ratio = 1.0;
    let image_width = 600;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let v_fov = 40.0;
    let look_from = Point3::new(0.0, 165.0, 100.0);
    let look_at = Point3::new(0.0, 120.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let background = Color::new(1.0, 1.0, 1.0);

    let mut world = HittableList::default();
    let mut lights = HittableList::default();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 100.0, -20.0),
        10.0,
        red,
    )));

    /*
    let empty_material = Arc::new(DummyMaterial);

    lights.add(Arc::new(Sphere::new(
        Point3::new(0.0, 100.0, -20.0),
        10.0,
        empty_material,
    )));
     */

    let empty_material = Arc::new(DummyMaterial);

    lights.add(Arc::new(Quad::new(
        Point3::new(-100.0, 300.0, -100.0),
        Vec3::new(200.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 200.0),
        empty_material,
    )));

    let mut raytracer = RayTracer::new(
        (aspect_ratio, image_width),
        (look_from, look_at, vup, v_fov),
        world,
        samples_per_pixel,
        max_depth,
        (defocus_angle, focus_dist),
        background,
    );
    raytracer.render(Arc::new(lights));
}

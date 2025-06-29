use raytracer::bvh::BvhNode;
use raytracer::hit_checker::{HittableList, Sphere};
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::random::{random_double, random_double_range};
use raytracer::raytracer::RayTracer;
use raytracer::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use raytracer::vec3::{Point3, Vec3};
use raytracer::vec3color::Color;
use std::rc::Rc;

fn main() {
    let mode = 3;
    match mode {
        1 => checkered_spheres(),
        2 => earth(),
        3 => perlin_spheres(),
        _ => bouncing_speres(),
    }
}

fn perlin_spheres() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let v_fov = 20.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let mut world = HittableList::default();

    let perlin_texture = Rc::new(NoiseTexture::new(4.0));
    let perlin_surface = Rc::new(Lambertian::from_tex(perlin_texture));
    let globe = Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_surface.clone(),
    ));
    let ground = Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_surface.clone(),
    ));
    world.add(globe);
    world.add(ground);

    let mut raytracer = RayTracer::new(
        (aspect_ratio, image_width),
        (look_from, look_at, vup),
        world,
        samples_per_pixel,
        max_depth,
        v_fov,
        (defocus_angle, focus_dist),
    );
    raytracer.render();
}

fn earth() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let v_fov = 20.0;
    let look_from = Point3::new(0.0, 0.0, 12.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let mut world = HittableList::default();

    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Rc::new(Lambertian::from_tex(earth_texture));
    let globe = Rc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));
    world.add(globe);

    let mut raytracer = RayTracer::new(
        (aspect_ratio, image_width),
        (look_from, look_at, vup),
        world,
        samples_per_pixel,
        max_depth,
        v_fov,
        (defocus_angle, focus_dist),
    );
    raytracer.render();
}

fn checkered_spheres() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let v_fov = 20.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let mut hittable_list = HittableList::default();
    let checker = Rc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    hittable_list.add(Rc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Rc::new(Lambertian::from_tex(checker.clone())),
    )));
    hittable_list.add(Rc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Rc::new(Lambertian::from_tex(checker.clone())),
    )));
    let bvh = BvhNode::from_list(&mut hittable_list);
    let mut world = HittableList::default();
    world.add(Rc::new(bvh));
    let mut raytracer = RayTracer::new(
        (aspect_ratio, image_width),
        (look_from, look_at, vup),
        world,
        samples_per_pixel,
        max_depth,
        v_fov,
        (defocus_angle, focus_dist),
    );
    raytracer.render();
}

fn bouncing_speres() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let v_fov = 20.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;
    let mut hittable_list = HittableList::default();

    let checker = Rc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    hittable_list.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::from_tex(checker)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    hittable_list.add(Rc::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    hittable_list.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    hittable_list.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                };
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    hittable_list.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    hittable_list.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    hittable_list.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    let bvh = BvhNode::from_list(&mut hittable_list);
    let mut world = HittableList::default();
    world.add(Rc::new(bvh));
    let mut raytracer = RayTracer::new(
        (aspect_ratio, image_width),
        (look_from, look_at, vup),
        world,
        samples_per_pixel,
        max_depth,
        v_fov,
        (defocus_angle, focus_dist),
    );
    raytracer.render();
}

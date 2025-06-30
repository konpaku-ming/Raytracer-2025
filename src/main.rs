use raytracer::bvh::BvhNode;
use raytracer::hit_checker::{HittableList, Quad, RotateY, Sphere, Translate, make_box};
use raytracer::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use raytracer::random::{random_double, random_double_range};
use raytracer::raytracer::RayTracer;
use raytracer::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use raytracer::vec3::{Point3, Vec3};
use raytracer::vec3color::Color;
use std::rc::Rc;

fn main() {
    let mode = 6;
    match mode {
        1 => checkered_spheres(),
        2 => earth(),
        3 => perlin_spheres(),
        4 => quads(),
        5 => simple_light(),
        6 => cornell_box(),
        _ => bouncing_speres(),
    }
}

fn cornell_box() {
    let aspect_ratio = 1.0;
    let image_width = 600;
    let samples_per_pixel = 200;
    let max_depth = 50;
    let v_fov = 40.0;
    let look_from = Point3::new(278.0, 278.0, -800.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let background = Color::new(0.0, 0.0, 0.0);

    let mut world = HittableList::default();

    let red = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    world.add(Rc::new(Quad::new(
        Point3::new(550.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
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
    let box1 = Rc::new(RotateY::new(box1, 15.0));
    let box1 = Rc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = Rc::new(RotateY::new(box2, -18.0));
    let box2 = Rc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let bvh = BvhNode::from_list(&mut world);
    let mut world = HittableList::default();
    world.add(Rc::new(bvh));

    let mut raytracer = RayTracer::new(
        (aspect_ratio, image_width),
        (look_from, look_at, vup, v_fov),
        world,
        samples_per_pixel,
        max_depth,
        (defocus_angle, focus_dist),
        background,
    );
    raytracer.render();
}

fn simple_light() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let v_fov = 20.0;
    let look_from = Point3::new(26.0, 3.0, 6.0);
    let look_at = Point3::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let background = Color::new(0.0, 0.0, 0.0);

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

    let diffuse_light = Rc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    let quad_light = Rc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        diffuse_light.clone(),
    ));
    let sphere_light = Rc::new(Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, diffuse_light));

    world.add(globe);
    world.add(ground);
    world.add(quad_light);
    world.add(sphere_light);

    let mut raytracer = RayTracer::new(
        (aspect_ratio, image_width),
        (look_from, look_at, vup, v_fov),
        world,
        samples_per_pixel,
        max_depth,
        (defocus_angle, focus_dist),
        background,
    );
    raytracer.render();
}

fn quads() {
    let aspect_ratio = 1.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let v_fov = 80.0;
    let look_from = Point3::new(0.0, 0.0, 9.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let background = Color::new(0.70, 0.80, 1.00);

    let mut world = HittableList::default();

    let left_red = Rc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let back_green = Rc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Rc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Rc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    world.add(Rc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
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
    raytracer.render();
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
    let background = Color::new(0.70, 0.80, 1.00);

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
        (look_from, look_at, vup, v_fov),
        world,
        samples_per_pixel,
        max_depth,
        (defocus_angle, focus_dist),
        background,
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
    let background = Color::new(0.70, 0.80, 1.00);

    let mut world = HittableList::default();

    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Rc::new(Lambertian::from_tex(earth_texture));
    let globe = Rc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));
    world.add(globe);

    let mut raytracer = RayTracer::new(
        (aspect_ratio, image_width),
        (look_from, look_at, vup, v_fov),
        world,
        samples_per_pixel,
        max_depth,
        (defocus_angle, focus_dist),
        background,
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
    let background = Color::new(0.70, 0.80, 1.00);

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
        (look_from, look_at, vup, v_fov),
        world,
        samples_per_pixel,
        max_depth,
        (defocus_angle, focus_dist),
        background,
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
    let background = Color::new(0.70, 0.80, 1.00);

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
        (look_from, look_at, vup, v_fov),
        world,
        samples_per_pixel,
        max_depth,
        (defocus_angle, focus_dist),
        background,
    );
    raytracer.render();
}

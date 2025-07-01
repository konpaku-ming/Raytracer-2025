use raytracer::bvh::BvhNode;
use raytracer::hit_checker::HittableList;
use raytracer::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use raytracer::modeling::{ConstantMedium, Quad, RotateY, Sphere, Translate, make_box};
use raytracer::random::{random, random_double, random_double_range, random_range};
use raytracer::raytracer::RayTracer;
use raytracer::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use raytracer::vec3::{Point3, Vec3};
use raytracer::vec3color::Color;
use std::sync::Arc;

fn main() {
    let mode = 8;
    match mode {
        1 => checkered_spheres(),
        2 => earth(),
        3 => perlin_spheres(),
        4 => quads(),
        5 => simple_light(),
        6 => cornell_box(),
        7 => cornell_smoke(),
        8 => final_scene(),
        _ => bouncing_speres(),
    }
}

fn final_scene() {
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));
    let mut boxes1 = HittableList::default();
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(make_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world = HittableList::default();
    world.add(Arc::new(BvhNode::from_list(&mut boxes1)));

    let light = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));
    let boundary = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::from_color(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));
    let earth_mat = Arc::new(Lambertian::from_tex(Arc::new(ImageTexture::new(
        "earthmap.jpg",
    ))));
    world.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_mat,
    )));
    let perlin_texture = Arc::new(NoiseTexture::new(0.2));
    world.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::from_tex(perlin_texture)),
    )));

    let mut boxes2 = HittableList::default();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }
    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::from_list(&mut boxes2)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let aspect_ratio = 1.0;
    let image_width = 800;
    let samples_per_pixel = 10000;
    let max_depth = 40;
    let v_fov = 40.0;
    let look_from = Point3::new(478.0, 278.0, -600.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let background = Color::new(0.0, 0.0, 0.0);

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
    raytracer.render();
}

fn cornell_smoke() {
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

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));

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
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
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
    world.add(Arc::new(ConstantMedium::from_color(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));

    let box2 = make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(Arc::new(ConstantMedium::from_color(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
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
    raytracer.render();
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

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

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

    let box2 = make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

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

    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    let perlin_surface = Arc::new(Lambertian::from_tex(perlin_texture));
    let globe = Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_surface.clone(),
    ));
    let ground = Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_surface.clone(),
    ));

    let diffuse_light = Arc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    let quad_light = Arc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        diffuse_light.clone(),
    ));
    let sphere_light = Arc::new(Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, diffuse_light));

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

    let left_red = Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
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

    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    let perlin_surface = Arc::new(Lambertian::from_tex(perlin_texture));
    let globe = Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_surface.clone(),
    ));
    let ground = Arc::new(Sphere::new(
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

    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_tex(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));
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
    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    hittable_list.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::from_tex(checker.clone())),
    )));
    hittable_list.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::from_tex(checker.clone())),
    )));
    let bvh = BvhNode::from_list(&mut hittable_list);
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

    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    hittable_list.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_tex(checker)),
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
                    let albedo = random() * random();
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    hittable_list.add(Arc::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    hittable_list.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    hittable_list.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                };
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    hittable_list.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    hittable_list.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    hittable_list.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    let bvh = BvhNode::from_list(&mut hittable_list);
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
    raytracer.render();
}

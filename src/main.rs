use raytracer::bvh::BvhNode;
use raytracer::hit_checker::HittableList;
use raytracer::material::{Dielectric, DiffuseLight, DummyMaterial, Lambertian, Metal};
use raytracer::modeling::{Quad, RotateY, Sphere, Translate, make_box};
use raytracer::obj::create_model;
use raytracer::raytracer::RayTracer;
use raytracer::texture::{ImageTexture, MappedTexture};
use raytracer::vec3::{Point3, Vec3};
use raytracer::vec3color::Color;
use std::sync::Arc;

fn main() {
    let op = 4;
    match op {
        1 => cornell_box(),
        3 => ground(),
        4 => final_scene(),
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
    let samples_per_pixel = 2000;
    let max_depth = 50;
    let v_fov = 40.0;
    let look_from = Point3::new(0.0, 165.0, 150.0);
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

    create_model(
        "assets/koishi_alpha.obj",
        "assets/koishi_alpha.mtl",
        &mut world,
        183.0,
        Vec3::new(-10.0, 0.0, -200.0),
        1.0,
    );

    create_model(
        "assets/koishi.obj",
        "assets/koishi.mtl",
        &mut world,
        3.0,
        Vec3::new(10.0, 0.0, 0.0),
        1.0,
    );

    let brick = Arc::new(MappedTexture::new(
        "default_diffuse.png",
        Some("ground_normal_map.png"),
        None,
    ));

    let ground1 = Quad::new(
        Point3::new(0.0, 0.0, -100.0),
        Vec3::new(200.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 200.0),
        Arc::new(Lambertian::from_tex(brick.clone())),
    );

    let ground2 = Quad::new(
        Point3::new(0.0, 0.0, -300.0),
        Vec3::new(200.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 200.0),
        Arc::new(Lambertian::from_tex(brick.clone())),
    );

    let ground3 = Quad::new(
        Point3::new(-200.0, 0.0, -100.0),
        Vec3::new(200.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 200.0),
        Arc::new(Lambertian::from_tex(brick.clone())),
    );

    let ground4 = Quad::new(
        Point3::new(-200.0, 0.0, -300.0),
        Vec3::new(200.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 200.0),
        Arc::new(Lambertian::from_tex(brick.clone())),
    );

    let ground5 = Quad::new(
        Point3::new(0.0, 0.0, -500.0),
        Vec3::new(200.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 200.0),
        Arc::new(Lambertian::from_tex(brick.clone())),
    );

    let ground6 = Quad::new(
        Point3::new(-200.0, 0.0, -500.0),
        Vec3::new(200.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 200.0),
        Arc::new(Lambertian::from_tex(brick.clone())),
    );

    world.add(Arc::new(ground1));
    world.add(Arc::new(ground2));
    world.add(Arc::new(ground3));
    world.add(Arc::new(ground4));
    world.add(Arc::new(ground5));
    world.add(Arc::new(ground6));

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

fn ground() {
    let aspect_ratio = 1.0;
    let image_width = 600;
    let samples_per_pixel = 500;
    let max_depth = 50;
    let v_fov = 40.0;
    let look_from = Point3::new(0.0, 100.0, 100.0);
    let look_at = Point3::new(0.0, 10.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let background = Color::new(1.0, 1.0, 1.0);

    let mut world = HittableList::default();
    let mut lights = HittableList::default();

    let brick = Lambertian::from_tex(Arc::new(MappedTexture::new(
        "default_diffuse.png",
        Some("ground_normal_map.png"),
        None,
    )));

    let ground = Quad::new(
        Point3::new(-100.0, 0.0, -100.0),
        Vec3::new(200.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 200.0),
        Arc::new(brick),
    );

    world.add(Arc::new(ground));

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

fn final_scene() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1600;
    let samples_per_pixel = 200;
    let max_depth = 50;
    let v_fov = 40.0;
    let look_from = Point3::new(160.0, 325.0, 420.0);
    let look_at = Point3::new(20.0, 120.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let background = Color::new(1.0, 1.0, 1.0);

    let mut world = HittableList::default();
    let mut lights = HittableList::default();

    let star = Arc::new(DiffuseLight::from_texture(Arc::new(ImageTexture::new(
        "star.png",
    ))));
    let background_star = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 1000.0, star));

    world.add(background_star);

    create_model(
        "assets/word.obj",
        "assets/word.mtl",
        &mut world,
        25.0,
        Vec3::new(50.0, 25.0, 120.0),
        1.0,
    );

    create_model(
        "assets/koishi_alpha.obj",
        "assets/koishi_alpha.mtl",
        &mut world,
        -150.0,
        Vec3::new(-120.0, 0.0, 150.0),
        1.6,
    );

    create_model(
        "assets/koishi.obj",
        "assets/koishi.mtl",
        &mut world,
        -55.0,
        Vec3::new(80.0, 0.0, 10.0),
        1.6,
    );

    let brick = Arc::new(MappedTexture::new(
        "default_diffuse.png",
        Some("ground_normal_map.png"),
        None,
    ));

    let magma = Arc::new(MappedTexture::new(
        "magma.jpg",
        Some("magma_normal_map.png"),
        None,
    ));

    let ground1 = make_box(
        Point3::new(-350.0, -20.0, -100.0),
        Point3::new(0.0, 0.0, 200.0),
        Arc::new(Lambertian::from_tex(brick.clone())),
    );

    let ground2 = make_box(
        Point3::new(-700.0, -20.0, -100.0),
        Point3::new(-350.0, 0.0, 200.0),
        Arc::new(Lambertian::from_tex(brick.clone())),
    );

    let ground3 = make_box(
        Point3::new(0.0, -20.0, -100.0),
        Point3::new(350.0, 0.0, 200.0),
        Arc::new(DiffuseLight::from_texture(magma.clone())),
    );

    let ground4 = make_box(
        Point3::new(350.0, -20.0, -100.0),
        Point3::new(700.0, 0.0, 200.0),
        Arc::new(DiffuseLight::from_texture(magma.clone())),
    );

    world.add(ground1);
    world.add(ground2);
    world.add(ground3);
    world.add(ground4);

    let metal = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let mirror = make_box(
        Point3::new(-2.0, 0.0, -100.0),
        Point3::new(2.0, 520.0, 200.0),
        metal.clone(),
    );

    world.add(mirror);

    let empty_material = Arc::new(DummyMaterial);

    let jupiter = Arc::new(DiffuseLight::from_texture(Arc::new(ImageTexture::new(
        "jupiter.jpg",
    ))));

    let earth = Arc::new(DiffuseLight::from_texture(Arc::new(ImageTexture::new(
        "earth.jpg",
    ))));

    //let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Sphere::new(
        Point3::new(200.0, 250.0, -50.0),
        50.0,
        jupiter.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-300.0, 200.0, -200.0),
        35.0,
        earth.clone(),
    )));

    lights.add(Arc::new(Sphere::new(
        Point3::new(200.0, 250.0, -50.0),
        50.0,
        empty_material.clone(),
    )));

    lights.add(Arc::new(Sphere::new(
        Point3::new(-250.0, 200.0, -200.0),
        35.0,
        empty_material.clone(),
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

use raytracer::bvh::BvhNode;
use raytracer::hit_checker::HittableList;
use raytracer::material::{DiffuseLight, DummyMaterial, Lambertian, Metal};
use raytracer::modeling::{ConstantMedium, Quad, Sphere, Translate, make_box};
use raytracer::obj::create_model;
use raytracer::random::random_double_range;
use raytracer::raytracer::RayTracer;
use raytracer::texture::{ImageTexture, MappedTexture};
use raytracer::vec3::{Point3, Vec3};
use raytracer::vec3color::Color;
use std::sync::Arc;

fn main() {
    final_scene();
}

fn final_scene() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1600;
    let samples_per_pixel = 100;
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

    let empty_material = Arc::new(DummyMaterial);

    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(60.0, 300.0, 500.0),
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(0.0, 10.0, 0.0),
        light.clone(),
    )));

    lights.add(Arc::new(Quad::new(
        Point3::new(60.0, 300.0, 500.0),
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(0.0, 10.0, 0.0),
        empty_material.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(-150.0, 0.0, 200.0),
        Vec3::new(15.0, 0.0, 0.0),
        Vec3::new(0.0, 20.0, 0.0),
        light.clone(),
    )));

    lights.add(Arc::new(Quad::new(
        Point3::new(-150.0, 0.0, 200.0),
        Vec3::new(15.0, 0.0, 0.0),
        Vec3::new(0.0, 20.0, 0.0),
        empty_material.clone(),
    )));

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

    create_model(
        "assets/morisa.obj",
        "assets/morisa.mtl",
        &mut world,
        0.0,
        Vec3::new(287.0, 0.0, -155.0),
        90.0,
    );

    create_model(
        "assets/utsuho.obj",
        "assets/utsuho.mtl",
        &mut world,
        10.0,
        Vec3::new(282.0, 80.0, -140.0),
        6.0,
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

    let jupiter = Arc::new(DiffuseLight::from_texture(Arc::new(ImageTexture::new(
        "jupiter.jpg",
    ))));

    let earth = Arc::new(DiffuseLight::from_texture(Arc::new(ImageTexture::new(
        "earth.jpg",
    ))));

    world.add(Arc::new(Sphere::new(
        Point3::new(230.0, 270.0, -120.0),
        70.0,
        jupiter.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-300.0, 200.0, -200.0),
        35.0,
        earth.clone(),
    )));

    let flame_box1 = make_box(
        Point3::new(50.0, 260.0, 30.0),
        Point3::new(0.0, 0.0, 0.0),
        Arc::new(DummyMaterial),
    );

    let flame_box2 = make_box(
        Point3::new(40.0, 200.0, 20.0),
        Point3::new(0.0, 0.0, 0.0),
        Arc::new(DummyMaterial),
    );

    let flame_box1 = Arc::new(Translate::new(flame_box1, Vec3::new(260.0, 55.0, -160.0)));

    let flame_box2 = Arc::new(Translate::new(flame_box2, Vec3::new(265.0, 45.0, -155.0)));

    let flame1 = Arc::new(ConstantMedium::from_color(
        flame_box1,
        0.02,
        Color::new(0.96, 0.26, 0.0),
    ));

    let flame2 = Arc::new(ConstantMedium::from_color(
        flame_box2,
        0.03,
        Color::new(0.93, 0.86, 0.0),
    ));

    world.add(flame1);
    world.add(flame2);

    let red = Arc::new(DiffuseLight::new(Color::new(3.0, 0.3, 0.1)));

    for _ in 0..90 {
        let x = random_double_range(-350.0, 350.0);
        let y = random_double_range(150.0, 350.0);
        let z = random_double_range(-300.0, 200.0);

        let cen = Point3::new(x, y, z);

        let dx = random_double_range(4.5, 4.6);
        let dy = random_double_range(10.0, 15.0);
        let dz = random_double_range(-0.1, 0.1);

        let offset = Vec3::new(dx, dy, dz);

        let r = random_double_range(0.5, 0.7);

        let firefly = Arc::new(Sphere::new_moving(cen, cen + offset, r, red.clone()));

        world.add(firefly);
    }

    let mut the_world = HittableList::default();
    the_world.add(Arc::new(BvhNode::from_list(&mut world)));
    
    

    let mut raytracer = RayTracer::new(
        (aspect_ratio, image_width),
        (look_from, look_at, vup, v_fov),
        the_world,
        samples_per_pixel,
        max_depth,
        (defocus_angle, focus_dist),
        background,
    );
    raytracer.render(Arc::new(lights));
}

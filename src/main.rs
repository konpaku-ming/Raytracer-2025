use raytracer::bvh::BvhNode;
use raytracer::hit_checker::HittableList;
use raytracer::material::{DiffuseLight, DummyMaterial, Lambertian, Metal};
use raytracer::modeling::{Quad, Sphere, make_box};
use raytracer::obj::create_model;
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
    let samples_per_pixel = 10000;
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
        Vec3::new(285.0, 0.0, -150.0),
        90.0,
    );

    create_model(
        "assets/utsuho.obj",
        "assets/utsuho.mtl",
        &mut world,
        10.0,
        Vec3::new(250.0, 140.0, 10.0),
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
    /*

    let light1 = Arc::new(Quad::new(
        Point3::new(-150.0, 500.0, -150.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 300.0),
        light.clone(),
    ));

    world.add(light1);

    lights.add(Arc::new(Quad::new(
        Point3::new(-150.0, 500.0, -150.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 300.0),
        empty_material.clone(),
    )));


     */
    /*

    let light2 = Arc::new(Quad::new(
        Point3::new(450.0, 100.0, 200.0),
        Vec3::new(-50.0, 0.0, -100.0),
        Vec3::new(0.0, -100.0, -50.0),
        light.clone(),
    ));

    world.add(light2);

    lights.add(Arc::new(Quad::new(
        Point3::new(450.0, 100.0, 200.0),
        Vec3::new(-50.0, 0.0, -100.0),
        Vec3::new(0.0, -100.0, -50.0),
        empty_material.clone(),
    )));

     */

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

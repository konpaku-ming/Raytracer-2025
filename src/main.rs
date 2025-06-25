use raytracer::raytracer::RayTracer;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let viewport_height = 2.0;
    let focal_length = 1.0;
    let mut raytracer = RayTracer::new(aspect_ratio, image_width, viewport_height, focal_length);
    raytracer.render();
}

use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::dot;

pub fn hit_sphere(center: &Point3, r: f64, ray: &Ray) -> f64 {
    let oc = *center - *ray.origin();
    let a = ray.direction().length_squared();
    let h = dot(ray.direction(), &oc);
    let c = oc.length_squared() - r * r;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

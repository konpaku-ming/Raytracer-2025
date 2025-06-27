use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::material::DummyMaterial;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::vec3::dot;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub pos: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Rc<dyn Material>,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            pos: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: true,
            mat: Rc::new(DummyMaterial {}),
        }
    }
}

impl HitRecord {
    pub fn new(ray: &Ray, t: f64, outward_normal: Vec3) -> Self {
        let pos = ray.at(t);
        let front_face = dot(ray.direction(), &outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            t,
            pos,
            normal,
            front_face,
            mat: Rc::new(DummyMaterial {}),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval, hit_record: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> Aabb;
}

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Rc<dyn Material>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(static_center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        Self {
            center: Ray::new(static_center, Vec3::new(0.0, 0.0, 0.0)),
            radius: radius.max(0.0),
            mat,
            bbox: Aabb::from_points(static_center - r_vec, static_center + r_vec),
        }
    }

    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Rc<dyn Material>,
    ) -> Self {
        let center = Ray::new(center1, center2 - center1);
        let r_vec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::from_points(center.at(0.0) - r_vec, center.at(0.0) + r_vec);
        let box2 = Aabb::from_points(center.at(1.0) - r_vec, center.at(1.0) + r_vec);
        Self {
            center,
            radius: radius.max(0.0),
            mat,
            bbox: Aabb::from_box(box1, box2),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval, hit_record: &mut HitRecord) -> bool {
        let current_center = self.center.at(ray.time());
        let oc = current_center - *ray.origin();
        let a = ray.direction().length_squared();
        let h = dot(ray.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;
        if !interval.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !interval.surrounds(root) {
                return false;
            }
        }
        hit_record.t = root;
        hit_record.pos = ray.at(hit_record.t);
        hit_record.normal = (hit_record.pos - current_center) / self.radius;
        let outward_normal = (hit_record.pos - current_center) / self.radius;
        hit_record.front_face = dot(ray.direction(), &outward_normal) < 0.0;
        hit_record.normal = if hit_record.front_face {
            outward_normal
        } else {
            -outward_normal
        };
        hit_record.mat = self.mat.clone();
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object.clone());
        self.bbox = Aabb::from_box(self.bbox, object.bounding_box());
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, interval: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = interval.max;

        for object in &self.objects {
            if object.hit(
                r,
                Interval::new(interval.min, closest_so_far),
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

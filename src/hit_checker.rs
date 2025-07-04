use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::material::{DummyMaterial, Material};
use crate::random::random_int_range;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, dot};
use std::sync::Arc;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

#[derive(Clone)]
pub struct HitRecord {
    pub pos: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
    pub u: f64,
    pub v: f64,
    pub tangent: Vec3,
    pub bitangent: Vec3,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            pos: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: true,
            mat: Arc::new(DummyMaterial {}),
            u: 0.0,
            v: 0.0,
            tangent: Vec3::new(1.0, 0.0, 0.0),
            bitangent: Vec3::new(0.0, 1.0, 0.0),
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
            mat: Arc::new(DummyMaterial {}),
            u: 0.0,
            v: 0.0,
            tangent: Vec3::new(1.0, 0.0, 0.0),
            bitangent: Vec3::new(0.0, 1.0, 0.0),
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: Interval, hit_record: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> Aabb;

    fn pdf_value(&self, _origin: Point3, _direction: Vec3) -> f64 {
        0.0
    }
    fn random(&self, _origin: Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = Aabb::from_box(self.bbox, object.bounding_box());
        self.objects.push(object);
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

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        self.objects
            .iter()
            .map(|object| weight * object.pdf_value(origin, direction))
            .sum()
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let int_size = self.objects.len() as i32;
        let index = random_int_range(0, int_size - 1) as usize;
        self.objects[index].random(origin)
    }
}

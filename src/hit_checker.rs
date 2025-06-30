use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::material::{DummyMaterial, Material};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, cross, dot, unit_vector};
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub pos: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Rc<dyn Material>,
    pub u: f64,
    pub v: f64,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            pos: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: true,
            mat: Rc::new(DummyMaterial {}),
            u: 0.0,
            v: 0.0,
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
            u: 0.0,
            v: 0.0,
        }
    }

    fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
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

    pub fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + std::f64::consts::PI;
        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;
        (u, v)
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
        hit_record.set_face_normal(ray, outward_normal);
        (hit_record.u, hit_record.v) = Sphere::get_sphere_uv(&outward_normal);
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

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Rc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Self {
        let n = cross(&u, &v);
        let normal = unit_vector(&n);
        let d = dot(&normal, &q);
        let w = n / dot(&n, &n);
        let mut quad = Quad {
            q,
            u,
            v,
            mat,
            bbox: Aabb::default(),
            normal,
            d,
            w,
        };
        quad.set_bounding_box();
        quad
    }

    fn set_bounding_box(&mut self) {
        let bbox_diagonal1 = Aabb::from_points(self.q, self.q + self.u + self.v);
        let bbox_diagonal2 = Aabb::from_points(self.q + self.u, self.q + self.v);
        self.bbox = Aabb::from_box(bbox_diagonal1, bbox_diagonal2);
    }

    fn is_interior(&self, a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }
        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = dot(&self.normal, r.direction());
        if denom.abs() < 1e-8 {
            return false;
        }
        let t = (self.d - dot(&self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }
        let intersection = r.at(t);
        let planar_hit_vector = intersection - self.q;
        let alpha = dot(&self.w, &cross(&planar_hit_vector, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &planar_hit_vector));
        if !self.is_interior(alpha, beta, rec) {
            return false;
        }
        rec.t = t;
        rec.pos = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, self.normal);
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub fn make_box(a: Point3, b: Point3, mat: Rc<dyn Material>) -> Rc<HittableList> {
    let mut sides = HittableList::default();

    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    sides.add(Rc::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx,
        dy,
        mat.clone(), //front
    )));
    sides.add(Rc::new(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -dz,
        dy,
        mat.clone(), //right
    )));
    sides.add(Rc::new(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -dx,
        dy,
        mat.clone(), //back
    )));
    sides.add(Rc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz,
        dy,
        mat.clone(), //left
    )));
    sides.add(Rc::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -dz,
        mat.clone(), //top
    )));
    sides.add(Rc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        mat.clone(), //bottom
    )));
    Rc::new(sides)
}

use crate::aabb::Aabb;
use crate::hit_checker::{HitRecord, Hittable, HittableList, degrees_to_radians};
use crate::interval::Interval;
use crate::material::{Isotropic, Material};
use crate::onb::ONB;
use crate::random::{random_double, random_double_range, random_to_sphere};
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::vec3::{Point3, Vec3, cross, dot, unit_vector};
use crate::vec3color::Color;
use std::f64::consts::PI;
use std::sync::Arc;

pub struct Sphere<M: Material> {
    center: Ray,
    radius: f64,
    mat: Arc<M>,
    bbox: Aabb,
}

impl<M: Material> Sphere<M> {
    pub fn new(static_center: Point3, radius: f64, mat: Arc<M>) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        Self {
            center: Ray::new(static_center, Vec3::new(0.0, 0.0, 0.0)),
            radius: radius.max(0.0),
            mat,
            bbox: Aabb::from_points(static_center - r_vec, static_center + r_vec),
        }
    }

    pub fn new_moving(center1: Point3, center2: Point3, radius: f64, mat: Arc<M>) -> Self {
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

pub fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
    let theta = (-p.y()).acos();
    let phi = (-p.z()).atan2(p.x()) + PI;
    let u = phi / (2.0 * PI);
    let v = theta / PI;
    (u, v)
}

impl<M: Material + Send + Sync + 'static> Hittable for Sphere<M> {
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
        let outward_normal = (hit_record.pos - current_center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);
        (hit_record.u, hit_record.v) = get_sphere_uv(&outward_normal);
        hit_record.normal = outward_normal;
        hit_record.mat = self.mat.clone();
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
        let ray = Ray::new(origin, direction);
        let mut rec = HitRecord::default();
        if !self.hit(&ray, Interval::new(0.001, f64::INFINITY), &mut rec) {
            return 0.0;
        }

        let dist_squared = (self.center.at(0.0) - origin).length_squared();
        let cos_theta_max = (1.0 - self.radius * self.radius / dist_squared).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let direction = self.center.at(0.0) - origin;
        let distance_squared = direction.length_squared();
        let uvw = ONB::new(&direction);
        uvw.transform(random_to_sphere(self.radius, distance_squared))
    }
}

pub struct Quad<M: Material> {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<M>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    area: f64,
}

impl<M: Material> Quad<M> {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<M>) -> Self {
        let n = cross(&u, &v);
        let normal = unit_vector(&n);
        let d = dot(&normal, &q);
        let w = n / dot(&n, &n);
        let mut quad = Self {
            q,
            u,
            v,
            w,
            mat,
            bbox: Aabb::default(),
            normal,
            d,
            area: n.length(),
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

impl<M: Material + Send + Sync + 'static> Hittable for Quad<M> {
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

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if !self.hit(
            &Ray::new(origin, direction),
            Interval::new(0.001, f64::INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }
        let distance_squared = rec.t * rec.t * direction.length_squared();
        let cosine = (dot(&direction, &rec.normal) / direction.length()).abs();
        distance_squared / (cosine * self.area)
    }

    fn random(&self, origin: Vec3) -> Vec3 {
        let p = self.q + (random_double() * self.u) + (random_double() * self.v);
        p - origin
    }
}

pub fn make_box<M: Material + Send + Sync + 'static>(
    a: Point3,
    b: Point3,
    mat: Arc<M>,
) -> Arc<HittableList> {
    let mut sides = HittableList::default();

    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx,
        dy,
        mat.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -dz,
        dy,
        mat.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -dx,
        dy,
        mat.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz,
        dy,
        mat.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -dz,
        mat.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        mat.clone(),
    ))); // bottom

    Arc::new(sides)
}

pub struct Translate<H: Hittable + Send + Sync + 'static> {
    object: Arc<H>,
    offset: Vec3,
    bbox: Aabb,
}

impl<H: Hittable + Send + Sync + 'static> Translate<H> {
    pub fn new(object: Arc<H>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl<H: Hittable + Send + Sync + 'static> Hittable for Translate<H> {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new_with_time(*r.origin() - self.offset, *r.direction(), r.time());
        if !self.object.hit(&moved_r, ray_t, rec) {
            return false;
        }
        rec.pos += self.offset;
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct RotateY<H: Hittable + Send + Sync + 'static> {
    object: Arc<H>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl<H: Hittable + Send + Sync + 'static> RotateY<H> {
    pub fn new(object: Arc<H>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();
        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = if i == 0 { bbox.x.min } else { bbox.x.max };
                    let y = if j == 0 { bbox.y.min } else { bbox.y.max };
                    let z = if k == 0 { bbox.z.min } else { bbox.z.max };

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;
                    let tester = Point3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        let bbox = Aabb::from_points(min, max);
        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl<H: Hittable + Send + Sync + 'static> Hittable for RotateY<H> {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let origin = Point3::new(
            (self.cos_theta * r.origin().x()) - (self.sin_theta * r.origin().z()),
            r.origin().y(),
            (self.sin_theta * r.origin().x()) + (self.cos_theta * r.origin().z()),
        );
        let direction = Vec3::new(
            (self.cos_theta * r.direction().x()) - (self.sin_theta * r.direction().z()),
            r.direction().y(),
            (self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().z()),
        );

        let rotated_r = Ray::new_with_time(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        rec.pos = Point3::new(
            (self.cos_theta * rec.pos.x()) + (self.sin_theta * rec.pos.z()),
            rec.pos.y(),
            (-self.sin_theta * rec.pos.x()) + (self.cos_theta * rec.pos.z()),
        );

        rec.normal = Vec3::new(
            (self.cos_theta * rec.normal.x()) + (self.sin_theta * rec.normal.z()),
            rec.normal.y(),
            (-self.sin_theta * rec.normal.x()) + (self.cos_theta * rec.normal.z()),
        );
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct ConstantMedium<H: Hittable + Send + Sync + 'static, M: Material + Send + Sync + 'static>
{
    boundary: Arc<H>,
    neg_inv_density: f64,
    phase_function: Arc<M>,
}

impl<H: Hittable + Send + Sync + 'static, T: Texture + Send + Sync + 'static>
    ConstantMedium<H, Isotropic<T>>
{
    pub fn from_texture(boundary: Arc<H>, density: f64, tex: Arc<T>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_from_texture(tex)),
        }
    }
}

impl<H> ConstantMedium<H, Isotropic<SolidColor>>
where
    H: Hittable + Send + Sync + 'static,
{
    pub fn from_color(boundary: Arc<H>, density: f64, color: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_from_color(color)),
        }
    }
}

impl<H: Hittable + Send + Sync + 'static, M: Material + Send + Sync + 'static> Hittable
    for ConstantMedium<H, M>
{
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self.boundary.hit(r, Interval::UNIVERSE, &mut rec1) {
            return false;
        }

        if !self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, f64::INFINITY), &mut rec2)
        {
            return false;
        }

        rec1.t = rec1.t.max(ray_t.min);
        rec2.t = rec2.t.min(ray_t.max);

        if rec1.t >= rec2.t {
            return false;
        }

        rec1.t = rec1.t.max(0.0);
        let ray_length = r.direction().length();
        let distance_inside = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double_range(0.0, 1.0).ln();

        if hit_distance > distance_inside {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.pos = r.at(rec.t);
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = self.phase_function.clone();
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}

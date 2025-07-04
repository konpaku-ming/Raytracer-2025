use crate::aabb::Aabb;
use crate::hit_checker::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::uv::UV;
use crate::vec3::{Point3, Vec3, cross, dot, unit_vector};
use std::sync::Arc;

pub struct Triangle {
    v0: Point3,
    v1: Point3,
    v2: Point3,
    pub uv0: UV,
    pub uv1: UV,
    pub uv2: UV,
    normal: Vec3,
    tangent: Vec3,
    bitangent: Vec3,
    mat: Arc<dyn Material + Send + Sync>,
    bbox: Aabb,
}

impl Triangle {
    pub fn new(
        v0: Point3,
        v1: Point3,
        v2: Point3,
        uv0: UV,
        uv1: UV,
        uv2: UV,
        mat: Arc<dyn Material>,
    ) -> Self {
        let normal = unit_vector(&cross(&(v1 - v0), &(v2 - v0)));

        let delta_pos1 = v1 - v0;
        let delta_pos2 = v2 - v0;
        let delta_uv1 = uv1 - uv0;
        let delta_uv2 = uv2 - uv0;

        let r = 1.0 / (delta_uv1.u() * delta_uv2.v() - delta_uv1.v() * delta_uv2.u());
        let tangent = unit_vector(&(r * (delta_pos1 * delta_uv2.v() - delta_pos2 * delta_uv1.v())));
        let bitangent =
            unit_vector(&(r * (delta_pos2 * delta_uv1.u() - delta_pos1 * delta_uv2.u())));

        let mut triangle = Triangle {
            v0,
            v1,
            v2,
            uv0,
            uv1,
            uv2,
            normal,
            tangent,
            bitangent,
            mat,
            bbox: Aabb::default(),
        };
        triangle.set_bounding_box();
        triangle
    }

    fn set_bounding_box(&mut self) {
        let min_x = self.v0.x().min(self.v1.x()).min(self.v2.x());
        let min_y = self.v0.y().min(self.v1.y()).min(self.v2.y());
        let min_z = self.v0.z().min(self.v1.z()).min(self.v2.z());
        let max_x = self.v0.x().max(self.v1.x()).max(self.v2.x());
        let max_y = self.v0.y().max(self.v1.y()).max(self.v2.y());
        let max_z = self.v0.z().max(self.v1.z()).max(self.v2.z());
        self.bbox = Aabb::new(
            Interval::new(min_x, max_x),
            Interval::new(min_y, max_y),
            Interval::new(min_z, max_z),
        );
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;

        let denom = dot(&self.normal, ray.direction());
        if denom.abs() < 1e-8 {
            return false;
        }

        let d = dot(&self.normal, &self.v0);

        let t = (d - dot(&self.normal, ray.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }
        let intersection = ray.at(t);
        let planar_hit_vector = intersection - self.v0;

        let d00 = dot(&edge1, &edge1);
        let d01 = dot(&edge1, &edge2);
        let d11 = dot(&edge2, &edge2);
        let d20 = dot(&planar_hit_vector, &edge1);
        let d21 = dot(&planar_hit_vector, &edge2);

        let a = d00 * d11 - d01 * d01;
        let u = (d11 * d20 - d01 * d21) / a;
        let v = (d00 * d21 - d01 * d20) / a;
        let w = 1.0 - v - u;

        if u < 0.0 || v < 0.0 || w < 0.0 {
            return false;
        }

        rec.u = u;
        rec.v = v;
        rec.t = t;
        rec.pos = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(ray, self.normal);
        rec.tangent = self.tangent;
        rec.bitangent = self.bitangent;
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

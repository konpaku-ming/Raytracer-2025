use crate::aabb::Aabb;
use crate::bvh::BvhNode;
use crate::hit_checker::{HitRecord, Hittable, HittableList};
use crate::interval::Interval;
use crate::material::{Lambertian, Material};
use crate::modeling::Translate;
use crate::mtl::{make_mapped_texture_from_mtl, parse_mtl_file};
use crate::ray::Ray;
use crate::uv::UV;
use crate::vec3::{Point3, Vec3, cross, dot, unit_vector};
use crate::vec3color::Color;
use std::collections::HashMap;
use std::sync::Arc;
use tobj::{LoadOptions, load_obj};

pub struct Triangle {
    p0: Point3, // 顶点0
    p1: Point3,
    p2: Point3,
    pub uv0: UV, //顶点0的uv坐标
    pub uv1: UV,
    pub uv2: UV,
    e1: Vec3, //边01
    e2: Vec3, //边02
    normal: Vec3,
    tangent: Vec3,
    bitangent: Vec3,
    mat: Arc<dyn Material + Send + Sync>,
    bbox: Aabb,
}

impl Triangle {
    pub fn new(
        p0: Point3,
        p1: Point3,
        p2: Point3,
        uv0: UV,
        uv1: UV,
        uv2: UV,
        mat: Arc<dyn Material>,
    ) -> Self {
        let e1 = p1 - p0;
        let e2 = p2 - p0;
        let n = cross(&e1, &e2);
        let normal = unit_vector(&n);

        let duv1 = uv1 - uv0;
        let duv2 = uv2 - uv0;

        let r = 1.0 / (duv1.u() * duv2.v() - duv2.u() * duv1.v());
        let tangent = unit_vector(&((e1 * duv2.v() - e2 * duv1.v()) * r));
        let bitangent = unit_vector(&cross(&normal, &tangent));
        let mut triangle = Self {
            p0,
            p1,
            p2,
            uv0,
            uv1,
            uv2,
            e1,
            e2,
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
        let min_x = self.p0.x().min(self.p1.x()).min(self.p2.x());
        let min_y = self.p0.y().min(self.p1.y()).min(self.p2.y());
        let min_z = self.p0.z().min(self.p1.z()).min(self.p2.z());
        let max_x = self.p0.x().max(self.p1.x()).max(self.p2.x());
        let max_y = self.p0.y().max(self.p1.y()).max(self.p2.y());
        let max_z = self.p0.z().max(self.p1.z()).max(self.p2.z());
        self.bbox = Aabb::new(
            Interval::new(min_x, max_x),
            Interval::new(min_y, max_y),
            Interval::new(min_z, max_z),
        );
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let h = cross(ray.direction(), &self.e2);
        let a = dot(&self.e1, &h);
        if a.abs() < 1e-8 {
            return false;
        }

        let f = 1.0 / a;
        let s = *ray.origin() - self.p0;
        let u = f * dot(&s, &h);
        if u < 0.0 || u > 1.0 {
            return false;
        }

        let q = cross(&s, &self.e1);
        let v = f * dot(ray.direction(), &q);
        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = f * dot(&self.e2, &q);
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = ray.at(t);

        let uv = self.uv0 * (1.0 - u - v) + self.uv1 * u + self.uv2 * v;

        rec.u = uv.u();
        rec.v = uv.v();
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

pub fn obj_loader(obj_path: &str, mtl_path: &str) -> Vec<Triangle> {
    // 加载 .obj 模型与材质列表
    let (models, materials) = load_obj(
        obj_path,
        &LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )
    .expect("OBJ 加载失败");

    let mut material_map = HashMap::new();

    let mtl_path = format!("{}", mtl_path);

    if let Ok(parsed) = std::panic::catch_unwind(|| parse_mtl_file(&mtl_path)) {
        for (_, info) in parsed {
            let tex = Arc::new(make_mapped_texture_from_mtl(&info));
            let mat = Arc::new(Lambertian::from_tex(tex));
            material_map.insert(info.name.clone(), mat);
        }
    }

    let mut triangles = Vec::new();

    let materials = materials.unwrap();
    for model in models {
        let mesh = &model.mesh;
        let positions = &mesh.positions;
        let texcoords = &mesh.texcoords;
        let indices = &mesh.indices;

        let material = if let Some(mat_id) = mesh.material_id {
            let tobj_material = materials.get(mat_id);
            if let Some(mat) = tobj_material {
                if let Some(resolved) = material_map.get(&mat.name) {
                    resolved.clone()
                } else {
                    Arc::new(Lambertian::new(Color::new(0.9, 0.5, 0.1)))
                }
            } else {
                Arc::new(Lambertian::new(Color::new(0.9, 0.5, 0.1)))
            }
        } else {
            Arc::new(Lambertian::new(Color::new(0.9, 0.5, 0.1)))
        };

        for i in (0..indices.len()).step_by(3) {
            let get_vertex = |j| {
                let idx = indices[i + j] as usize;
                Point3::new(
                    positions[3 * idx] as f64,
                    positions[3 * idx + 1] as f64,
                    positions[3 * idx + 2] as f64,
                )
            };

            let get_uv = |j| {
                let idx = indices[i + j] as usize;
                if texcoords.len() >= 2 * idx + 2 {
                    UV::new(texcoords[2 * idx] as f64, texcoords[2 * idx + 1] as f64)
                } else {
                    UV::default()
                }
            };

            let v0 = get_vertex(0);
            let v1 = get_vertex(1);
            let v2 = get_vertex(2);

            let uv0 = get_uv(0);
            let uv1 = get_uv(1);
            let uv2 = get_uv(2);

            triangles.push(Triangle::new(v0, v1, v2, uv0, uv1, uv2, material.clone()));
        }
    }

    triangles
}

pub fn create_model(
    obj_path: &str,
    mtl_path: &str,
    world: &mut HittableList,
    lights: &mut HittableList,
) {
    let vec = obj_loader(obj_path, mtl_path);
    let mut model = HittableList::default();
    for triangle in vec {
        model.add(Arc::new(triangle));
    }
    let bvh = BvhNode::from_list(&mut model);
    let model_translate = Arc::new(Translate::new(Arc::new(bvh), Vec3::new(0.0, 0.0, 0.0)));
    world.add(model_translate.clone());
    lights.add(model_translate);
}

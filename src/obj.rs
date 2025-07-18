use crate::aabb::Aabb;
use crate::bvh::BvhNode;
use crate::hit_checker::{HitRecord, Hittable, HittableList};
use crate::interval::Interval;
use crate::material::Lambertian;
use crate::modeling::{RotateY, Translate};
use crate::mtl::{make_mapped_texture_from_mtl, parse_mtl_file};
use crate::random::random_double;
use crate::ray::Ray;
use crate::texture::{MappedTexture, Texture};
use crate::uv::UV;
use crate::vec3::{Point3, Vec3, cross, dot, unit_vector};
use std::collections::HashMap;
use std::sync::Arc;
use tobj::{LoadOptions, load_obj};

pub fn interpolate_normals(n0: Vec3, n1: Vec3, n2: Vec3, u: f64, v: f64) -> Vec3 {
    let w = 1.0 - u - v;
    let interpolated = n0 * w + n1 * u + n2 * v;
    unit_vector(&interpolated)
}

pub struct Triangle<T: Texture> {
    p0: Point3, //顶点0
    p1: Point3,
    p2: Point3,
    pub uv0: UV, //顶点0的uv坐标
    pub uv1: UV,
    pub uv2: UV,
    n0: Vec3, //顶点0的点法线
    n1: Vec3,
    n2: Vec3,
    e1: Vec3, //边01
    e2: Vec3, //边02
    tangent: Vec3,
    mat: Arc<Lambertian<T>>,
    bbox: Aabb,
}

impl<T: Texture> Triangle<T> {
    pub fn new(
        (p0, p1, p2): (Point3, Point3, Point3),
        (uv0, uv1, uv2): (UV, UV, UV),
        (n0, n1, n2): (Vec3, Vec3, Vec3),
        mat: Arc<Lambertian<T>>,
    ) -> Self {
        let e1 = p1 - p0;
        let e2 = p2 - p0;
        let delta_uv1 = uv1 - uv0;
        let delta_uv2 = uv2 - uv0;

        let r = 1.0 / (delta_uv1.u() * delta_uv2.v() - delta_uv1.v() * delta_uv2.u());
        let tangent = (e1 * delta_uv2.v() - e2 * delta_uv1.v()) * r;

        let mut triangle = Self {
            p0,
            p1,
            p2,
            uv0,
            uv1,
            uv2,
            n0,
            n1,
            n2,
            e1,
            e2,
            tangent,
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

impl<T: Texture + 'static> Hittable for Triangle<T> {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // ... 你的 hit 实现逻辑保持不变，只确保 self.mat 是 Arc<Lambertian<T>>
        let h = cross(ray.direction(), &self.e2);
        let a = dot(&self.e1, &h);
        if a.abs() < 1e-8 {
            return false;
        }

        let f = 1.0 / a;
        let s = *ray.origin() - self.p0;
        let u = f * dot(&s, &h);
        if !(0.0..=1.0).contains(&u) {
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
        let normal = interpolate_normals(self.n0, self.n1, self.n2, u, v);
        let true_normal = unit_vector(&cross(&self.e1, &self.e2));
        let tangent = unit_vector(&(self.tangent - normal * dot(&self.tangent, &normal)));
        let bitangent = cross(&normal, &tangent);

        let alpha = self.mat.alpha(uv.u(), uv.v());
        if random_double() <= alpha {
            return false;
        }

        rec.u = uv.u();
        rec.v = uv.v();
        rec.t = t;
        rec.pos = intersection;
        rec.mat = self.mat.clone();
        rec.tangent = tangent;
        rec.bitangent = bitangent;
        rec.normal = true_normal;
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
        distance_squared / (cosine * cross(&self.e1, &self.e2).length() / 2.0)
    }

    fn random(&self, origin: Vec3) -> Vec3 {
        let p = self.p0 + (random_double() * self.e1) + (random_double() * self.e2);
        p - origin
    }
}

pub fn obj_loader(obj_path: &str, mtl_path: &str, rate: f64) -> Vec<Triangle<MappedTexture>> {
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

    let mtl_path = mtl_path.to_string();

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
        let normals = &mesh.normals;

        let material = if let Some(mat_id) = mesh.material_id {
            let tobj_material = materials.get(mat_id);
            if let Some(mat) = tobj_material {
                if let Some(resolved) = material_map.get(&mat.name) {
                    resolved.clone()
                } else {
                    Arc::new(Lambertian::from_tex(Arc::new(MappedTexture::new(
                        "default_diffuse.png",
                        None,
                        None,
                    ))))
                }
            } else {
                Arc::new(Lambertian::from_tex(Arc::new(MappedTexture::new(
                    "default_diffuse.png",
                    None,
                    None,
                ))))
            }
        } else {
            Arc::new(Lambertian::from_tex(Arc::new(MappedTexture::new(
                "default_diffuse.png",
                None,
                None,
            ))))
        };

        for i in (0..indices.len()).step_by(3) {
            let get_vertex = |j| {
                let idx = indices[i + j] as usize;
                Point3::new(
                    rate * positions[3 * idx] as f64,
                    rate * positions[3 * idx + 1] as f64,
                    rate * positions[3 * idx + 2] as f64,
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

            let get_normal = |j| {
                let idx = indices[i + j] as usize;
                Vec3::new(
                    rate * normals[3 * idx] as f64,
                    rate * normals[3 * idx + 1] as f64,
                    rate * normals[3 * idx + 2] as f64,
                )
            };

            let v0 = get_vertex(0);
            let v1 = get_vertex(1);
            let v2 = get_vertex(2);

            let uv0 = get_uv(0);
            let uv1 = get_uv(1);
            let uv2 = get_uv(2);

            let n0 = get_normal(0);
            let n1 = get_normal(1);
            let n2 = get_normal(2);

            triangles.push(Triangle::new(
                (v0, v1, v2),
                (uv0, uv1, uv2),
                (n0, n1, n2),
                material.clone(),
            ));
        }
    }

    triangles
}

pub fn create_model(
    obj_path: &str,
    mtl_path: &str,
    world: &mut HittableList,
    angle: f64,
    offset: Vec3,
    rate: f64, //放大倍率
) {
    let vec = obj_loader(obj_path, mtl_path, rate);
    let mut model = HittableList::default();
    for triangle in vec {
        model.add(Arc::new(triangle));
    }
    let bvh = BvhNode::from_list(&mut model);
    let model_rotate_y = Arc::new(RotateY::new(Arc::new(bvh), angle));
    let model_translate = Arc::new(Translate::new(model_rotate_y, offset));
    world.add(model_translate.clone());
}

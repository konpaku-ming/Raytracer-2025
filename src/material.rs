use crate::hit_checker::HitRecord;
use crate::pdf::{CosinePdf, DummyPdf, Pdf, SpherePdf};
use crate::random::{random_double, random_unit_vector};
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::vec3::{Point3, Vec3, dot, unit_vector};
use crate::vec3color::Color;
use std::f64::consts::PI;
use std::sync::Arc;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Arc<dyn Pdf>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}

impl Default for ScatterRecord {
    fn default() -> Self {
        Self {
            attenuation: Color::default(),
            pdf_ptr: Arc::new(DummyPdf),
            skip_pdf: true,
            skip_pdf_ray: Ray::default(),
        }
    }
}

pub trait Material: Send + Sync {
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
    fn scatter(&self, _r_in: &Ray, _rec: &mut HitRecord, _s_rec: &mut ScatterRecord) -> bool {
        false
    }

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

#[derive(Default)]
pub struct DummyMaterial;

impl Material for DummyMaterial {}

pub struct Lambertian<T: Texture> {
    pub tex: Arc<T>,
}

impl<T: Texture> Lambertian<T> {
    pub fn from_tex(tex: Arc<T>) -> Self {
        Self { tex }
    }

    pub fn alpha(&self, u: f64, v: f64) -> f64 {
        match self.tex.alpha(u, v) {
            Some(a) => a.sqrt(),
            _ => 0.0,
        }
    }
}

impl Lambertian<SolidColor> {
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = dot(&rec.normal, &unit_vector(scattered.direction()));
        if cos_theta < 0.0 { 0.0 } else { cos_theta / PI }
    }

    fn scatter(&self, _r_in: &Ray, rec: &mut HitRecord, s_rec: &mut ScatterRecord) -> bool {
        rec.normal = match self
            .tex
            .normal(rec.u, rec.v, rec.normal, rec.tangent, rec.bitangent)
        {
            Some(n) => n,
            _ => rec.normal,
        };

        s_rec.attenuation = self.tex.value(rec.u, rec.v, &rec.pos);
        s_rec.pdf_ptr = Arc::new(CosinePdf::new(&rec.normal));
        s_rec.skip_pdf = false;
        true
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, s_rec: &mut ScatterRecord) -> bool {
        let mut reflected = Vec3::reflect(r_in.direction(), &rec.normal);
        reflected = unit_vector(&reflected) + (self.fuzz * random_unit_vector());

        s_rec.attenuation = self.albedo;
        s_rec.pdf_ptr = Arc::new(DummyPdf);
        s_rec.skip_pdf = true;
        s_rec.skip_pdf_ray = Ray::new_with_time(rec.pos, reflected, r_in.time());
        true
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, s_rec: &mut ScatterRecord) -> bool {
        s_rec.attenuation = Color::new(1.0, 1.0, 1.0);
        s_rec.pdf_ptr = Arc::new(DummyPdf);
        s_rec.skip_pdf = true;

        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0; //全反射

        let direction = if cannot_refract || reflectance(cos_theta, ri) > random_double() {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, ri)
        };
        s_rec.skip_pdf_ray = Ray::new_with_time(rec.pos, direction, r_in.time());
        true
    }
}

pub struct DiffuseLight<T: Texture> {
    tex: Arc<T>,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn from_texture(tex: Arc<T>) -> Self {
        Self { tex }
    }
}

impl DiffuseLight<SolidColor> {
    pub fn new(emit: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        self.tex.value(u, v, p)
    }
}

pub struct Isotropic<T: Texture + ?Sized> {
    tex: Arc<T>,
}

impl<T: Texture> Isotropic<T> {
    pub fn new_from_texture(tex: Arc<T>) -> Self {
        Self { tex }
    }
}

impl Isotropic<SolidColor> {
    pub fn new_from_color(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn scatter(&self, _r_in: &Ray, rec: &mut HitRecord, s_rec: &mut ScatterRecord) -> bool {
        s_rec.attenuation = self.tex.value(rec.u, rec.v, &rec.pos);
        s_rec.pdf_ptr = Arc::new(SpherePdf::new());
        s_rec.skip_pdf = false;
        true
    }
}

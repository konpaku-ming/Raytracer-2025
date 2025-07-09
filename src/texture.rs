use crate::interval::Interval;
use crate::my_image::MyImage;
use crate::perlin::Perlin;
use crate::vec3::{Point3, Vec3};
use crate::vec3color::Color;
use std::sync::Arc;
pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;

    fn normal(
        &self,
        _u: f64,
        _v: f64,
        _geom_normal: Vec3,
        _tangent: Vec3,
        _bitangent: Vec3,
    ) -> Option<Vec3> {
        None
    }

    fn alpha(&self, _u: f64, _v: f64) -> Option<f64> {
        None
    }
}

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self {
            albedo: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, c1: Color, c2: Color) -> Self {
        let even = Arc::new(SolidColor::new(c1));
        let odd = Arc::new(SolidColor::new(c2));
        CheckerTexture::new(scale, even, odd)
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x = (self.inv_scale * p[0]).floor() as i32;
        let y = (self.inv_scale * p[1]).floor() as i32;
        let z = (self.inv_scale * p[2]).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: MyImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            image: MyImage::new(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.image.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = u * self.image.width() as f64;
        let j = v * self.image.height() as f64;

        let i1 = i as usize;
        let j1 = j as usize;
        let i2 = (i + 1.0).min(self.image.width() as f64) as usize;
        let j2 = (j + 1.0).min(self.image.height() as f64) as usize;

        let sx = i - i1 as f64;
        let sy = j - j1 as f64;

        let pixel1 = self.image.pixel_rgba(i1, j1);
        let pixel2 = self.image.pixel_rgba(i2, j1);
        let pixel3 = self.image.pixel_rgba(i1, j2);
        let pixel4 = self.image.pixel_rgba(i2, j2);

        let pixel: [u8; 4] = [
            ((1.0 - sy) * (1.0 - sx) * (pixel1[0] as f64)
                + (1.0 - sy) * sx * (pixel2[0] as f64)
                + (1.0 - sx) * sy * (pixel3[0] as f64)
                + sx * sy * (pixel4[0] as f64)) as u8,
            ((1.0 - sy) * (1.0 - sx) * (pixel1[1] as f64)
                + (1.0 - sy) * sx * (pixel2[1] as f64)
                + (1.0 - sx) * sy * (pixel3[1] as f64)
                + sx * sy * (pixel4[1] as f64)) as u8,
            ((1.0 - sy) * (1.0 - sx) * (pixel1[2] as f64)
                + (1.0 - sy) * sx * (pixel2[2] as f64)
                + (1.0 - sx) * sy * (pixel3[2] as f64)
                + sx * sy * (pixel4[2] as f64)) as u8,
            ((1.0 - sy) * (1.0 - sx) * (pixel1[3] as f64)
                + (1.0 - sy) * sx * (pixel2[3] as f64)
                + (1.0 - sx) * sy * (pixel3[3] as f64)
                + sx * sy * (pixel4[3] as f64)) as u8,
        ];

        let color_scale = 1.0 / 255.0;

        Color::new(
            (color_scale * pixel[0] as f64) * (color_scale * pixel[0] as f64),
            (color_scale * pixel[1] as f64) * (color_scale * pixel[1] as f64),
            (color_scale * pixel[2] as f64) * (color_scale * pixel[2] as f64),
        )
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        let noise_float =
            0.5 * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turbulence(p, 7)).sin());
        Color::new(noise_float, noise_float, noise_float)
    }
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self::new(1.0)
    }
}

pub struct Mat3 {
    cols: [Vec3; 3], //三个列向量：分别是 tangent, bitangent, normal
}

impl Mat3 {
    pub fn from_cols(t: Vec3, b: Vec3, n: Vec3) -> Self {
        Mat3 { cols: [t, b, n] }
    }

    pub fn mul_vec3(&self, v: Vec3) -> Vec3 {
        self.cols[0] * v.x() + self.cols[1] * v.y() + self.cols[2] * v.z()
    }
}

pub struct MappedTexture {
    color_map: MyImage,          //颜色纹理
    normal_map: Option<MyImage>, //法线贴图
    alpha_map: Option<MyImage>,  //Alpha 通道
}

impl MappedTexture {
    pub fn new(color_path: &str, normal_path: Option<&str>, alpha_path: Option<&str>) -> Self {
        let color_map = MyImage::new(color_path);
        let normal_map = normal_path.map(MyImage::new);
        let alpha_map = alpha_path.map(MyImage::new);

        Self {
            color_map,
            normal_map,
            alpha_map,
        }
    }
}

impl Texture for MappedTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.color_map.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = u * self.color_map.width() as f64;
        let j = v * self.color_map.height() as f64;

        let i1 = i as usize;
        let j1 = j as usize;
        let i2 = (i + 1.0).min(self.color_map.width() as f64) as usize;
        let j2 = (j + 1.0).min(self.color_map.height() as f64) as usize;

        let sx = i - i1 as f64;
        let sy = j - j1 as f64;

        let pixel1 = self.color_map.pixel_rgba(i1, j1);
        let pixel2 = self.color_map.pixel_rgba(i2, j1);
        let pixel3 = self.color_map.pixel_rgba(i1, j2);
        let pixel4 = self.color_map.pixel_rgba(i2, j2);

        let mut pixel: [u8; 4] = [0; 4];
        pixel[0] = (((1.0 - sx) * (pixel1[0] as f64) + sx * (pixel2[0] as f64)) * (1.0 - sy)
            + ((1.0 - sx) * (pixel3[0] as f64) + sx * (pixel4[0] as f64)) * sy)
            as u8;
        pixel[1] = (((1.0 - sx) * (pixel1[1] as f64) + sx * (pixel2[1] as f64)) * (1.0 - sy)
            + ((1.0 - sx) * (pixel3[1] as f64) + sx * (pixel4[1] as f64)) * sy)
            as u8;
        pixel[2] = (((1.0 - sx) * (pixel1[2] as f64) + sx * (pixel2[2] as f64)) * (1.0 - sy)
            + ((1.0 - sx) * (pixel3[2] as f64) + sx * (pixel4[2] as f64)) * sy)
            as u8;
        pixel[3] = (((1.0 - sx) * (pixel1[3] as f64) + sx * (pixel2[3] as f64)) * (1.0 - sy)
            + ((1.0 - sx) * (pixel3[3] as f64) + sx * (pixel4[3] as f64)) * sy)
            as u8;

        let color_scale = 1.0 / 255.0;

        let color = Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        );
        color * color
    }

    fn normal(&self, u: f64, v: f64, normal: Vec3, tangent: Vec3, bitangent: Vec3) -> Option<Vec3> {
        let map = self.normal_map.as_ref()?;
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * map.width() as f64) as usize;
        let j = (v * map.height() as f64) as usize;
        let pixel = map.pixel_rgba(i, j);

        let nx = 2.0 * (pixel[0] as f64 / 255.0) - 1.0;
        let ny = 2.0 * (pixel[1] as f64 / 255.0) - 1.0;
        let nz = 2.0 * (pixel[2] as f64 / 255.0) - 1.0;

        let tangent_space_normal = Vec3::new(nx, ny, nz);
        let tbn = Mat3::from_cols(tangent, bitangent, normal);
        Some(tbn.mul_vec3(tangent_space_normal))
    }

    fn alpha(&self, u: f64, v: f64) -> Option<f64> {
        let map = self.alpha_map.as_ref()?;
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * map.width() as f64) as usize;
        let j = (v * map.height() as f64) as usize;
        let pixel = map.pixel_rgba(i, j);

        Some(pixel[0] as f64 / 255.0)
    }
}

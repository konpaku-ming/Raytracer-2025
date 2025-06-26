use crate::vec3::Vec3;
pub type Color = Vec3;
use crate::interval::Interval;

impl Color {
    pub fn r_byte(&self) -> u8 {
        let r = self.x();
        (255.999 * r) as u8
    }

    pub fn g_byte(&self) -> u8 {
        let g = self.y();
        (255.999 * g) as u8
    }

    pub fn b_byte(&self) -> u8 {
        let b = self.z();
        (255.999 * b) as u8
    }

    pub fn write_color(pixel_color: &Color) -> [u8; 3] {
        let intensity = Interval::new(0.000, 0.999);
        let r = pixel_color.x();
        let g = pixel_color.y();
        let b = pixel_color.z();
        let r_byte = (256.0 * intensity.clamp(r)) as u8;
        let g_byte = (256.0 * intensity.clamp(g)) as u8;
        let b_byte = (256.0 * intensity.clamp(b)) as u8;
        [r_byte, g_byte, b_byte]
    }
}

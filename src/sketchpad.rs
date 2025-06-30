use crate::vec3color::Color;
use console::style;
use image::{ImageBuffer, RgbImage};

pub struct Sketchpad {
    image: RgbImage,
}

impl Sketchpad {
    pub fn new(width: u32, aspect_ratio: f64) -> Self {
        let height = (width as f64 / aspect_ratio) as u32;
        let height = if height < 1 { 1 } else { height };
        Self {
            image: ImageBuffer::new(width, height),
        }
    }

    pub fn draw(&mut self, x: u32, y: u32, color: Color) {
        let pixel = self.image.get_pixel_mut(x, y);
        *pixel = image::Rgb(Color::write_color(&color));
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }

    pub fn save(&self) {
        let path = std::path::Path::new("output/book2/image5.png");
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
        println!(
            "Output image as \"{}\"",
            style(path.to_str().unwrap()).yellow()
        );
        self.image
            .save(path)
            .expect("Cannot save the image to the file");
    }
}

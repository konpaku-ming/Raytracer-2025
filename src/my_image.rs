use image::ImageReader;
use std::env;
use std::path::PathBuf;

pub struct MyImage {
    image_width: usize,
    image_height: usize,
    bytes_per_pixel: usize,
    bytes_per_scanline: usize,
    b_data: Option<Vec<u8>>,
}

impl MyImage {
    pub fn new(image_filename: &str) -> Self {
        let mut image = MyImage {
            image_width: 0,
            image_height: 0,
            bytes_per_pixel: 4,
            bytes_per_scanline: 0,
            b_data: None,
        };

        let filename = image_filename.to_string();
        let image_dir = env::var("MY_IMAGES").ok();

        let mut paths = vec![];

        if let Some(dir) = image_dir {
            paths.push(PathBuf::from(dir).join(&filename));
        }
        paths.push(PathBuf::from(&filename));
        paths.push(PathBuf::from("images").join(&filename));

        let mut current = PathBuf::from("..");
        for _ in 0..6 {
            paths.push(current.join("images").join(&filename));
            current = current.join("..");
        }

        for path in paths {
            if image.load(&path) {
                return image;
            }
        }

        eprintln!("ERROR: Could not load image file '{}'.", filename);
        image
    }

    fn load(&mut self, path: &std::path::Path) -> bool {
        let reader = match ImageReader::open(path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("无法打开图像文件 {:?}: {}", path, e);
                return false;
            }
        };

        let img = match reader.decode() {
            Ok(img) => img.to_rgba8(),
            Err(e) => {
                eprintln!("图像解码失败 {:?}: {}", path, e);
                return false;
            }
        };

        self.image_width = img.width() as usize;
        self.image_height = img.height() as usize;
        self.bytes_per_scanline = self.image_width * self.bytes_per_pixel;
        self.b_data = Some(img.into_raw());
        true
    }

    pub fn width(&self) -> usize {
        if self.b_data.is_none() {
            0
        } else {
            self.image_width
        }
    }

    pub fn height(&self) -> usize {
        if self.b_data.is_none() {
            0
        } else {
            self.image_height
        }
    }

    pub fn pixel_rgba(&self, x: usize, y: usize) -> [u8; 4] {
        static MAGENTA: [u8; 4] = [255, 0, 255, 255];
        if let Some(data) = &self.b_data {
            let x = Self::clamp(x, 0, self.image_width);
            let y = Self::clamp(y, 0, self.image_height);
            let index = y * self.bytes_per_scanline + x * self.bytes_per_pixel;
            if index + 3 < data.len() {
                return [
                    data[index],
                    data[index + 1],
                    data[index + 2],
                    data[index + 3],
                ];
            }
        }
        MAGENTA
    }

    fn clamp(x: usize, low: usize, high: usize) -> usize {
        if x < low {
            low
        } else if x < high {
            x
        } else {
            high - 1
        }
    }
}

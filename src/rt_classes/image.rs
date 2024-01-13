use std::fmt::format;
use std::path::Path;
use std::fs::{self, File};
use std::io::BufWriter;
use std::sync::Arc;

use crate::config::Config;
use crate::rt_classes::color::Color;

pub struct Image {
    data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            data: vec![0; (3 * width * height) as usize],
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, index: usize, color: &Color) {
        self.data[index] = (color.r * 255f64) as u8;
        self.data[index + 1] = (color.g * 255f64) as u8;
        self.data[index + 2] = (color.b * 255f64) as u8;
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn save(&self, conf: &Config) {

        let path = Path::new(&conf.path);
        fs::create_dir_all(Path::new(&conf.path)).expect("Could not find or create path to store images at.");
        let file = File::create(path.join("img.png")).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let source_chromaticities = png::SourceChromaticities::new(
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&self.data).unwrap();
    }
}

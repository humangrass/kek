use crossterm::style::{Color, Stylize, SetBackgroundColor, ResetColor};
use image::{DynamicImage, GenericImageView, imageops::FilterType};

pub struct Converter {
    pub image_path: String,
    pub width: u32,
    pub should_display: bool,
    pub with_background: bool,
}

impl Converter {
    pub fn image_to_ascii(&self) -> String {
        let img = image::open(&self.image_path).expect("Can't open image.");
        let resized = self.resize_image(&img);

        let mut lines = String::new();
        for y in (0..resized.height()).step_by(3) {
            let line = self.process_line(&resized, y);
            if self.should_display {
                println!("{}", line);
            }
            lines.push_str(&line);
            lines.push('\n');
        }

        lines
    }

    fn resize_image(&self, img: &DynamicImage) -> DynamicImage {
        let (img_width, img_height) = img.dimensions();
        let scale = self.width as f32 / img_width as f32;
        let new_height = (img_height as f32 * scale) as u32;
        img.resize(self.width, new_height, FilterType::Nearest)
    }

    fn process_line(&self, img: &DynamicImage, y: u32) -> String {
        let mut line = String::new();
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let colored_char = self.pixel_to_char(pixel);
            line.push_str(&colored_char);
        }
        line
    }

    fn pixel_to_char(&self, pixel: image::Rgba<u8>) -> String {
        if self.with_background {
            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
            format!("{} {}", SetBackgroundColor(Color::Rgb { r, g, b }), ResetColor)
        } else {
            let brightness = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3;
            let ascii_chars = ["@", "#", "S", "%", "?", "*", "+", ";", ":", ",", "."];
            let idx = brightness * (ascii_chars.len() - 1) as u32 / 255;
            let char = ascii_chars[idx as usize];
            format!("{char}").with(Color::Rgb {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
            }).to_string()
        }
    }
}

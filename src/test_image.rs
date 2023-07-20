use std::path::Path;

use image::{DynamicImage, GenericImageView};
use screenshots::Screen;

pub struct Image {
    img: DynamicImage,
}

impl Image {
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        Self {
            img: image::open(path).unwrap(),
        }
    }

    pub fn from_screen(x: i32, y: i32, width: usize, height: usize) -> Self {
        let screen_shot = capture_screen(x, y, width, height);
        Self { img: screen_shot }
    }

    pub fn get_dims(&self) -> (u32, u32) {
        self.img.dimensions()
    }

    pub fn get_buffer(&self) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        self.img.clone().into_rgb8()
    }
}

pub fn test_image() -> Result<(), Box<dyn std::error::Error>> {
    // Read an image from file
    let img = image::open("./写轮眼_128x128.png")?;

    // Convert the image to grayscale
    let gray_img = img.grayscale();

    // Write the grayscale image to file
    gray_img.save("myimage_gray.png")?;

    Ok(())
}

pub fn capture_screen(x: i32, y: i32, width: usize, height: usize) -> DynamicImage {
    let _start = std::time::Instant::now();
    let screen = Screen::all().unwrap()[0];

    let image = screen
        .capture_area(x, y, width as u32, height as u32)
        .unwrap();

    let buffer = image.to_png().unwrap();
    let cursor = std::io::Cursor::new(buffer);
    let img = image::load(cursor, image::ImageFormat::Png).unwrap();

    img
}

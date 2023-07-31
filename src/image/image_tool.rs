use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use screenshots::{Compression, Screen};
use std::path::Path;

pub struct Image {
    img: DynamicImage,
}

impl Image {
    pub fn get_image(&self) -> &DynamicImage {
        &self.img
    }

    pub fn gray(&mut self) {
        self.img = self.img.grayscale();
    }

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

    pub fn get_buffer_u8(&self) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        self.img.clone().into_rgb8()
    }

    pub fn get_buffer_f32(&self) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
        self.img.clone().into_rgb32f()
    }

    pub fn get_buffer_u32(&self) -> ImageBuffer<Rgb<u32>, Vec<u32>> {
        todo!()
        // TODO:
        // let rgb_image =self.img.into_rgba8();
    //     let buffer: Vec<u32> = image
    // .into_raw()
    // .chunks(4)
    // .map(|chunk| {
    //     let r = chunk[0] as u32;
    //     let g = chunk[1] as u32;
    //     let b = chunk[2] as u32;
    //     let a = chunk[3] as u32;
    //     (a << 24) | (r << 16) | (g << 8) | b
    // })
    // .collect();
    //     result
    }

    pub fn get_buffer_u32_2(&self) -> ImageBuffer<Rgb<u32>, Vec<u32>> {
        let rgb_image = self.get_buffer_f32();
        let (width, height) = self.img.dimensions();
        let mut result = ImageBuffer::new(width, height);

        for (x, y, pixel) in rgb_image.enumerate_pixels() {
            if pixel[0] != 0.0 || pixel[1] != 0. || pixel[2] != 0. {
                println!("pixel: {:?}", pixel);
            }
            let r = (pixel[0] * 255.0) as u32;
            let g = (pixel[1] * 255.0) as u32;
            let b = (pixel[2] * 255.0) as u32;
            result.put_pixel(x, y, Rgb([r, g, b]));
        }

        result
    }

    pub fn save(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        self.img.save(path)?;
        Ok(())
    }
}

pub fn capture_screen(x: i32, y: i32, width: usize, height: usize) -> DynamicImage {
    let _start = std::time::Instant::now();
    let screen = Screen::all().unwrap()[0];

    let image = screen
        .capture_area(x, y, width as u32, height as u32)
        .unwrap();

    let buffer = image.to_png(Some(Compression::default())).unwrap();
    let cursor = std::io::Cursor::new(buffer);

    image::load(cursor, image::ImageFormat::Png).unwrap()
}

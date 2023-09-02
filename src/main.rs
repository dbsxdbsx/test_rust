mod image_multi;
use image_multi::{test_show_image, ImageType};
use rand::Rng;

fn main() {
    loop {
        let a = rand::thread_rng().gen_range(0..=8);
        test_show_image(
            Some(format!("title{}", a)),
            ImageType::Screenshot {
                x: a,
                y: a,
                w: 500,
                h: 600,
            },
        );
        // std::thread::sleep(ten_millis);
    }
}

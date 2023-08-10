mod image_multi;
use crate::image_multi::{test_show_image_3, ImageType};
mod tensor;

use rand::Rng;

fn main() {
    loop {
        let a = rand::thread_rng().gen_range(0..=100);
        // let num = rand::thread_rng().gen_range(0..100);
        test_show_image_3(
            Some(format!("title{}", a)),
            ImageType::Screenshot {
                x: a,
                y: a,
                w: 500,
                h: 500,
            },
        );
    }


    // test image
    // test_show_image_3(
    //     Some("标题1".to_string()),
    //     ImageType::Screenshot {
    //         x: 0,
    //         y: 0,
    //         w: 500,
    //         h: 500,
    //     },
    // );

    // let ten_millis = std::time::Duration::from_millis(1000);
    // std::thread::sleep(ten_millis);

    // test_show_image_3(
    //     Some("".to_string()),
    //     ImageType::Screenshot {
    //         x: 100,
    //         y: 100,
    //         w: 500,
    //         h: 500,
    //     },
    // );

    println!("======================");

    // test tensor
    // let tensor = Tensor::eye(1);
    // println!("{}", tensor);
    // let tensor = Tensor::eye(2);
    // println!("{}", tensor);
    // let tensor = Tensor::eye(3);
    // println!("{}", tensor);
    // let tensor = Tensor::eye(7);
    // println!("{}", tensor);
    // let tensor = Tensor::random(2, 2, 0.0, 1.0);
    // println!("{}", tensor);
    // let tensor = Tensor::random(7, 4, 0.0, 1.0);
    // println!("{}", tensor);
    // let tensor = Tensor::random(4, 7, 0.0, 1.0);
    // println!("{}", tensor);
    // let tensor = Tensor::random(1, 7, 0.0, 1.0);
    // println!("{}", tensor);
    // let tensor = Tensor::random(7, 1, 0.0, 1.0);
    // println!("{}", tensor);
    // let tensor = Tensor::random(6, 6, 0.0, 1.0);
    // println!("{}", tensor);
}

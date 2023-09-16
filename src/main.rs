mod image_multi;
use image_multi::{test_show_image, ImageType};
use rand::Rng;

use show_image::{create_window, ImageInfo, ImageView};

use crate::test_lazy::{
    expensive_computation_1, expensive_computation_2, get_obj1, test_multi_call, LazyObject,
};
mod test_lazy;

#[show_image::main]
fn main() {
    // let mut lazy_value = Lazy::new(move || expensive_computation_1());
    // let mut lazy_value = Lazy::new(move || expensive_computation_2(2, 3));

    test_multi_call();

    // let  v:Vec<i32> = vec![];
    // let _r = v.iter().all(|item| *item == 1);
    // loop {
    //     let a = rand::thread_rng().gen_range(0..=8);
    //     test_show_image(
    //         Some(format!("title{}", a)),
    //         ImageType::Screenshot {
    //             x: a,
    //             y: a,
    //             w: 500,
    //             h: 600,
    //         },
    //     );
    //     // std::thread::sleep(ten_millis);
    // }
}

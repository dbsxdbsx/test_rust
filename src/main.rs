mod image_multi;

use crate::test_clousure_on_instance::{CustomizeStruct, InputStruct};
mod tensor;
mod test_clousure_on_instance;

fn main() {
    let x = CustomizeStruct::new();
    let tensor = InputStruct::new(21, "示例对象");
    let y: i32 = x(&tensor);
    println!("y = {}", y);
}

// fn main() {
//     let first_shape = vec![3];
//     let t_shape = vec![3];
//     let r = t_shape
//         .iter()
//         .skip(1)
//         .zip(first_shape.iter().skip(1))
//         .all(|(a, b)| a == b);
//     // let ten_millis = std::time::Duration::from_millis(1000);
//     loop {
//         let a = rand::thread_rng().gen_range(0..=8);
//         test_show_image_3(
//             Some(format!("title{}", a)),
//             ImageType::Screenshot {
//                 x: a,
//                 y: a,
//                 w: 500,
//                 h: 500,
//                 // panic with different sizes
//                 // w: (a * 100) as usize,
//                 // h: (a * 100) as usize,
//             },
//         );
//         // std::thread::sleep(ten_millis);
//     }

//     // test tensor
//     // let tensor = Tensor::eye(1);
//     // println!("{}", tensor);
//     // let tensor = Tensor::eye(2);
//     // println!("{}", tensor);
//     // let tensor = Tensor::eye(3);
//     // println!("{}", tensor);
//     // let tensor = Tensor::eye(7);
//     // println!("{}", tensor);
//     // let tensor = Tensor::random(2, 2, 0.0, 1.0);
//     // println!("{}", tensor);
//     // let tensor = Tensor::random(7, 4, 0.0, 1.0);
//     // println!("{}", tensor);
//     // let tensor = Tensor::random(4, 7, 0.0, 1.0);
//     // println!("{}", tensor);
//     // let tensor = Tensor::random(1, 7, 0.0, 1.0);
//     // println!("{}", tensor);
//     // let tensor = Tensor::random(7, 1, 0.0, 1.0);
//     // println!("{}", tensor);
//     // let tensor = Tensor::random(6, 6, 0.0, 1.0);
//     // println!("{}", tensor);
// }

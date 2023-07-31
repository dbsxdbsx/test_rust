mod image;
use crate::image::{test_show_image, ImageType};
mod tensor;
use crate::tensor::Tensor;
fn main() {
    let tensor = Tensor::eye(1);
    println!("{}", tensor);
    let tensor = Tensor::eye(2);
    println!("{}", tensor);
    let tensor = Tensor::eye(3);
    println!("{}", tensor);
    let tensor = Tensor::eye(7);
    println!("{}", tensor);
    let tensor = Tensor::random(2, 2, 0.0, 1.0);
    println!("{}", tensor);
    let tensor = Tensor::random(7, 4, 0.0, 1.0);
    println!("{}", tensor);
    let tensor = Tensor::random(4, 7, 0.0, 1.0);
    println!("{}", tensor);
    let tensor = Tensor::random(1, 7, 0.0, 1.0);
    println!("{}", tensor);
    let tensor = Tensor::random(7, 1, 0.0, 1.0);
    println!("{}", tensor);
    let tensor = Tensor::random(6, 6, 0.0, 1.0);
    println!("{}", tensor);
}

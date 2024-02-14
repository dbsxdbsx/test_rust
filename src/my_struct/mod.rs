// use crate::my_trait::private::PrivateTrait;
// use crate::my_trait::private::PrivateTrait;
// use crate::my_trait::{private, PublicTrait};

// pub struct MyStruct;

// impl PublicTrait for MyStruct {
//     fn public_method(&self) {
//         println!("public metod");
//         self.private_method();
//     }
// }

// // 在其他模块中实现PublicTrait
// // 由于PrivateTrait是私有的，所以它的方法不会被外部代码直接调用
// impl private::PrivateTrait for MyStruct {
//     fn private_method(&self) {
//         println!("private method");
//     }
// }

use crate::my_trait::private::B;
pub use crate::my_trait::A;

pub struct MyType;

impl B for MyType {
    fn hidden(&self) {
        println!("This is a hidden method.");
    }
}

impl A for MyType {
    fn visible(&self) {
        println!("This is a visible method.");
    }
}

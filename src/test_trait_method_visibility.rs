// use crate::my_struct::MyStruct;
// use crate::my_trait::private::PrivateTrait;
// use crate::my_trait::PublicTrait;

// mod private {
//     pub trait B {
//         fn hidden(&self);
//     }
// }

// pub trait A: private::B {
//     fn visible(&self);
// }

// struct MyType;

// impl private::B for MyType {
//     fn hidden(&self) {
//         println!("This is a hidden method.");
//     }
// }

// impl A for MyType {
//     fn visible(&self) {
//         println!("This is a visible method.");
//     }
// }

use crate::my_struct::{MyType, A};
use crate::my_trait::private::B;

pub fn test() {
    let s = MyType {};
    s.visible();
    s.hidden();
    // let s = MyStruct {};
    // s.public_method();
    // s.private_method();
}

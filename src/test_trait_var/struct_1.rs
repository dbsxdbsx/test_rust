use super::{ExampleTrait, MyTrait};
use trait_variable::trait_var;

// #[trait_var(MyTrait)]
// struct ExampleStruct1 {
//     pub value: i32,
// }

// crate::MyTrait_for_struct! {
//     pub struct ExampleStruct1 {
//         pub value: i32,
//     }
// }

// impl ExampleTrait<i32> for ExampleStruct1 {
//     fn get_value(&self) -> i32 {
//         return self.value;
//     }
// }
// impl MyTrait for ExampleStruct1 {}

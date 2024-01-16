// use std::any::{Any, TypeId};

// pub trait CheckType {
//     fn check_type(&self) -> TypeId;
// }

// impl<T: Any> CheckType for T {
//     fn check_type(&self) -> TypeId {
//         TypeId::of::<T>()
//     }
// }

// pub struct EmptyStruct;

// pub trait EmptyTrait {}

// impl EmptyTrait for EmptyStruct {}

// impl EmptyStruct {
//     pub fn new() -> Self {
//         EmptyStruct
//     }

//     pub fn is_same_type(&mut self, result: &mut Box<dyn EmptyTrait>) -> bool {
//         self.check_type() == result.check_type()

//     }
// }

// pub fn test() {
//     let a = 5;
//     let b = "hello";

//     println!("{}", a.check_type() == TypeId::of::<i32>()); // 输出：true
//     println!("{}", b.check_type() == TypeId::of::<&str>()); // 输出：true

//     println!("{}", b.check_type() == TypeId::of::<i32>()); // 输出：false
//     println!("{}", a.check_type() == TypeId::of::<&str>()); // 输出：false

//     let mut empty_struct = EmptyStruct::new();
//     let mut empty_struct2 = EmptyStruct::new();
//     // println!("{}", empty_struct.is_same_type(&mut empty_struct2));
// }

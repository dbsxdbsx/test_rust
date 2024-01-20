// use paste::paste;

// use syn::{parse_macro_input, parse_quote, Expr, ExprMethodCall, ExprPath, ItemFn, Stmt, visit_mut::VisitMut};
// use quote::quote;
// use proc_macro2::TokenStream;

// macro_rules! replace_self {
//     ($($t:tt)*) => {
//         (|| {
//             macro_rules! self_replacement {
//                 ($name:ident) => {
//                     (|| $name )()
//                 };
//             }
//             $($t)*
//         })()
//     };
// }

// macro_rules! trait_macro {
//     ($trait_name:ident {
//         $(
//             let $var_name:ident : $var_type:ty;
//         )*
//         $(
//             // fn $fn_name:ident($($arg:tt)*) $(-> $ret_ty:ty)? {$(body:tt)*} // 不行，因不支持函数体内写宏
//             fn $fn_name:ident($($arg:tt)*) $(-> $ret_ty:ty)? $body:block
//         )*
//      }) => {
//         paste! {
//             trait [<_ $trait_name Parent>] {
//                 $(
//                     fn [< _ $var_name >](&self) -> &$var_type;
//                     fn [< _ $var_name _mut>](&mut self) -> &mut $var_type;
//                 )*
//             }
//             trait $trait_name: [<_ $trait_name Parent>] {
//                 $(
//                     // fn $fn_name($($arg)*) $(-> $ret_ty)? $body  // ok
//                     fn $fn_name($($arg)*) $(-> $ret_ty)? {
//                         replace_self!($body)
//                     }
//                 )*
//             }
//         }
//     };
// }

// trait_macro! {
//     MyTrait {
//         let x: i32;
//         let y :bool;
//         let z : String ;
//         fn with_body(&self) {
//             self.x=9 ;
//             // println!("orig: I am here in with number: {}", self._x());
//         }
//         // fn without_body();
//         // fn without_body2();
//     }
// }

// struct MyStruct {
//     x: i32,
//     y: bool,
//     z: String,
// }
// impl _MyTraitParent for MyStruct {
//     fn _x(&self) -> &i32 {
//         &self.x
//     }
//     fn _x_mut(&mut self) -> &mut i32 {
//         &mut self.x
//     }
//     fn _y(&self) -> &bool {
//         &self.y
//     }
//     fn _y_mut(&mut self) -> &mut bool {
//         &mut self.y
//     }
//     fn _z(&self) -> &String {
//         &self.z
//     }
//     fn _z_mut(&mut self) -> &mut String {
//         &mut self.z
//     }
// }

// impl MyTrait for MyStruct {
//     fn with_body(&self) {
//         println!("orig: I am here in with number: {}", self._x());
//     }
// }
// fn _x(){
// }
// pub fn test() {
//     replace_self! {
//         self.x;
//         // println!("{}", x);
//     }

//     // let s = MyStruct {
//     //     x: 1,
//     //     y: true,
//     //     z: "hello".to_string(),
//     // };
//     // s.with_body();
//     // println!("x: {}, y: {}, z: {}", s.x, s.y, s.z);
//     // MyStruct::without_body();
//     // MyStruct::without_body2();
// }

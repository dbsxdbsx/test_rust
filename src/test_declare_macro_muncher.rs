// macro_rules! trait_var {
//     (
//         trait $trait_name:ident {
//             $($input:tt)*
//         }
//     ) => (
//         my_helper! {
//             $trait_name
//             [fields: ]
//             [fns_impls: ]
//             [fns_no_impls: ]
//             $($input)*
//         }
//     );
// }

// macro_rules! my_helper {
//     // 1.match fields
//     (
//         $trait_name:ident
//         [fields: $($fields:tt)*]
//         [fns_impls: $($fns_impls:tt)*]
//         [fns_no_impls: $($fns_no_impls:tt)*]
//         $var_name:ident : $var_type:ty;
//         $($rest:tt)*
//     ) => (my_helper! {
//         $trait_name
//         [fields: $($fields)* $var_name $var_type]
//         [fns_impls: $($fns_impls)*]
//         [fns_no_impls: $($fns_no_impls)*]
//         $($rest)*
//     });
//     // 2.match fns(functions or methods) with default implementation
//     // (
//     //     [fields: $($fields:tt)*]
//     //     [fns_impls: $($fns_impl:tt)*]
//     //     [fns_no_impls: $($fns_no_impl:tt)*]
//     //     fn $fn_name:ident($($arg:tt)*) $(-> $ret_ty:ty)? { $($fn_body:tt)* }
//     //     $($rest:tt)*
//     // ) => (my_helper! {
//     //     [fields: $($fields)*]
//     //     [fns_impls: $($fns_impl)* $fn_name $($arg)* $($ret_ty)? $($fn_body)*]
//     //     [fns_no_impls: $($fns_no_impl)*]
//     //     $($rest)*
//     // });
//     // 3.match fns(functions or methods) with no default implementation
//     // (
//     //     [fields: $($fields:tt)*]
//     //     [fns_impls: $($fns_impl:tt)*]
//     //     [fns_no_impls: $($fns_no_impl:tt)*]
//     //     fn $fn_name_no_impl:ident($($arg_no_impl:tt)*) $(-> $ret_ty_no_impl:ty)? ;
//     //     $($rest:tt)*
//     // ) => (my_helper! {
//     //     [fields: $($fields)*]
//     //     [fns_impls: $($fns_impl)*]
//     //     [fns_no_impls: $($fns_no_impl)* $fn_name_no_impl $($arg_no_impl)* $($ret_ty_no_impl)?]
//     //     $($rest)*
//     // });
//     // 4.final output
//     (
//         $trait_name:ident
//         [fields: $($fields:tt)*]
//         [fns_impls: $($fns_impls:tt)*]
//         [fns_no_impls: $($fns_no_impls:tt)*]
//         // $trait_name:ident
//         // [fields: $( $var_name:ident : $var_type:ty )*]
//         // [fns_impls: $( $fn_name_impl:tt $(arg_impl:tt)* $(ret_ty_impl:tt)? $fn_body:tt )*]
//         // [fns_no_impls: $( $fn_name_no_impl:tt $(arg_no_impl:tt)* $(ret_ty_no_impl:tt)? )*]
//     ) => (
//         trait $trait_name{
//             // 4.1 generate `getter` and `setter` for each field
//             $(
//                 paste! {
//                     fn [< _ $var_name >](&self) -> &$var_type;
//                     fn [< _ $var_name _mut>](&mut self) -> &mut $var_type;
//                 }
//             )*
//             // 4.2 copy and paste for each function with default implementation
//             $(paste! {
//             fn $fn_name_impl($($arg_impl)*) {
//                 $($fn_body)*
//             }
//             // TODO: fn $fn_name_impl($($arg_impl)*) -> $ret_ty_impl {
//                 //     $($fn_body)*
//                 // }
//             })*
//             // 4.3 copy and paste for each function with no default implementation
//             $(paste! {
//                 fn $fn_name_no_impl($($arg_no_impl)*);
//                 // TODO: fn $fn_name_no_impl($($arg_no_impl)*) -> $ret_ty_no_impl;
//             })*
//         }
//     );
// }

// // --------------------------------------------

// trait_var! {
//     trait MyTrait {
//     // TODO: 有返回值的情况
//     // let the field definition feasible in trait
//     // the below code is formatted arbitrarily for testing purpose.
//           x: i32;
//          y :bool;
//          z : String ;

//         // 1.func with or without default impl
//         // fn trait_func_with_default_impl() {
//         //     println!("trait_func_with_default_impl");
//         // }
//         // // fn trait_func_with_no_default_impl();

//         // // 2.`&self` method with or without default impl,
//         // fn trait_method_with_default_impl( &self ) {
//         //     println!("trait_method_with_default_impl， the trait field x is `{}`", self.x);
//         // }
//         // // fn trait_method_mut_with_no_default_impl(&  self);

//         // // 3.`&mut self` method with or without default impl
//         // fn trait_method_mut_with_default_impl(&  mut self) {
//         //     println!("trait_method_mut_with_default_impl");
//         // }
//         // fn trait_method_with_no_default_impl(&mut  self);
//     }
// }

// // #[trait_variable(MyTrait)]
// struct MyStruct {
//     my_field: i32,
// }

// impl MyStruct {
//     fn self_method(&self) {
//         println!("self_method: my_field is {}", self.my_field);
//     }
// }

// impl MyTrait for MyStruct {}

// pub fn test() {
//     let my_struct = MyStruct { my_field: 10 };
//     my_struct.self_method();
//     // my_struct.trait_method_with_default_impl();
//     // my_struct.trait_print2();
// }

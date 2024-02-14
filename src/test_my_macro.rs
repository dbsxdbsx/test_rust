use trait_variable::{trait_var, trait_variable};

trait_variable! {
    pub trait MyTrait {  // feel free to add `pub` when needed
        // 1.put the variable fields definition at the top of the target trait before any function
        x: i32;
        pub y: bool;

        // 2.the order of the function definition doesn't matter
        fn print_x(&self) {
            println!("x: `{}`", self.x);
        }
        fn print_y(&self){
            println!("y: `{}`", self.y);
        }
    }
}
// // way1: use attribute macro for struct
// #[trait_var(MyTrait)]
// pub struct MyStruct {
//     a: i32,
// }
// // way2: use declarative macro for struct
// // MyTrait_for_struct! {
// //     (_MyTrait) // put this at the top of the struct
// //     pub struct MyStruct { // feel free to add `pub` when needed
// //         // feel free to add any fields as usual or leave it empty
// //         pub a: i32,
// //     }
// // }

// impl MyStruct {
//     pub fn print_a(&self) {
//         println!("a: `{}`", self.a);
//     }
//     pub fn test(&mut self) {
//         self.y = true;
//     }
// }
// impl MyTrait for MyStruct {
//     //     fn print_y(&self) {
//     //         println!("y: `{}`", self.y);
//     //     }
// }

// pub fn test() {
//     let s = MyStruct {
//         a: 2,
//         x: 3,
//         y: true,
//     };
//     let p = module_path!();
//     println!("module path: {}", p);
//     // s.print_a();
//     s.print_x();
//     s.print_y();
// }

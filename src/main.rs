use test_my_macro::{MyTrait, _MyTrait};
use trait_variable::trait_var;

// mod test_correct_macro;
// mod test_declare_macro_example;
// mod test_declare_macro_muncher;
// mod test_declare_macro_test;
mod test_enum_dispatch;
// mod test_matrix;
mod test_my_macro;
// mod test_proxy;
// mod test_trait_combination;
// mod test_type_check;
// pub use test_correct_macro::{TraitEnhance, TraitEnhanceType};
// use test_my_macro::MyStruct;

// way1: use attribute macro for struct
#[trait_var(MyTrait)]
pub struct MyStruct {
    a: i32,
}
// way2: use declarative macro for struct
// MyTrait_for_struct! {
//     (_MyTrait) // put this at the top of the struct
//     pub struct MyStruct { // feel free to add `pub` when needed
//         // feel free to add any fields as usual or leave it empty
//         pub a: i32,
//     }
// }

impl MyStruct {
    pub fn print_a(&self) {
        println!("a: `{}`", self.a);
    }
    pub fn test(&mut self) {
        self.y = true;
    }
}
impl MyTrait for MyStruct {
    //     fn print_y(&self) {
    //         println!("y: `{}`", self.y);
    //     }
}

#[tokio::main]
async fn main() {
    let mut s = MyStruct {
        a: 5,
        x: 3,
        y: true,
    };
    s.print_y();

    // test_type_check::test();
    // test_matrix::test();
    // test_proxy::test_proxy().await;
    // test_variadic_param::test();
    // test_trait_combination::test();
    // test_declare_macro_muncher::test();
    // test_declare_macro_example::test();
    // test_declare_macro_test::test();
    // test_enum_dispatch::test();

    // test_my_macro::test();
    // test_correct_macro::test();

    // test_trait_method_visibility::test();
}

mod struct_1;
use macro_magic::{export_tokens, export_tokens_no_emit};
// mod struct_2;
use trait_variable::{trait_var, trait_variable};

pub trait ExampleTrait<T: std::fmt::Debug>
where
    T: std::fmt::Debug + std::fmt::Display,
{
    fn get_value(&self) -> T;
    fn print_value(&self, value: T) {
        println!("{:?}", value);
    }
}

#[trait_var(MyTrait)]
struct ExampleStruct1 {
    pub value: i32,
}
impl MyTrait for ExampleStruct1 {}

// NOTE: if in the same file, make sure the trait is defined before(above) the struct
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

pub fn test() {
    println!(
        "{}",
        std::env::current_dir().unwrap().join(file!()).display()
    );

    crate::declare!(5);
    //     let e1 = ExampleStruct1 {
    //         value: 5,
    //         x: -1,
    //         y: true,
    //     };
    //     e1.print_value(5);
    //     e1.print_x();

    //     //     let e2 = ExampleStruct2 {
    //     //         value: 3.14,
    //     //         x: -1,
    //     //         y: true,
    //     //     };
    //     //     e2.print_value(3.14);
    //     //     e2.print_x();

    //     //     assert_eq!(e1.get_value(), 5);
    //     //     assert_eq!(e2.get_value(), 3.14);
}

// a simple declare macro
// #[export_tokens_no_emit]
#[doc(hidden)]
#[macro_export]
macro_rules! declare {
    ($value:expr) => {
        println!("{}", $value);
    };
}

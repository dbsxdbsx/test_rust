mod test_commander_runner;
mod test_duct;

// mod test_super_trait;
// mod test_trait_var;
// use test_trait_var::MyTrait;
// use trait_variable::trait_var;
// OK
// #[trait_var(MyTrait)]
// struct ExampleStruct1 {
//     pub value: i32,
// }

// impl MyTrait for ExampleStruct1 {}

// #[tokio::main]
/* async */
use std::io::{self, Write};
fn main() {
    // test_commander_runner::test();

    //
    test_duct::test();
}

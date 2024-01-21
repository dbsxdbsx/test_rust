use trait_variable::{trait_var, trait_variable};

#[trait_var]
struct MyStruct {}
impl MyStruct {
    fn my_function(&self) {
        println!("my_function");
    }
}

pub fn test() {
    let my_struct = MyStruct {};
    my_struct.my_function();
    my_struct.print();
}

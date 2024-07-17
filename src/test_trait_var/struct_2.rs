use trait_variable::trait_var;

use super::{ExampleTrait, MyTrait, _MyTrait};
#[trait_var(MyTrait)]
pub struct ExampleStruct2 {
    pub value: f32,
}

// 为ExampleStruct实现ExampleTrait
impl ExampleTrait<f32> for ExampleStruct2 {
    fn get_value(&self) -> f32 {
        return self.value;
    }
}
impl MyTrait for ExampleStruct2 {}

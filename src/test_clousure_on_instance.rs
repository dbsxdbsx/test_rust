use std::ops::Deref;

// https://www.perplexity.ai/search/c62da098-7b6f-4c24-b904-e016c8857b96?s=c
// https://github.com/rust-lang/rust/issues/29625
pub struct InputStruct {
    value: i32,
    name: String,
}

impl InputStruct {
    pub fn new(value: i32, name: &str) -> Self {
        InputStruct {
            value,
            name: name.to_string(),
        }
    }
}

pub struct CustomizeStruct {
    closure: Box<dyn Fn(&InputStruct) -> i32>,
}

impl CustomizeStruct {
    pub fn new() -> Self {
        CustomizeStruct {
            closure: Box::new(|tensor: &InputStruct| {
                println!("调用CustomizeStruct");
                println!("InputStruct名称: {}", tensor.name);
                tensor.value * 2
            }),
        }
    }
}

impl Deref for CustomizeStruct {
    type Target = dyn Fn(&InputStruct) -> i32;

    fn deref(&self) -> &Self::Target {
        &*self.closure
    }
}

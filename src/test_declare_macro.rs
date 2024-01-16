macro_rules! trait_var {
    (
        // 匹配 trait 关键字和 trait 名称
        trait $trait_name:ident {
            // 匹配变量定义
            $(
                let $var_name:ident : $var_type:ty;
            )*
            // 匹配函数定义，无论是否有默认实现
            $(fn $($fn_def:tt)*)*
        }
    ) => {
        // 生成 trait 定义
        trait $trait_name {
            // 为每个变量生成 getter 和 setter 方法
            $(
                paste! {
                    fn [< _ $var_name >](&self) -> &$var_type;
                    fn [< _ $var_name _mut>](&mut self) -> &mut $var_type;
                }
            )*
            // 使用 refine_fn_body 宏处理函数定义
            $(refine_fn_body!($($fn_def)*))*;
        }
    };
}

trait_var! {
    trait MyTrait {
    // TODO: 有返回值的情况
    // let the field definition feasible in trait
    // the below code is formatted arbitrarily for testing purpose.
        let  x: i32;
        let y :bool;
        let z : String ;

        // 1.func with or without default impl
        fn trait_func_with_default_impl() {
            println!("trait_func_with_default_impl");
        }
        fn trait_func_with_no_default_impl();

        // 2.`&self` method with or without default impl,
        fn trait_method_with_default_impl( &self ) {
            // println!("trait_method_with_default_impl， the trait field x is `{}`", self.x);
        }
        fn trait_method_mut_with_no_default_impl(&  self);

        // 3.`&mut self` method with or without default impl
        fn trait_method_mut_with_default_impl(&  mut self) {
            println!("trait_method_mut_with_default_impl");
        }
        fn trait_method_with_no_default_impl(&mut  self);
    }
}

// #[trait_variable(MyTrait)]
struct MyStruct {
    my_field: i32,
}

impl MyStruct {
    fn self_method(&self) {
        println!("self_method: my_field is {}", self.my_field);
    }
}

impl MyTrait for MyStruct {}

pub fn test() {
    let my_struct = MyStruct { my_field: 10 };
    my_struct.struct_method();
    my_struct.trait_method_with_default_impl();
    // my_struct.trait_print2();
}

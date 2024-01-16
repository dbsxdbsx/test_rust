macro_rules! trait_var {
    (
        // 匹配 trait 关键字和 trait 名称
        trait $trait_name:ident {
            // 匹配变量定义
            $(
                let $var_name:ident : $var_type:ty;
            )*
            // 匹配有默认实现的函数定义
            $(
                fn $fn_name_with_default:ident($($self_type:ident)? $(, $arg_name_def:ident : $arg_type_def:ty)*) {
                    $($fn_body:tt)*
                }
            )*
            // // 匹配没有默认实现的函数定义，注意这里检测到分号来区分
            // $(
            //     fn $fn_name_with_no_default:ident($($self_type_no_def:tt)? $(, $arg_name:ident : $arg_type:ty)*);
            // )*
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
            // 生成有默认实现的函数定义
            $(
                fn $fn_name_with_default($($self_type)? $(, $arg_name_def: $arg_type_def)*) {
                    $($fn_body)*
                }
            )*
            // // 生成没有默认实现的函数定义
            // $(
            //     fn $fn_name_with_no_default($($self_type_no_def)? $(, $arg_name: $arg_type)*) ;
            // )*
        }
    };
}
// --------------------------------------------

trait_var! {
trait MyTrait {
    let x: i32; // issue 1,
    let y :bool;
    let z : String ;
    // fn trait_func_with_no_default_impl();
    // fn trait_method_with_no_default_impl(&self);
    fn trait_method_with_default_impl(&self) {
        // some code
    }
    //     // self.x = 6; // issue 2
    //     println!("trait print2: x is {}", self.x);
    // }
}
}

// #[trait_variable(MyTrait)]
struct MyStruct {
    my_field: i32,
    x: i32,
}

impl MyStruct {
    fn self_print(&self) {
        println!("self_print: my_field is {}", self.my_field);
    }
}

impl MyTrait for MyStruct {
    fn trait_print(&self) {
        println!("trait print: my_field is {}", self.my_field);
    }

    fn _x(&self) -> &i32 {
        todo!()
    }

    fn _x_mut(&mut self) -> &mut i32 {
        todo!()
    }

    fn _y(&self) -> &bool {
        todo!()
    }

    fn _y_mut(&mut self) -> &mut bool {
        todo!()
    }

    fn _z(&self) -> &String {
        todo!()
    }

    fn _z_mut(&mut self) -> &mut String {
        todo!()
    }
}

pub fn test() {
    let my_struct = MyStruct { my_field: 10, x: 5 };
    my_struct.self_print();
    my_struct.trait_print();
    // my_struct.trait_print2();
}

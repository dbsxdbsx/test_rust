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
                fn $fn_name_with_impl:ident($($arg_with_impl:tt)*) $(-> $ret_ty_with_impl:ty)? {
                    $($fn_body:tt)*
                }
            )*
            // 匹配没有默认实现的函数定义（这里检测到分号来区分）
            $(
                fn $fn_name_with_no_impl:ident($($arg_with_no_impl:tt)*) $(-> $ret_ty_with_no_impl:ty)?;
            )*
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
                paste! {
                    fn $fn_name_with_impl($arg_with_impl) -> $ret_ty_with_impl {
                        $fn_body
                    }
                }
            )*
            // 生成没有默认实现的函数定义
            $(
                paste! {
                    fn $fn_name_with_no_impl($arg_with_no_impl) -> $ret_ty_with_no_imple;
                }
            )*

        
        }
    };
}

// --------------------------------------------
macro_rules! refine_fn_body {
    // 匹配空的函数体
    ( ; ) => {
        // 生成一个空的函数体
        ;
    };
    // 匹配带有具体内容的函数体
    ( { $( $body:tt )* } ) => {
        // 生成具体的函数体
        {
            $( $body )*
        }
    };
}
// --------------------------------------------
macro_rules! replace_self {
    ($self:ident . $var:ident) => {
        _ $var ()
    };
}
// --------------------------------------------

trait_var! {
trait MyTrait {
    // let the field definition feasible in trait
    // the below code is formatted arbitrarily for testing purpose.
    let  x: i32;
     let y :bool;
    let z : String ;

    // func with or without default impl
    d fn trait_func_with_default_impl() {
        println!("trait_func_with_default_impl");
    }
    // fn trait_func_with_no_default_impl();

    // `&self` method with or without default impl,
    fn trait_method_with_default_impl( &self ) {
        // println!("trait_method_with_default_impl， the trait field x is `{}`", self.x);
    }
    // fn trait_method_mut_with_no_default_impl(&  self);

    // `&mut self` method with or without default impl
    fn trait_method_mut_with_default_impl(&  mut self) {
        println!("trait_method_mut_with_default_impl");
    }
    // fn trait_method_with_no_default_impl(&mut  self);
}
}

// #[trait_variable(MyTrait)]
struct MyStruct {
    my_field: i32,
    x: i32,
}

impl MyStruct {
    fn struct_method(&self) {
        println!("struct_method: my_field is {}", self.my_field);
    }
}

impl MyTrait for MyStruct {
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
    my_struct.struct_method();
    my_struct.trait_method_with_default_impl();
    // my_struct.trait_print2();
}

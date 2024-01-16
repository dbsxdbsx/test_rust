    // 复制函数定义
            // $(
            //         paste! {
            //             fn $fn_name($($arg)*) $($body)*
            //         }
            // )*

            // $(
            //     paste! {
            //         // 使用 refine_fn_body 宏处理函数体
            //         fn $fn_name($($arg)*) refine_fn_body!($($fn_body)*)
            //     }
            // )*

            // $(
            //     paste! {
            //         fn $fn_name($($arg)*)$($body)*
            //     }
            //     // {
            //     //     // 尝试将函数定义解析为带有函数体的函数
            //     //     let result: Result<syn::ItemFn, _> = syn::parse_quote! {
            //     //         fn $fn_name($($arg)*)$($body)*
            //     //     };
            //     //     match result {
            //     //         // 如果解析成功，说明函数有默认实现
            //     //         Ok(_) => {
            //     //             paste! {
            //     //                 fn $fn_name($($arg)*)$($body)*
            //     //             }
            //     //         }
            //     //         // 如果解析失败，说明函数没有默认实现，生成一个空的函数体
            //     //         Err(_) => {
            //     //             paste! {
            //     //                 fn $fn_name($($arg)*);
            //     //             }
            //     //         }
            //     //     }
            //     // }
            // )*
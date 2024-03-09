use quote::ToTokens;
use regex::Regex;
use syn::{parse_file, Item, Type};

fn filter_type_content(raw_rust_file: &str, types_names: &[&str]) -> String {
    let syntax = parse_file(raw_rust_file).unwrap();
    let mut final_result = String::new();
    for type_name in types_names {
        let mut result = String::new();
        let target_type: Type = syn::parse_str(type_name).unwrap();

        for item in syntax.clone().items {
            match item {
                Item::Struct(item_struct) if item_struct.ident == type_name => {
                    result.push_str(&item_struct.into_token_stream().to_string());
                    result.push('\n');
                }
                Item::Enum(item_enum) if item_enum.ident == type_name => {
                    result.push_str(&item_enum.into_token_stream().to_string());
                    result.push('\n');
                }
                Item::Impl(item_impl)
                    if item_impl.self_ty.to_token_stream().to_string()
                        == target_type.to_token_stream().to_string() =>
                {
                    result.push_str(&item_impl.into_token_stream().to_string());
                    result.push('\n');
                }
                _ => {}
            }
        }
        final_result.push_str(&result);
    }

    final_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_type_content() {
        let raw_rust_file = r#"
            struct MyStruct {
                field1: i32,
                field2: String,
            }

            impl MyStruct {
                fn new(field1: i32, field2: String) -> Self {
                    Self { field1, field2 }
                }
            }

            struct OtherStruct {
                field: MyStruct,
            }

            impl MyStruct {
                // existing code here
            }

            fn fake_MyStruct_func(){
                // fake code here
                println!("fake code");
            }
            impl MyStruct {
                // new separated block here
                pub fn test(&mut self){
                    todo!()
                }
            }

            enum MyEnum {
                Variant1,
                Variant2,
            }
            impl MyEnum {
                fn is_variant1(&self) -> bool {
                    matches!(self, MyEnum::Variant1)
                }

                fn is_variant2(&self) -> bool {
                    matches!(self, MyEnum::Variant2)
                }
            }
            fn fake_MyEnum_func(){
                // fake code here
                println!("fake code");
            }
            impl MyEnum{
                pub fn test (& mut self) { todo ! () }
            }
        "#;

        let expected_output = r#"
            struct MyStruct {
                field1: i32,
                field2: String,
            }

            impl MyStruct {
                fn new(field1: i32, field2: String) -> Self {
                    Self { field1, field2 }
                }
            }
        "#;

        let actual_output = filter_type_content(raw_rust_file, &["Myenum", "MyStruct"]);
        // let actual_output = filter_type_content("MyStruct", raw_rust_file);
        println!("Actual output: {}", actual_output);
        assert_eq!(actual_output.trim(), expected_output.trim());
    }
}



/// 根据匹配模式调整`input`字符串中的内容，并根据`deref`参数决定是否进行解引用转换
///
/// 当遇到`&mut self.x`模式时，转换为`&mut self._x_mut()`，如果`deref`为`true`，则进一步转换为`&mut (*self._x_mut())`；
/// 当遇到`& self.x`模式时，转换为`& self._x()`，如果`deref`为`true`，则进一步转换为`& (*self._x())`；
/// 当遇到`self.x`模式时，转换为`self._x()`，如果`deref`为`true`，则进一步转换为`(*self._x())`。
/// 不会匹配已经是函数调用的`self.x()`形式。
///
/// # 参数
///
/// * `input` - 待处理的字符串
/// * `deref` - 是否进行解引用转换
///
/// # 返回值
///
/// 返回处理后的字符串
pub fn adjust_self_pattern(input: &str, deref: bool) -> String {
    let re = Regex::new(r"(&\s*mut\s+self\.)([a-zA-Z_]\w*)|(&\s*self\.)([a-zA-Z_]\w*)|(self\.)([a-zA-Z_]\w*)").unwrap();
    let mut result = String::new();
    let mut last_end = 0;
    for cap in re.captures_iter(input) {
        let match_start = cap.get(0).unwrap().start();
        let match_end = cap.get(0).unwrap().end();
        // 如果匹配后紧跟`(`，则不进行替换
        if input[match_end..].starts_with('(') {
            continue;
        }
        // 将上一个匹配结束到当前匹配开始之间的文本追加到结果中
        result.push_str(&input[last_end..match_start]);
        match (cap.get(1), cap.get(3), cap.get(5)) {
            (Some(_), _, _) => {
                // 匹配到 &mut self.x
                let name = &cap[2];
                let replacement = if deref {
                    format!("&mut (*self._{}_mut())", name)
                } else {
                    format!("&mut self._{}_mut()", name)
                };
                result.push_str(&replacement);
            },
            (_, Some(_), _) => {
                // 匹配到 & self.x
                let name = &cap[4];
                let replacement = if deref {
                    format!("&(*self._{}())", name)
                } else {
                    format!("&self._{}()", name)
                };
                result.push_str(&replacement);
            },
            (_, _, Some(_)) => {
                // 匹配到 self.x
                let name = &cap[6];
                let replacement = if deref {
                    format!("(*self._{}())", name)
                } else {
                    format!("self._{}()", name)
                };
                result.push_str(&replacement);
            },
            _ => unreachable!(),
        }
        last_end = match_end;
    }
    // 将最后一个匹配结束到字符串末尾之间的文本追加到结果中
    result.push_str(&input[last_end..]);
    result
}
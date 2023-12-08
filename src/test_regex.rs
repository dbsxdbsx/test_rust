use quote::ToTokens;
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

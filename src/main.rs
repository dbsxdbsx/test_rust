mod test_custom_type_loop;

struct MyStruct {
    rust_output_path: String,
}
impl MyStruct {
    pub fn get_complex(&self) -> Vec<usize> {
        println!("======================");
        vec![1, 2, 3, 4]
    }
}

fn test() {
    panic!("test");
}
fn main() {
    test();
    let op = Some(MyStruct {
        rust_output_path: "".to_string(),
    });
    let r = op.unwrap();

    r.get_complex().iter().for_each(|each| {
        println!("{:?}", each);
    });
}

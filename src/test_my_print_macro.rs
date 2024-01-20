macro_rules! print_var {
    ($var:ident) => {
        println!("这是 {}，值为：{:?}", stringify!($var), $var);
    };
}
pub fn test() {
    let my_var = 10;
    print_var!(my_var);
}

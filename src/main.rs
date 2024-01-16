mod test_matrix;
mod test_proxy;
mod test_trait_combination;
mod test_type_check;
mod test_declare_macro;
#[tokio::main]
async fn main() {
    // test_type_check::test();
    // test_matrix::test();
    // test_proxy::test_proxy().await;
    // test_variadic_param::test();
    // test_trait_combination::test();
    test_declare_macro::test();

}

mod test_correct_macro;
mod test_declare_macro_example;
mod test_declare_macro_muncher;
mod test_declare_macro_test;
mod test_enum_dispatch;
mod test_matrix;
mod test_my_macro;
mod test_proxy;
mod test_trait_combination;
mod test_type_check;
pub use test_correct_macro::{TraitEnhance, TraitEnhanceType};

#[tokio::main]
async fn main() {
    // test_type_check::test();
    // test_matrix::test();
    // test_proxy::test_proxy().await;
    // test_variadic_param::test();
    // test_trait_combination::test();
    // test_declare_macro_muncher::test();
    // test_declare_macro_example::test();
    // test_declare_macro_test::test();
    // test_enum_dispatch::test();
    // test_my_macro::test();
    test_correct_macro::test();
}

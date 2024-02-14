// // 私有模块中定义私有trait
// // 参考：https://jack.wrenn.fyi/blog/private-trait-methods/
// pub(crate) mod private {
//     #[doc(hidden)]
//     pub trait PrivateTrait {
//         fn private_method(&self);
//     }
// }

// // 公开trait，它继承自私有trait
// pub trait PublicTrait: private::PrivateTrait {
//     // 公开方法
//     fn public_method(&self);
// }



pub(crate) mod private {
    pub trait B {
        fn hidden(&self);
    }
}

pub trait A: private::B {
    fn visible(&self);
}

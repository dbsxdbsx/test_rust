pub struct Lazy<F, T>
where
    F: FnOnce() -> T,
{
    f: Option<F>,
    value: Option<T>,
}

impl<F, T> Lazy<F, T>
where
    F: FnOnce() -> T,
{
    pub fn new(f: F) -> Self {
        Lazy {
            f: f.into(),
            value: None,
        }
    }

    pub fn get_or_compute(&mut self) -> &T {
        if self.value.is_none() {
            let f = self.f.take().expect("闭包已被调用");
            self.value = Some(f());
        }
        self.value.as_ref().unwrap()
    }
}

pub fn expensive_computation() -> i32 {
    println!("执行昂贵的计算...");
    42
}
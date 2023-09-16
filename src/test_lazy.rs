use std::sync::{Mutex, Once};

#[derive(Debug)]
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
        Self {
            f: Some(f),
            value: None,
        }
    }

    pub fn get(&mut self) -> &T {
        if self.value.is_none() {
            let f = self.f.take().expect("闭包已被调用");
            self.value = Some(f());
        }
        self.value.as_ref().unwrap()
    }
}

pub fn expensive_computation_1() -> i32 {
    println!("执行昂贵的计算1...");
    42
}

pub fn expensive_computation_2(a: i32, b: i32) -> i32 {
    println!("执行昂贵的计算2...");
    a + b
}

/// 给用户调用的函数
pub fn get_obj1<F: FnOnce() -> i32 + 'static>(f: F) -> i32 {
    static mut OBJ1: Option<Lazy<Box<dyn FnOnce() -> i32>, i32>> = None;
    static INIT1: Once = Once::new();
    static LOCK1: Mutex<()> = Mutex::new(());

    unsafe {
        INIT1.call_once(|| {
            OBJ1 = Some(Lazy::new(Box::new(f)));
        });

        let _guard = LOCK1.lock().unwrap();
        OBJ1.as_mut().unwrap().get().clone()
    }
}

/// 给用户调用的函数
fn get_obj2<F: FnOnce(i32, i32) -> i32 + 'static>(f: F, a: i32, b: i32) -> i32 {
    static mut OBJ2: Option<Lazy<Box<dyn FnOnce() -> i32>, i32>> = None;
    static INIT2: Once = Once::new();
    static LOCK2: Mutex<()> = Mutex::new(());

    unsafe {
        INIT2.call_once(|| {
            let f = Box::new(move || f(a, b));
            OBJ2 = Some(Lazy::new(f));
        });

        let _guard = LOCK2.lock().unwrap();
        OBJ2.as_mut().unwrap().get().clone()
    }
}

/// 测试用例
pub fn test_multi_call() {
    for i in 0..3 {
        let value_1 = get_obj1(expensive_computation_1);
        let value_2 = get_obj2(expensive_computation_2, 2, 3);
        // println!("第{}次获取值,obj1={:?}", i, value_1);
        println!("第{}次获取值,obj1={:?},obj2={:?}", i, value_1, value_2);
    }
}

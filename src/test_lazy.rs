use std::sync::{Mutex, Once};

// the basic struct for lazy const object, replacing global variables
#[derive(Debug, Clone)]
pub struct LazyObject<F, T>
where
    F: FnOnce() -> T,
{
    f: Option<F>,
    value: Option<T>,
}

impl<F, T> LazyObject<F, T>
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
            let f = self.f.take().expect("Closure has been called");
            self.value = Some(f());
        }
        self.value.as_ref().unwrap()
    }
}

pub fn expensive_computation_1() -> i32 {
    println!("Performing expensive computation 1...");
    42
}

pub fn expensive_computation_2(a: i32, b: i32) -> i32 {
    println!("Performing expensive computation 2...");
    a + b
}

// Custom struct
#[derive(Debug, Clone)]
pub struct CustomStruct {
    value: usize,
    text: String,
}

pub fn expensive_computation_3(a: usize, b: String) -> CustomStruct {
    println!("Performing expensive computation 3...");
    CustomStruct { value: a, text: b }
}

/// Function for users to call
pub fn get_obj1<F: FnOnce() -> i32 + 'static>(f: F) -> i32 {
    static mut OBJ1: Option<LazyObject<Box<dyn FnOnce() -> i32>, i32>> = None;
    static INIT1: Once = Once::new();
    static LOCK1: Mutex<()> = Mutex::new(());

    unsafe {
        INIT1.call_once(|| {
            OBJ1 = Some(LazyObject::new(Box::new(f)));
        });

        let _guard = LOCK1.lock().unwrap();
        OBJ1.as_mut().unwrap().get().clone()
    }
}

/// Function for users to call
fn get_obj2<F: FnOnce(i32, i32) -> i32 + 'static>(f: F, a: i32, b: i32) -> i32 {
    static mut OBJ2: Option<LazyObject<Box<dyn FnOnce() -> i32>, i32>> = None;
    static INIT2: Once = Once::new();
    static LOCK2: Mutex<()> = Mutex::new(());

    unsafe {
        INIT2.call_once(|| {
            let f = Box::new(move || f(a, b));
            OBJ2 = Some(LazyObject::new(f));
        });

        let _guard = LOCK2.lock().unwrap();
        OBJ2.as_mut().unwrap().get().clone()
    }
}

/// Function for users to call
fn get_obj3<F: FnOnce(usize, String) -> CustomStruct + 'static>(
    f: F,
    a: usize,
    b: String,
) -> CustomStruct {
    static mut OBJ3: Option<LazyObject<Box<dyn FnOnce() -> CustomStruct>, CustomStruct>> = None;
    static INIT3: Once = Once::new();
    static LOCK3: Mutex<()> = Mutex::new(());

    unsafe {
        INIT3.call_once(|| {
            let f = Box::new(move || f(a, b.clone()));
            OBJ3 = Some(LazyObject::new(f));
        });

        let _guard = LOCK3.lock().unwrap();
        OBJ3.as_mut().unwrap().get().clone()
    }
}

/// Test case: the loop is used to mimic multi-times call on the same lazy object
pub fn test_multi_call() {
    for i in 0..3 {
        let value_1 = get_obj1(expensive_computation_1);
        let value_2 = get_obj2(expensive_computation_2, i, i + 1);
        let value_3 = get_obj3(
            expensive_computation_3,
            (i + 2) as usize,
            "example".to_string(),
        );
        println!(
            "{}: obj1={:?},obj2={:?},obj3={:?}",
            i, value_1, value_2, value_3
        );
    }
}

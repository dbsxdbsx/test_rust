use once_cell::sync::Lazy; // need once_cell crate
use std::cell::RefCell;
use std::sync::Mutex;

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
            let f = self.f.take().unwrap();
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
pub fn get_global_obj1<F: FnOnce() -> i32 + Send + 'static>(f: F) -> i32 {
    static OBJ1: Lazy<Mutex<RefCell<Option<LazyObject<Box<dyn FnOnce() -> i32 + Send>, i32>>>>> =
        Lazy::new(|| Mutex::new(RefCell::new(None)));

    let obj1 = OBJ1.lock().unwrap();
    let mut obj1_ref = obj1.borrow_mut();
    if obj1_ref.is_none() {
        *obj1_ref = Some(LazyObject::new(Box::new(f)));
    }

    *obj1_ref.as_mut().unwrap().get()
}

/// Function for users to call
pub fn get_global_obj2<F: FnOnce(i32, i32) -> i32 + Send + 'static>(f: F, a: i32, b: i32) -> i32 {
    static OBJ2: Lazy<Mutex<RefCell<Option<LazyObject<Box<dyn FnOnce() -> i32 + Send>, i32>>>>> =
        Lazy::new(|| Mutex::new(RefCell::new(None)));

    let obj2 = OBJ2.lock().unwrap();
    let mut obj2_ref = obj2.borrow_mut();
    if obj2_ref.is_none() {
        *obj2_ref = Some(LazyObject::new(Box::new(move || f(a, b))));
    }
    *obj2_ref.as_mut().unwrap().get()
}

/// Function for users to call
pub fn get_global_obj3<F: FnOnce(usize, String) -> CustomStruct + Send + 'static>(
    f: F,
    a: usize,
    b: String,
) -> CustomStruct {
    static OBJ3: Lazy<
        Mutex<RefCell<Option<LazyObject<Box<dyn FnOnce() -> CustomStruct + Send>, CustomStruct>>>>,
    > = Lazy::new(|| Mutex::new(RefCell::new(None)));

    let obj3 = OBJ3.lock().unwrap();
    let mut obj3_ref = obj3.borrow_mut();
    if obj3_ref.is_none() {
        *obj3_ref = Some(LazyObject::new(Box::new(move || f(a, b))));
    }
    obj3_ref.as_mut().unwrap().get().clone()
}

/// Test case: the loop is used to mimic multi-times call on the same lazy object
pub fn test_multi_call() {
    for i in 0..3 {
        let value_1 = get_global_obj1(expensive_computation_1);
        let value_2 = get_global_obj2(expensive_computation_2, i, i + 1);
        let value_3 = get_global_obj3(
            expensive_computation_3,
            (i + 2) as usize,
            "example".to_string(),
        );
        // println!("{}: obj1={:?}", i, value_1);
        println!(
            "{}: obj1={:?},obj2={:?},obj3={:?}",
            i, value_1, value_2, value_3
        );
    }
}

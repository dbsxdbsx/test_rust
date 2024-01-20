use enum_dispatch::enum_dispatch;

// 定义一个特性
#[enum_dispatch]
pub trait Animal {
    fn make_sound(&self) -> &'static str;
}

// 定义两个结构体
struct Dog;
struct Cat;

// 为这两个结构体实现 Animal 特性
impl Animal for Dog {
    fn make_sound(&self) -> &'static str {
        "汪汪"
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> &'static str {
        "喵喵"
    }
}

// 使用 enum_dispatch 宏定义一个枚举，该枚举包含 Dog 和 Cat
#[enum_dispatch(Animal)]
pub enum MyAnimals {
    Dog,
    Cat,
}

// 为枚举实现 Animal 特性

pub fn test() {
    // 初始化两个枚举实例
    let dog = MyAnimals::Dog(Dog);
    let cat = MyAnimals::Cat(Cat);

    // 调用相同的特性方法
    println!("{}", dog.make_sound()); // 输出 "汪汪"
    println!("{}", cat.make_sound()); // 输出 "喵喵"
}

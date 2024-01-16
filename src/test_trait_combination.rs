trait TraitA {
    fn a();
}

trait TraitB {
    fn b();
}

trait MyTrait: TraitA + TraitB {}

struct MyStruct;

impl TraitA for MyStruct {
    fn a() {
        // 实现方法a
    }
}

impl TraitB for MyStruct {
    fn b() {
        // 实现方法b
    }
}


pub fn test(){
    MyStruct::a();
    MyStruct::b();
}
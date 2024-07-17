use supertrait::*;

#[supertrait]
pub trait Fizz<T: Copy>: Copy + Sized {
    type Foo = Option<T>;
    type Bar;
    // pub  var:i32;

    const fn double_value(val: T) -> (T, T) {
        (val, val)
    }

    const fn triple_value(val: T) -> (T, T, T);

    fn double_self_plus(&self, plus: Self::Foo) -> (Self, Self, Self::Foo) {
        (*self, *self, plus)
    }

    const fn interleave<I>(&self, a: T, b: I) -> (I, Self::Foo, T);
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Buzz;

#[impl_supertrait]
impl<T: Copy> Fizz<T> for Buzz {
    type Bar = usize;

    const fn triple_value(val: T) -> (T, T, T) {
        (val, val, val)
    }

    const fn interleave<I>(&self, a: T, b: I) -> (I, Self::Foo, T) {
        (b, Some(a), a)
    }
}

#[test]
const fn test_buzz_const() {
    assert!(Buzz::triple_value(3).0 == 3);
    let buzz = Buzz {};
    match buzz.interleave('h', false).1 {
        Some(c) => assert!(c == 'h'),
        None => unreachable!(),
    }
}

#[test]
fn test_buzz_default_associated_types() {
    let buzz = Buzz {};
    assert_eq!(buzz.double_self_plus(Some(3)), (buzz, buzz, Some(3)))
}

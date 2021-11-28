use std::ops::{Div, Mul, Rem};

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Clone + Rem<Output = T> + Zero,
{
    while !b.is_zero() {
        let new_a = b.clone();
        b = a % b;
        a = new_a;
    }
    a
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Clone + Mul<Output = T> + Rem<Output = T> + Div<Output = T> + Zero,
{
    a.clone() * (b.clone() / gcd(a, b))
}

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}
pub trait One {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

macro_rules! impl_zero_one_primitive {
    (($zero:expr, $one:expr), $typ:ty) => {
        impl Zero for $typ {
            fn zero() -> Self { $zero }
            fn is_zero(&self) -> bool { *self == $zero }
        }
        impl One for $typ {
            fn one() -> Self { $one }
            fn is_one(&self) -> bool { *self == $one }
        }
    };
    (($zero:expr, $one:expr), $typ:ty, $($rest:ty),+) => {
        impl_zero_one_primitive!(($zero, $one), $typ);
        impl_zero_one_primitive!(($zero, $one), $($rest),+);
    };
}
impl_zero_one_primitive!(
    (0, 1),
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize
);
impl_zero_one_primitive!((0.0, 1.0), f32, f64);

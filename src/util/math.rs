use std::ops::{Div, Mul, Rem};

use num_traits::Zero;

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

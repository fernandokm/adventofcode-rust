use std::ops::{Add, Div, Mul, Neg, Sub};

use num_traits::{CheckedAdd, CheckedSub, One, Zero};

#[derive(Debug, Clone, Copy)]
pub enum Signed<T> {
    Positive(T),
    Negative(T),
}

impl<T> Signed<T> {
    #[must_use]
    pub fn new(value: T) -> Self {
        Self::Positive(value)
    }

    #[must_use]
    pub fn zero() -> Self
    where
        T: Zero,
    {
        // This function is equivalent to Zero::zero, but has no generic bounds
        Self::Positive(T::zero())
    }

    #[must_use]
    pub fn one() -> Self
    where
        T: One,
    {
        // This function is equivalent to One::one, but has no generic bounds
        Self::Positive(T::one())
    }

    #[must_use]
    pub fn inner_unsigned(&self) -> &T {
        match self {
            Signed::Positive(v) | Signed::Negative(v) => v,
        }
    }

    #[must_use]
    pub fn into_inner_unsigned(self) -> T {
        match self {
            Signed::Positive(v) | Signed::Negative(v) => v,
        }
    }

    pub fn unwrap(self) -> T
    where
        T: Neg<Output = T>,
    {
        match self {
            Signed::Positive(v) => v,
            Signed::Negative(v) => -v,
        }
    }

    #[must_use]
    pub fn abs_mut(&mut self) -> &mut T {
        match self {
            Signed::Positive(v) | Signed::Negative(v) => v,
        }
    }

    #[must_use]
    pub fn is_positive(&self) -> bool {
        matches!(self, Self::Positive(_))
    }
}

impl<T> From<T> for Signed<T> {
    fn from(inner: T) -> Self {
        Self::new(inner)
    }
}

impl<T: Zero> Zero for Signed<T>
where
    Signed<T>: Add<Output = Signed<T>>,
{
    fn zero() -> Self {
        Self::Positive(T::zero())
    }

    fn is_zero(&self) -> bool {
        T::is_zero(self.inner_unsigned())
    }

    fn set_zero(&mut self) {
        T::set_zero(self.abs_mut());
    }
}

// TODO: remove the PartialEq bound as soon as it's no longer required by
// num_traits
impl<T: One> One for Signed<T>
where
    T: PartialEq,
{
    fn one() -> Self {
        Self::Positive(T::one())
    }

    fn is_one(&self) -> bool {
        T::is_one(self.inner_unsigned())
    }

    fn set_one(&mut self) {
        T::set_one(self.abs_mut());
    }
}

impl<T> Neg for Signed<T> {
    type Output = Signed<T>;

    fn neg(self) -> Self::Output {
        match self {
            Signed::Positive(v) => Signed::Negative(v),
            Signed::Negative(v) => Signed::Positive(v),
        }
    }
}

impl<T, Output> Add for Signed<T>
where
    T: Add<Output = Output> + Sub<Output = Output> + Ord,
{
    type Output = Signed<Output>;

    fn add(self, rhs: Signed<T>) -> Self::Output {
        use Signed::{Negative, Positive};
        match (self, rhs) {
            (Positive(x), Positive(y)) => Positive(x + y),
            (Negative(x), Negative(y)) => Negative(x + y),
            (Positive(x), Negative(y)) => {
                if x >= y {
                    Positive(x - y)
                } else {
                    Negative(y - x)
                }
            }
            (Negative(x), Positive(y)) => {
                if x >= y {
                    Negative(x - y)
                } else {
                    Positive(y - x)
                }
            }
        }
    }
}

impl<T, Output> Sub<Signed<T>> for Signed<T>
where
    T: Add<Output = Output> + Sub<Output = Output> + Ord,
{
    type Output = Signed<Output>;

    fn sub(self, rhs: Signed<T>) -> Self::Output {
        self + (-rhs)
    }
}

impl<T> Mul<Signed<T>> for Signed<T>
where
    T: Mul,
{
    type Output = Signed<<T as Mul>::Output>;

    fn mul(self, rhs: Signed<T>) -> Self::Output {
        let is_positive = self.is_positive() == rhs.is_positive();
        let result = Signed::new(self.into_inner_unsigned() * rhs.into_inner_unsigned());
        if is_positive { result } else { -result }
    }
}

impl<T> Div<Signed<T>> for Signed<T>
where
    T: Div,
{
    type Output = Signed<<T as Div>::Output>;

    fn div(self, rhs: Signed<T>) -> Self::Output {
        let is_positive = self.is_positive() == rhs.is_positive();
        let result = Signed::new(self.into_inner_unsigned() / rhs.into_inner_unsigned());
        if is_positive { result } else { -result }
    }
}

#[allow(clippy::module_name_repetitions)]
pub trait SignedExt {
    #[must_use]
    fn checked_add_signed(&self, rhs: &Signed<Self>) -> Option<Self>
    where
        Self: Sized;
}

impl<T> SignedExt for T
where
    T: CheckedAdd + CheckedSub,
{
    #[must_use]
    fn checked_add_signed(&self, rhs: &Signed<Self>) -> Option<Self> {
        match rhs {
            Signed::Positive(val) => self.checked_add(val),
            Signed::Negative(val) => self.checked_sub(val),
        }
    }
}

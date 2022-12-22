use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_traits::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, SaturatingAdd, SaturatingSub, WrappingAdd,
    WrappingSub,
};

use super::signed::{Signed, SignedExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct P2<T>(pub T, pub T);

impl<T> P2<T> {
    pub fn norm_l2_squared(&self) -> T
    where
        T: Clone + Add<Output = T> + Mul<Output = T>,
    {
        self.0.clone() * self.0.clone() + self.1.clone() * self.1.clone()
    }

    #[must_use]
    pub fn checked_add_signed(&self, rhs: &P2<Signed<T>>) -> Option<P2<T>>
    where
        T: CheckedAdd + CheckedSub,
    {
        Some(P2(
            self.0.checked_add_signed(&rhs.0)?,
            self.1.checked_add_signed(&rhs.1)?,
        ))
    }

    #[must_use]
    pub fn as_tuple(&self) -> (&T, &T) {
        (&self.0, &self.1)
    }

    #[must_use]
    pub fn into_tuple(self) -> (T, T) {
        (self.0, self.1)
    }
}

impl<T> P2<Signed<T>> {
    pub fn unwrap_signed(self) -> P2<T>
    where
        T: Neg<Output = T>,
    {
        P2(
            match self.0 {
                Signed::Positive(v) => v,
                Signed::Negative(v) => -v,
            },
            match self.1 {
                Signed::Positive(v) => v,
                Signed::Negative(v) => -v,
            },
        )
    }
}

impl<T> From<(T, T)> for P2<T> {
    fn from((x, y): (T, T)) -> Self {
        P2(x, y)
    }
}

impl<T> From<P2<T>> for (T, T) {
    fn from(point: P2<T>) -> Self {
        point.into_tuple()
    }
}

macro_rules! impl_direction {
    (@P2<$mod:ident>; [] -> [$($body:tt)*]) => {
        P2($($body)*)
    };
    (@P2<$mod:ident>; [0, $($rest:tt)*] -> [$($body:tt)*]) => {
        impl_direction!(@P2<$mod>; [$($rest)*] -> [$($body)* $mod::zero(),])
    };
    (@P2<$mod:ident>; [1, $($rest:tt)*] -> [$($body:tt)*]) => {
        impl_direction!(@P2<$mod>; [$($rest)*] -> [$($body)* $mod::one(),])
    };
    (@P2<$mod:ident>; [-1, $($rest:tt)*] -> [$($body:tt)*]) => {
        impl_direction!(@P2<$mod>; [$($rest)*] -> [$($body)* -$mod::one(),])
    };
    (@ImplSingle; => ($($args:tt)*)) => {};
    (@ImplSingle; $name:ident => ($($args:tt)*)) => {
        paste::paste! {
            #[must_use]
            pub fn $name<T>() -> P2<T>
            where
                T: Zero + One + Neg<Output = T>,
            {
                impl_direction!(@P2<T>; [$($args)*,] -> [])
            }
            #[must_use]
            pub fn [<signed_ $name>]<T>() -> P2<Signed<T>>
            where
                T: Zero + One,
            {
                impl_direction!(@P2<Signed>; [$($args)*,] -> [])
            }
        }
    };
    ($($name:ident$(|$rest:ident)* => ($($args:tt)*)),+ $(,)?) => {
        $(
            impl_direction!(@ImplSingle; $name => ($($args)*));
            impl_direction!(@ImplSingle; $($rest)|* => ($($args)*));
        )+
    };
}

pub mod ij {
    use std::ops::Neg;

    use num_traits::{One, Zero};

    use super::P2;
    use crate::util::signed::Signed;

    impl_direction!(
        left | west => (0, -1),
        right | east => (0, 1),
        up | north => (-1, 0),
        down | south => (1, 0),
        right_turn => (0, -1),
        left_turn  => (0, 1),
    );
}

pub mod xy {
    use std::ops::Neg;

    use num_traits::{One, Zero};

    use super::P2;
    use crate::util::signed::Signed;

    impl_direction!(
        left | west => (-1, 0),
        right | east => (1, 0),
        up | north => (0, 1),
        down | south => (0, -1),
        right_turn => (0, -1),
        left_turn  => (0, 1),
    );
}

impl<T> Neg for P2<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl<Lhs, Rhs> Add<P2<Rhs>> for P2<Lhs>
where
    Lhs: Add<Rhs>,
{
    type Output = P2<<Lhs as Add<Rhs>>::Output>;
    fn add(self, rhs: P2<Rhs>) -> Self::Output {
        P2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<Lhs, Rhs> AddAssign<P2<Rhs>> for P2<Lhs>
where
    Lhs: AddAssign<Rhs>,
{
    fn add_assign(&mut self, rhs: P2<Rhs>) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T> CheckedAdd for P2<T>
where
    T: CheckedAdd,
{
    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        Some(P2(self.0.checked_add(&rhs.0)?, self.1.checked_add(&rhs.1)?))
    }
}

impl<T> SaturatingAdd for P2<T>
where
    T: SaturatingAdd,
{
    fn saturating_add(&self, rhs: &Self) -> Self {
        P2(self.0.saturating_add(&rhs.0), self.1.saturating_add(&rhs.1))
    }
}

impl<T> WrappingAdd for P2<T>
where
    T: WrappingAdd,
{
    fn wrapping_add(&self, rhs: &Self) -> Self {
        P2(self.0.wrapping_add(&rhs.0), self.1.wrapping_add(&rhs.1))
    }
}

impl<Lhs, Rhs> Sub<P2<Rhs>> for P2<Lhs>
where
    Lhs: Sub<Rhs>,
{
    type Output = P2<<Lhs as Sub<Rhs>>::Output>;
    fn sub(self, rhs: P2<Rhs>) -> Self::Output {
        P2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<Lhs, Rhs> SubAssign<P2<Rhs>> for P2<Lhs>
where
    Lhs: SubAssign<Rhs>,
{
    fn sub_assign(&mut self, rhs: P2<Rhs>) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl<T> CheckedSub for P2<T>
where
    T: CheckedSub,
{
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        Some(P2(self.0.checked_sub(&rhs.0)?, self.1.checked_sub(&rhs.1)?))
    }
}

impl<T> SaturatingSub for P2<T>
where
    T: SaturatingSub,
{
    fn saturating_sub(&self, rhs: &Self) -> Self {
        P2(self.0.saturating_sub(&rhs.0), self.1.saturating_sub(&rhs.1))
    }
}

impl<T> WrappingSub for P2<T>
where
    T: WrappingSub,
{
    fn wrapping_sub(&self, rhs: &Self) -> Self {
        P2(self.1.wrapping_sub(&rhs.0), self.0.wrapping_sub(&rhs.1))
    }
}

impl<T> Mul for P2<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = P2<T>;

    fn mul(self, rhs: P2<T>) -> Self::Output {
        P2(
            self.0.clone() * rhs.0.clone() - self.1.clone() * rhs.1.clone(),
            self.0.clone() * rhs.1.clone() + self.1 * rhs.0,
        )
    }
}

impl<T> MulAssign for P2<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    fn mul_assign(&mut self, rhs: P2<T>) {
        *self = self.clone() * rhs;
    }
}

impl<T> CheckedMul for P2<T>
where
    T: Clone + CheckedAdd + CheckedSub + CheckedMul,
{
    fn checked_mul(&self, rhs: &Self) -> Option<Self> {
        Some(P2(
            T::checked_sub(&self.0.checked_mul(&rhs.0)?, &self.1.checked_mul(&rhs.1)?)?,
            T::checked_add(&self.0.checked_mul(&rhs.1)?, &self.1.checked_mul(&rhs.0)?)?,
        ))
    }
}

impl<T> Div for P2<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    type Output = P2<T>;

    fn div(self, rhs: P2<T>) -> Self::Output {
        let denom = rhs.norm_l2_squared();
        P2(
            (self.0.clone() * rhs.0.clone() + self.1.clone() * rhs.1.clone()) / denom.clone(),
            (self.1.clone() * rhs.0.clone() - self.0 * rhs.1) / denom,
        )
    }
}

impl<T> DivAssign for P2<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    fn div_assign(&mut self, rhs: P2<T>) {
        *self = self.clone() / rhs;
    }
}

impl<T> CheckedDiv for P2<T>
where
    T: Clone
        + CheckedAdd<Output = T>
        + CheckedSub<Output = T>
        + CheckedMul<Output = T>
        + CheckedDiv<Output = T>,
{
    fn checked_div(&self, rhs: &Self) -> Option<Self> {
        let denom = rhs
            .0
            .checked_mul(&rhs.0)?
            .checked_add(&rhs.1.checked_mul(&rhs.1)?)?;
        Some(P2(
            T::checked_add(&self.0.checked_mul(&rhs.0)?, &self.1.checked_mul(&rhs.1)?)?
                .checked_div(&denom)?,
            T::checked_sub(&self.1.checked_mul(&rhs.0)?, &self.0.checked_mul(&rhs.1)?)?
                .checked_div(&denom)?,
        ))
    }
}
